use enigo::Button;
use enigo::Direction;
use enigo::Enigo;
use enigo::Key;
use enigo::Keyboard;
use enigo::Mouse;
use serde::{Deserialize, Serialize};
use winit::event::MouseButton;
use winit::{event::ElementState, keyboard::KeyCode};
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
    pub fn process_enigo(&self, dev: &mut Enigo) {
        match self {
            HidEvent::Key(scan_code) => dev.key(scan_code.to_enigo(), scan_code.dir).unwrap(),
            HidEvent::MouseDelta(x, y) => {
                dev.move_mouse(*x, *y, enigo::Coordinate::Rel).unwrap();
            }
            HidEvent::MouseButton(mouse_buttons) => {
                dev.button(mouse_buttons.button, mouse_buttons.dir).unwrap();
            }
            HidEvent::MouseScroll(scroll) => {
                dev.scroll(*scroll, enigo::Axis::Vertical).unwrap();
            }
        };
    }

    pub fn process_winput(&self) {
        match self {
            HidEvent::Key(scan_code) => winput::press(scan_code.to_winput()),
            HidEvent::MouseDelta(x, y) => {
                winput::Mouse::move_relative(*x, *y);
            }
            // HidEvent::MouseButton(mouse_buttons) => {
            //     dev.button(mouse_buttons.button, mouse_buttons.dir).unwrap();
            // }
            // HidEvent::MouseScroll(scroll) => {
            //     dev.scroll(*scroll, enigo::Axis::Vertical).unwrap();
            // }
            _ => {}
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScanCode {
    code: KeyCode,
    pub dir: Direction,
}

impl ScanCode {
    pub fn new(code: KeyCode, dir: ElementState) -> Self {
        Self {
            code,
            dir: match dir {
                ElementState::Pressed => Direction::Press,
                ElementState::Released => Direction::Release,
            },
        }
    }

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

    #[cfg(target_os = "windows")]
    pub fn to_enigo(&self) -> Key {
        match self.code {
            KeyCode::Digit0 => Key::Num0,
            KeyCode::Digit1 => Key::Num1,
            KeyCode::Digit2 => Key::Num2,
            KeyCode::Digit3 => Key::Num3,
            KeyCode::Digit4 => Key::Num4,
            KeyCode::Digit5 => Key::Num5,
            KeyCode::Digit6 => Key::Num6,
            KeyCode::Digit7 => Key::Num7,
            KeyCode::Digit8 => Key::Num8,
            KeyCode::Digit9 => Key::Num9,
            KeyCode::KeyA => Key::A,
            KeyCode::KeyB => Key::B,
            KeyCode::KeyC => Key::C,
            KeyCode::KeyD => Key::D,
            KeyCode::KeyE => Key::E,
            KeyCode::KeyF => Key::F,
            KeyCode::KeyG => Key::G,
            KeyCode::KeyH => Key::H,
            KeyCode::KeyI => Key::I,
            KeyCode::KeyJ => Key::J,
            KeyCode::KeyK => Key::K,
            KeyCode::KeyL => Key::L,
            KeyCode::KeyM => Key::M,
            KeyCode::KeyN => Key::N,
            KeyCode::KeyO => Key::O,
            KeyCode::KeyP => Key::P,
            KeyCode::KeyQ => Key::Q,
            KeyCode::KeyR => Key::R,
            KeyCode::KeyS => Key::S,
            KeyCode::KeyT => Key::T,
            KeyCode::KeyU => Key::U,
            KeyCode::KeyV => Key::V,
            KeyCode::KeyW => Key::W,
            KeyCode::KeyX => Key::X,
            KeyCode::KeyY => Key::Y,
            KeyCode::KeyZ => Key::Z,
            KeyCode::NumpadAdd => Key::Add,
            KeyCode::AltLeft | KeyCode::AltRight => Key::Alt,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::CapsLock => Key::CapsLock,
            KeyCode::ControlLeft | KeyCode::ControlRight => Key::Control,
            KeyCode::NumpadDecimal => Key::Decimal,
            KeyCode::Delete => Key::Delete,
            KeyCode::NumpadDivide => Key::Divide,
            KeyCode::ArrowDown => Key::DownArrow,
            KeyCode::End => Key::End,
            KeyCode::Escape => Key::Escape,
            KeyCode::F1 => Key::F1,
            KeyCode::F2 => Key::F2,
            KeyCode::F3 => Key::F3,
            KeyCode::F4 => Key::F4,
            KeyCode::F5 => Key::F5,
            KeyCode::F6 => Key::F6,
            KeyCode::F7 => Key::F7,
            KeyCode::F8 => Key::F8,
            KeyCode::F9 => Key::F9,
            KeyCode::F10 => Key::F10,
            KeyCode::F11 => Key::F11,
            KeyCode::F12 => Key::F12,
            KeyCode::F13 => Key::F13,
            KeyCode::F14 => Key::F14,
            KeyCode::F15 => Key::F15,
            KeyCode::F16 => Key::F16,
            KeyCode::F17 => Key::F17,
            KeyCode::F18 => Key::F18,
            KeyCode::F19 => Key::F19,
            KeyCode::F20 => Key::F20,
            KeyCode::F21 => Key::F21,
            KeyCode::F22 => Key::F22,
            KeyCode::F23 => Key::F23,
            KeyCode::F24 => Key::F24,
            KeyCode::Help => Key::Help,
            KeyCode::Home => Key::Home,
            KeyCode::Insert => Key::Insert,
            KeyCode::MediaTrackNext => Key::MediaNextTrack,
            KeyCode::MediaPlayPause => Key::MediaPlayPause,
            KeyCode::MediaTrackPrevious => Key::MediaPrevTrack,
            KeyCode::Meta => Key::Meta,
            KeyCode::NumpadMultiply => Key::Multiply,
            KeyCode::Numpad0 => Key::Numpad0,
            KeyCode::Numpad1 => Key::Numpad1,
            KeyCode::Numpad2 => Key::Numpad2,
            KeyCode::Numpad3 => Key::Numpad3,
            KeyCode::Numpad4 => Key::Numpad4,
            KeyCode::Numpad5 => Key::Numpad5,
            KeyCode::Numpad6 => Key::Numpad6,
            KeyCode::Numpad7 => Key::Numpad7,
            KeyCode::Numpad8 => Key::Numpad8,
            KeyCode::Numpad9 => Key::Numpad9,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::Pause => Key::Pause,
            KeyCode::PrintScreen => Key::PrintScr,
            KeyCode::Enter => Key::Return,
            KeyCode::ArrowRight => Key::RightArrow,
            KeyCode::ShiftLeft => Key::LShift,
            KeyCode::ShiftRight => Key::RShift,
            KeyCode::Space => Key::Space,
            KeyCode::NumpadSubtract => Key::Subtract,
            KeyCode::Tab => Key::Tab,
            KeyCode::ArrowUp => Key::UpArrow,
            KeyCode::AudioVolumeDown => Key::VolumeDown,
            KeyCode::AudioVolumeMute => Key::VolumeMute,
            KeyCode::AudioVolumeUp => Key::VolumeUp,
            KeyCode::SuperLeft | KeyCode::SuperRight => Key::Meta,
            KeyCode::ContextMenu => Key::Apps,
            KeyCode::BrowserBack => Key::BrowserBack,
            KeyCode::BrowserFavorites => Key::BrowserFavorites,
            KeyCode::BrowserForward => Key::BrowserForward,
            KeyCode::BrowserHome => Key::BrowserHome,
            KeyCode::BrowserRefresh => Key::BrowserRefresh,
            KeyCode::BrowserSearch => Key::BrowserSearch,
            KeyCode::BrowserStop => Key::BrowserStop,
            KeyCode::Convert => Key::Convert,
            KeyCode::KanaMode => Key::Kana,
            KeyCode::LaunchApp1 => Key::LaunchApp1,
            KeyCode::LaunchApp2 => Key::LaunchApp2,
            KeyCode::LaunchMail => Key::LaunchMail,
            KeyCode::MediaSelect => Key::LaunchMediaSelect,
            KeyCode::MediaStop => Key::MediaStop,
            KeyCode::NonConvert => Key::NonConvert,
            KeyCode::Sleep => Key::Sleep,
            KeyCode::Backquote => Key::OEM3,
            KeyCode::Backslash => Key::OEM5,
            KeyCode::BracketLeft => Key::OEM4,
            KeyCode::BracketRight => Key::OEM6,
            KeyCode::Comma => Key::OEMComma,
            KeyCode::Equal => Key::OEMPlus,
            KeyCode::Minus => Key::OEMMinus,
            KeyCode::Period => Key::OEMPeriod,
            KeyCode::Quote => Key::OEM7,
            KeyCode::Semicolon => Key::OEM1,
            KeyCode::Slash => Key::OEM2,
            KeyCode::ArrowLeft => Key::LeftArrow,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MouseButtons {
    button: Button,
    dir: Direction,
}

impl MouseButtons {
    pub fn from_winit(button: MouseButton, dir: ElementState) -> Self {
        let button = match button {
            MouseButton::Left => Button::Left,
            MouseButton::Right => Button::Right,
            MouseButton::Middle => Button::Middle,
            MouseButton::Back => Button::Back,
            MouseButton::Forward => Button::Forward,
            _ => Button::ScrollDown,
        };
        Self {
            button,
            dir: match dir {
                ElementState::Pressed => Direction::Press,
                ElementState::Released => Direction::Release,
            },
        }
    }
}
