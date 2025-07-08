use std::{
    ops::Deref,
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

use tokio::{runtime::Handle, sync::Mutex, time::sleep};
use winit::event::ElementState;

use hidapi::{DeviceInfo, HidApi, HidDevice};

use crate::codes::HidEvent;

pub trait Emulator {
    fn emulate_input(&self, hid_event: &HidEvent);
}

pub struct WinputEmulator;

impl WinputEmulator {
    pub fn new() -> Self {
        WinputEmulator {}
    }
}

#[cfg(target_os = "windows")]
impl Emulator for WinputEmulator {
    fn emulate_input(&self, hid_event: &HidEvent) {
        match hid_event {
            HidEvent::Key(scan_code) => {
                if let Some(code) = scan_code.to_winput() {
                    match scan_code.dir {
                        ElementState::Pressed => winput::press(code),
                        ElementState::Released => winput::release(code),
                    }
                }
            }

            HidEvent::MouseDelta(x, y) => {
                winput::Mouse::move_relative(*x, *y);
            }
            HidEvent::MouseButton(mouse_buttons) => match mouse_buttons.dir {
                ElementState::Pressed => winput::press(mouse_buttons.to_winput()),
                ElementState::Released => winput::release(mouse_buttons.to_winput()),
            },
            HidEvent::MouseScroll(scroll) => {
                winput::Mouse::scroll(*scroll as f32);
            }
        };
    }
}

#[repr(u8)]
enum HidType {
    Key = 0,
    Mouse = 1,
    MouseButtons = 2,
    MouseScroll = 3,
}
async fn get_device(
    vid: u16,
    pid: u16,
    i_num: i32,
    searching: Arc<AtomicBool>,
    dev: Arc<Mutex<Option<HidDevice>>>,
) {
    let mut api = HidApi::new().unwrap();
    loop {
        api.refresh_devices().unwrap();
        match api.device_list().find(|dev| {
            dev.vendor_id() == vid && dev.product_id() == pid && dev.interface_number() == i_num
        }) {
            Some(new_dev) => {
                if let Ok(open_dev) = new_dev.open_device(&api) {
                    let mut dev = dev.lock().await;
                    println!("connected to device");
                    *dev = Some(open_dev);
                    searching.store(false, std::sync::atomic::Ordering::Release);
                    break;
                } else {
                    sleep(Duration::from_secs(1)).await;
                }
            }
            None => sleep(Duration::from_secs(1)).await,
        }
    }
}

pub struct HidEmulator {
    dev: Arc<Mutex<Option<HidDevice>>>,
    searching: Arc<AtomicBool>,
    vid: u16,
    pid: u16,
    i_num: i32,
}

impl HidEmulator {
    fn write_spawn(&self, dev: &HidDevice, buf: &[u8]) {
        if dev.write(buf).is_err() {
            self.searching
                .store(true, std::sync::atomic::Ordering::Release);
            Handle::current().spawn(get_device(
                self.vid,
                self.pid,
                self.i_num,
                self.searching.clone(),
                self.dev.clone(),
            ));
        }
    }

    pub fn new(vid: u16, pid: u16, i_num: i32) -> Self {
        let emu = HidEmulator {
            dev: Arc::new(Mutex::new(None)),
            searching: Arc::new(AtomicBool::new(true)),
            vid,
            pid,
            i_num,
        };
        Handle::current().spawn(get_device(
            vid,
            pid,
            i_num,
            emu.searching.clone(),
            emu.dev.clone(),
        ));
        emu
    }
}

impl Emulator for HidEmulator {
    fn emulate_input(&self, hid_event: &HidEvent) {
        if self.searching.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }

        let dev = match self.dev.try_lock() {
            Ok(dev) => dev,
            Err(_) => return,
        };
        if let Some(dev) = dev.as_ref() {
            match hid_event {
                HidEvent::Key(scan_code) => {
                    if let Some(code) = scan_code.to_hid() {
                        let mut buf = [0u8, HidType::Key as u8, code as u8, 0];
                        buf[1] = HidType::Key as u8;
                        buf[2] = code as u8;
                        match scan_code.dir {
                            ElementState::Pressed => buf[3] = 1,
                            ElementState::Released => buf[3] = 0,
                        }
                        self.write_spawn(dev, &buf);
                    }
                }
                HidEvent::MouseDelta(x, y) => {
                    let buf = [0, HidType::Mouse as u8, *x as u8, *y as u8];
                    self.write_spawn(dev, &buf);
                }
                HidEvent::MouseButton(mouse_buttons) => {
                    let mut buf = [0u8; 4];
                    buf[1] = HidType::MouseButtons as u8;
                    buf[2] = mouse_buttons.to_hid() as u8;
                    match mouse_buttons.dir {
                        ElementState::Pressed => buf[3] = 1,
                        ElementState::Released => buf[3] = 0,
                    }
                    self.write_spawn(dev, &buf);
                }
                HidEvent::MouseScroll(offset) => {
                    let buf = [0u8, HidType::MouseScroll as u8, *offset as u8];
                    self.write_spawn(dev, &buf);
                }
            }
        }
    }
}
