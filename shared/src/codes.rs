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
            HidEvent::Key(scan_code) => match scan_code.dir {
                ElementState::Pressed => winput::press(scan_code.to_winput()),
                ElementState::Released => winput::release(scan_code.to_winput()),
            },

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
    pub fn to_winput(&self) -> Vk {
        match self.code {
            KeyCode::Digit0 => Vk::_0,
            KeyCode::Digit1 => Vk::_1,
            KeyCode::Digit2 => Vk::_2,
            KeyCode::Digit3 => Vk::_3,
            KeyCode::Digit4 => Vk::_4,
            KeyCode::Digit5 => Vk::_5,
            KeyCode::Digit6 => Vk::_6,
            KeyCode::Digit7 => Vk::_7,
            KeyCode::Digit8 => Vk::_8,
            KeyCode::Digit9 => Vk::_9,
            KeyCode::KeyA => Vk::A,
            KeyCode::KeyB => Vk::B,
            KeyCode::KeyC => Vk::C,
            KeyCode::KeyD => Vk::D,
            KeyCode::KeyE => Vk::E,
            KeyCode::KeyF => Vk::F,
            KeyCode::KeyG => Vk::G,
            KeyCode::KeyH => Vk::H,
            KeyCode::KeyI => Vk::I,
            KeyCode::KeyJ => Vk::J,
            KeyCode::KeyK => Vk::K,
            KeyCode::KeyL => Vk::L,
            KeyCode::KeyM => Vk::M,
            KeyCode::KeyN => Vk::N,
            KeyCode::KeyO => Vk::O,
            KeyCode::KeyP => Vk::P,
            KeyCode::KeyQ => Vk::Q,
            KeyCode::KeyR => Vk::R,
            KeyCode::KeyS => Vk::S,
            KeyCode::KeyT => Vk::T,
            KeyCode::KeyU => Vk::U,
            KeyCode::KeyV => Vk::V,
            KeyCode::KeyW => Vk::W,
            KeyCode::KeyX => Vk::X,
            KeyCode::KeyY => Vk::Y,
            KeyCode::KeyZ => Vk::Z,
            KeyCode::NumpadAdd => Vk::Add,
            KeyCode::AltLeft | KeyCode::AltRight => Vk::Alt,
            KeyCode::Backspace => Vk::Backspace,
            KeyCode::CapsLock => Vk::CapsLock,
            KeyCode::ControlLeft | KeyCode::ControlRight => Vk::Control,
            KeyCode::NumpadDecimal => Vk::Decimal,
            KeyCode::Delete => Vk::Delete,
            KeyCode::NumpadDivide => Vk::Divide,
            KeyCode::ArrowDown => Vk::DownArrow,
            KeyCode::End => Vk::End,
            KeyCode::Escape => Vk::Escape,
            KeyCode::F1 => Vk::F1,
            KeyCode::F2 => Vk::F2,
            KeyCode::F3 => Vk::F3,
            KeyCode::F4 => Vk::F4,
            KeyCode::F5 => Vk::F5,
            KeyCode::F6 => Vk::F6,
            KeyCode::F7 => Vk::F7,
            KeyCode::F8 => Vk::F8,
            KeyCode::F9 => Vk::F9,
            KeyCode::F10 => Vk::F10,
            KeyCode::F11 => Vk::F11,
            KeyCode::F12 => Vk::F12,
            KeyCode::F13 => Vk::F13,
            KeyCode::F14 => Vk::F14,
            KeyCode::F15 => Vk::F15,
            KeyCode::F16 => Vk::F16,
            KeyCode::F17 => Vk::F17,
            KeyCode::F18 => Vk::F18,
            KeyCode::F19 => Vk::F19,
            KeyCode::F20 => Vk::F20,
            KeyCode::F21 => Vk::F21,
            KeyCode::F22 => Vk::F22,
            KeyCode::F23 => Vk::F23,
            KeyCode::F24 => Vk::F24,
            KeyCode::Help => Vk::Help,
            KeyCode::Home => Vk::Home,
            KeyCode::Insert => Vk::Insert,
            KeyCode::MediaTrackNext => Vk::NextTrack,
            KeyCode::MediaPlayPause => Vk::MediaPlayPause,
            KeyCode::MediaTrackPrevious => Vk::PrevTrack,
            KeyCode::Meta => Vk::LeftWin,
            KeyCode::NumpadMultiply => Vk::Multiply,
            KeyCode::Numpad0 => Vk::Numpad0,
            KeyCode::Numpad1 => Vk::Numpad1,
            KeyCode::Numpad2 => Vk::Numpad2,
            KeyCode::Numpad3 => Vk::Numpad3,
            KeyCode::Numpad4 => Vk::Numpad4,
            KeyCode::Numpad5 => Vk::Numpad5,
            KeyCode::Numpad6 => Vk::Numpad6,
            KeyCode::Numpad7 => Vk::Numpad7,
            KeyCode::Numpad8 => Vk::Numpad8,
            KeyCode::Numpad9 => Vk::Numpad9,
            KeyCode::PageDown => Vk::PageDown,
            KeyCode::PageUp => Vk::PageUp,
            KeyCode::Pause => Vk::Pause,
            KeyCode::PrintScreen => Vk::PrintScreen,
            KeyCode::Enter => Vk::Enter,
            KeyCode::ArrowRight => Vk::RightArrow,
            KeyCode::ShiftLeft => Vk::LeftShift,
            KeyCode::ShiftRight => Vk::RightShift,
            KeyCode::Space => Vk::Space,
            KeyCode::NumpadSubtract => Vk::Subtract,
            KeyCode::Tab => Vk::Tab,
            KeyCode::ArrowUp => Vk::UpArrow,
            KeyCode::AudioVolumeDown => Vk::VolumeDown,
            KeyCode::AudioVolumeMute => Vk::VolumeMute,
            KeyCode::AudioVolumeUp => Vk::VolumeUp,
            KeyCode::SuperLeft => Vk::LeftWin,
            KeyCode::SuperRight => Vk::RightWin,
            KeyCode::ContextMenu => Vk::Apps,
            KeyCode::BrowserBack => Vk::BrowserBack,
            KeyCode::BrowserFavorites => Vk::BrowserFavorites,
            KeyCode::BrowserForward => Vk::BrowserForward,
            KeyCode::BrowserHome => Vk::BrowserHome,
            KeyCode::BrowserRefresh => Vk::BrowserRefresh,
            KeyCode::BrowserSearch => Vk::BrowserSearch,
            KeyCode::BrowserStop => Vk::BrowserStop,
            KeyCode::Convert => Vk::Convert,
            KeyCode::KanaMode => Vk::Kana,
            KeyCode::LaunchApp1 => Vk::StartApp1,
            KeyCode::LaunchApp2 => Vk::StartApp2,
            KeyCode::LaunchMail => Vk::StartMail,
            KeyCode::MediaSelect => Vk::SelectMedia,
            KeyCode::MediaStop => Vk::MediaStop,
            KeyCode::NonConvert => Vk::NonConvert,
            KeyCode::Sleep => Vk::Sleep,
            KeyCode::Backquote => Vk::Oem3,
            KeyCode::Backslash => Vk::Oem5,
            KeyCode::BracketLeft => Vk::Oem4,
            KeyCode::BracketRight => Vk::Oem6,
            KeyCode::Comma => Vk::Comma,
            KeyCode::Equal => Vk::Plus,
            KeyCode::Minus => Vk::Minus,
            KeyCode::Period => Vk::Period,
            KeyCode::Quote => Vk::Oem7,
            KeyCode::Semicolon => Vk::Oem1,
            KeyCode::Slash => Vk::Oem2,
            KeyCode::ArrowLeft => Vk::LeftArrow,
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
