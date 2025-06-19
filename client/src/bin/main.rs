use enigo::Key;
use shared::codes::{HidEvent, MouseButtons, ScanCode};
use softbuffer::{Context, Surface};
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::num::NonZeroU32;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

type ChannelData = HidEvent;

fn main() {
    let (tx, rx) = channel();
    let internet_handle = thread::spawn(move || {
        let addr = "192.168.10.3:8080";

        let mut stream = TcpStream::connect(addr).expect("Unable to connect to stream");
        stream.set_nodelay(true).unwrap();
        loop {
            let key: ChannelData = rx.recv().unwrap();
            let mesg = bincode::serialize(&key).unwrap();
            stream
                .write_all((mesg.len()).to_le_bytes().as_slice())
                .unwrap();
            stream.write_all(mesg.as_slice()).unwrap();
            println!("Code: {:?} | Sent: {:?}", key, mesg);
        }
    });

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::new(tx);
    event_loop.run_app(&mut app).unwrap();
}

// Your application state
struct App {
    // The window is optional because it is not created until the
    // `resumed` event is received.
    window: Option<std::rc::Rc<Window>>,
    context: Option<Context<std::rc::Rc<Window>>>,
    surface: Option<Surface<std::rc::Rc<Window>, std::rc::Rc<Window>>>,
    tx: Sender<ChannelData>,
}

impl App {
    fn new(tx: Sender<ChannelData>) -> Self {
        Self {
            window: None,
            context: None,
            surface: None,
            tx,
        }
    }
}

// The main application logic, implemented as a trait
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            println!("resumed: creating window");
            let window_attributes = Window::default_attributes().with_title("It works!");
            let window = std::rc::Rc::new(event_loop.create_window(window_attributes).unwrap());
            window
                .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                .unwrap();
            self.window = Some(window.clone());

            let context = Context::new(window.clone()).unwrap();
            let surface = Surface::new(&context, window.clone()).unwrap();

            self.context = Some(context);
            self.surface = Some(surface);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Unless you handle RedrawRequested, your window will not be redrawn.
        // if let Some(window) = &self.window {
        //     window.request_redraw();
        // }
    }

    // ------------------- Window-Specific Events -------------------

    /// This is the most common event to handle. It's sent for all window events.
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed! Stopping...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                println!("Redraw requested");
                if let Some(surface) = &mut self.surface {
                    let mut buffer = surface.buffer_mut().unwrap();

                    for pixel in buffer.iter_mut() {
                        *pixel = 0x003498DB; // A nice blue color
                    }

                    // Present the buffer to the screen
                    buffer.present().unwrap();
                }
            }
            WindowEvent::Resized(new_size) => {
                if let Some(surface) = &mut self.surface {
                    let (width, height) = (new_size.width, new_size.height);
                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();
                }
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let scanCode = HidEvent::Key(ScanCode::new(code, event.state));
                    self.tx.send(scanCode).unwrap();
                }
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                let code = HidEvent::MouseButton(MouseButtons::new(button, state));
                self.tx.send(code).unwrap();
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {
                let delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, i) => -i as i32,
                    winit::event::MouseScrollDelta::PixelDelta(physical_position) => 0,
                };
                self.tx.send(HidEvent::MouseScroll(delta)).unwrap();
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                self.tx
                    .send(HidEvent::MouseDelta(delta.0 as i32, delta.1 as i32))
                    .unwrap();
            }
            _ => {}
        }
    }
}
