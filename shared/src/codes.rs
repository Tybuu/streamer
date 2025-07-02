use serde::{Deserialize, Serialize};
use winit::event::MouseButton;
use winit::{event::ElementState, keyboard::KeyCode};
#[cfg(target_os = "windows")]
use winput::Vk;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HidEvent {
    Key(ScanCode),
    MouseDelta(i32, i32),
    MouseButton(MouseButtons),
    MouseScroll(i32),
}

impl HidEvent {
    #[cfg(target_os = "windows")]
    pub fn process_winput(&self) {
        match self {
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
            _ => {}
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScanCode {
    code: KeyCode,
    pub dir: ElementState,
}

impl ScanCode {
    pub fn new(code: KeyCode, dir: ElementState) -> Self {
        Self { code, dir }
    }

    #[cfg(target_os = "windows")]
    pub fn to_winput(&self) -> Option<Vk> {
        match self.code {
            KeyCode::Digit0 => Some(Vk::_0),
            KeyCode::Digit1 => Some(Vk::_1),
            KeyCode::Digit2 => Some(Vk::_2),
            KeyCode::Digit3 => Some(Vk::_3),
            KeyCode::Digit4 => Some(Vk::_4),
            KeyCode::Digit5 => Some(Vk::_5),
            KeyCode::Digit6 => Some(Vk::_6),
            KeyCode::Digit7 => Some(Vk::_7),
            KeyCode::Digit8 => Some(Vk::_8),
            KeyCode::Digit9 => Some(Vk::_9),
            KeyCode::KeyA => Some(Vk::A),
            KeyCode::KeyB => Some(Vk::B),
            KeyCode::KeyC => Some(Vk::C),
            KeyCode::KeyD => Some(Vk::D),
            KeyCode::KeyE => Some(Vk::E),
            KeyCode::KeyF => Some(Vk::F),
            KeyCode::KeyG => Some(Vk::G),
            KeyCode::KeyH => Some(Vk::H),
            KeyCode::KeyI => Some(Vk::I),
            KeyCode::KeyJ => Some(Vk::J),
            KeyCode::KeyK => Some(Vk::K),
            KeyCode::KeyL => Some(Vk::L),
            KeyCode::KeyM => Some(Vk::M),
            KeyCode::KeyN => Some(Vk::N),
            KeyCode::KeyO => Some(Vk::O),
            KeyCode::KeyP => Some(Vk::P),
            KeyCode::KeyQ => Some(Vk::Q),
            KeyCode::KeyR => Some(Vk::R),
            KeyCode::KeyS => Some(Vk::S),
            KeyCode::KeyT => Some(Vk::T),
            KeyCode::KeyU => Some(Vk::U),
            KeyCode::KeyV => Some(Vk::V),
            KeyCode::KeyW => Some(Vk::W),
            KeyCode::KeyX => Some(Vk::X),
            KeyCode::KeyY => Some(Vk::Y),
            KeyCode::KeyZ => Some(Vk::Z),
            KeyCode::NumpadAdd => Some(Vk::Add),
            KeyCode::AltLeft => Some(Vk::Alt),
            KeyCode::AltRight => Some(Vk::LeftWin),
            KeyCode::Backspace => Some(Vk::Backspace),
            KeyCode::CapsLock => Some(Vk::CapsLock),
            KeyCode::ControlLeft | KeyCode::ControlRight => Some(Vk::Control),
            KeyCode::NumpadDecimal => Some(Vk::Decimal),
            KeyCode::Delete => Some(Vk::Delete),
            KeyCode::NumpadDivide => Some(Vk::Divide),
            KeyCode::ArrowDown => Some(Vk::DownArrow),
            KeyCode::End => Some(Vk::End),
            KeyCode::Escape => Some(Vk::Escape),
            KeyCode::F1 => Some(Vk::F1),
            KeyCode::F2 => Some(Vk::F2),
            KeyCode::F3 => Some(Vk::F3),
            KeyCode::F4 => Some(Vk::F4),
            KeyCode::F5 => Some(Vk::F5),
            KeyCode::F6 => Some(Vk::F6),
            KeyCode::F7 => Some(Vk::F7),
            KeyCode::F8 => Some(Vk::F8),
            KeyCode::F9 => Some(Vk::F9),
            KeyCode::F10 => Some(Vk::F10),
            KeyCode::F11 => Some(Vk::F11),
            KeyCode::F12 => Some(Vk::F12),
            KeyCode::F13 => Some(Vk::F13),
            KeyCode::F14 => Some(Vk::F14),
            KeyCode::F15 => Some(Vk::F15),
            KeyCode::F16 => Some(Vk::F16),
            KeyCode::F17 => Some(Vk::F17),
            KeyCode::F18 => Some(Vk::F18),
            KeyCode::F19 => Some(Vk::F19),
            KeyCode::F20 => Some(Vk::F20),
            KeyCode::F21 => Some(Vk::F21),
            KeyCode::F22 => Some(Vk::F22),
            KeyCode::F23 => Some(Vk::F23),
            KeyCode::F24 => Some(Vk::F24),
            KeyCode::Help => Some(Vk::Help),
            KeyCode::Home => Some(Vk::Home),
            KeyCode::Insert => Some(Vk::Insert),
            KeyCode::MediaTrackNext => Some(Vk::NextTrack),
            KeyCode::MediaPlayPause => Some(Vk::MediaPlayPause),
            KeyCode::MediaTrackPrevious => Some(Vk::PrevTrack),
            KeyCode::Meta => Some(Vk::LeftWin),
            KeyCode::NumpadMultiply => Some(Vk::Multiply),
            KeyCode::Numpad0 => Some(Vk::Numpad0),
            KeyCode::Numpad1 => Some(Vk::Numpad1),
            KeyCode::Numpad2 => Some(Vk::Numpad2),
            KeyCode::Numpad3 => Some(Vk::Numpad3),
            KeyCode::Numpad4 => Some(Vk::Numpad4),
            KeyCode::Numpad5 => Some(Vk::Numpad5),
            KeyCode::Numpad6 => Some(Vk::Numpad6),
            KeyCode::Numpad7 => Some(Vk::Numpad7),
            KeyCode::Numpad8 => Some(Vk::Numpad8),
            KeyCode::Numpad9 => Some(Vk::Numpad9),
            KeyCode::PageDown => Some(Vk::PageDown),
            KeyCode::PageUp => Some(Vk::PageUp),
            KeyCode::Pause => Some(Vk::Pause),
            KeyCode::PrintScreen => Some(Vk::PrintScreen),
            KeyCode::Enter => Some(Vk::Enter),
            KeyCode::ArrowRight => Some(Vk::RightArrow),
            KeyCode::ShiftLeft => Some(Vk::LeftShift),
            KeyCode::ShiftRight => Some(Vk::RightShift),
            KeyCode::Space => Some(Vk::Space),
            KeyCode::NumpadSubtract => Some(Vk::Subtract),
            KeyCode::Tab => Some(Vk::Tab),
            KeyCode::ArrowUp => Some(Vk::UpArrow),
            KeyCode::AudioVolumeDown => Some(Vk::VolumeDown),
            KeyCode::AudioVolumeMute => Some(Vk::VolumeMute),
            KeyCode::AudioVolumeUp => Some(Vk::VolumeUp),
            KeyCode::SuperLeft => None,
            KeyCode::SuperRight => None,
            KeyCode::ContextMenu => Some(Vk::Apps),
            KeyCode::BrowserBack => Some(Vk::BrowserBack),
            KeyCode::BrowserFavorites => Some(Vk::BrowserFavorites),
            KeyCode::BrowserForward => Some(Vk::BrowserForward),
            KeyCode::BrowserHome => Some(Vk::BrowserHome),
            KeyCode::BrowserRefresh => Some(Vk::BrowserRefresh),
            KeyCode::BrowserSearch => Some(Vk::BrowserSearch),
            KeyCode::BrowserStop => Some(Vk::BrowserStop),
            KeyCode::Convert => Some(Vk::Convert),
            KeyCode::KanaMode => Some(Vk::Kana),
            KeyCode::LaunchApp1 => Some(Vk::StartApp1),
            KeyCode::LaunchApp2 => Some(Vk::StartApp2),
            KeyCode::LaunchMail => Some(Vk::StartMail),
            KeyCode::MediaSelect => Some(Vk::SelectMedia),
            KeyCode::MediaStop => Some(Vk::MediaStop),
            KeyCode::NonConvert => Some(Vk::NonConvert),
            KeyCode::Sleep => Some(Vk::Sleep),
            KeyCode::Backquote => Some(Vk::Oem3),
            KeyCode::Backslash => Some(Vk::Oem5),
            KeyCode::BracketLeft => Some(Vk::Oem4),
            KeyCode::BracketRight => Some(Vk::Oem6),
            KeyCode::Comma => Some(Vk::Comma),
            KeyCode::Equal => Some(Vk::Plus),
            KeyCode::Minus => Some(Vk::Minus),
            KeyCode::Period => Some(Vk::Period),
            KeyCode::Quote => Some(Vk::Oem7),
            KeyCode::Semicolon => Some(Vk::Oem1),
            KeyCode::Slash => Some(Vk::Oem2),
            KeyCode::ArrowLeft => Some(Vk::LeftArrow),
            KeyCode::NumLock => Some(Vk::Numlock),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MouseButtons {
    button: MouseButton,
    dir: ElementState,
}

impl MouseButtons {
    pub fn new(button: MouseButton, dir: ElementState) -> Self {
        Self { button, dir }
    }

    #[cfg(target_os = "windows")]
    pub fn to_winput(&self) -> winput::Button {
        use winput::Button;
        let button = match self.button {
            MouseButton::Left => Button::Left,
            MouseButton::Right => Button::Right,
            MouseButton::Middle => Button::Middle,
            MouseButton::Back => Button::X1,
            MouseButton::Forward => Button::X2,
            _ => Button::Left,
        };
        button
    }
}
