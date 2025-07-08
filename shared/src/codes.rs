use serde::{Deserialize, Serialize};
use winit::event::MouseButton;
use winit::{event::ElementState, keyboard::KeyCode};
#[cfg(target_os = "windows")]
use winput::Vk;

use crate::scan_codes::HidCodes;

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

    pub fn to_hid(&self) -> Option<HidCodes> {
        match self.code {
            KeyCode::Backquote => Some(HidCodes::KeyboardBacktickTilde),
            KeyCode::Backslash => Some(HidCodes::KeyboardBackslashBar),
            KeyCode::BracketLeft => Some(HidCodes::KeyboardOpenBracketBrace),
            KeyCode::BracketRight => Some(HidCodes::KeyboardCloseBracketBrace),
            KeyCode::Comma => Some(HidCodes::KeyboardCommaLess),
            KeyCode::Digit0 => Some(HidCodes::Keyboard0CloseParens),
            KeyCode::Digit1 => Some(HidCodes::Keyboard1Exclamation),
            KeyCode::Digit2 => Some(HidCodes::Keyboard2At),
            KeyCode::Digit3 => Some(HidCodes::Keyboard3Hash),
            KeyCode::Digit4 => Some(HidCodes::Keyboard4Dollar),
            KeyCode::Digit5 => Some(HidCodes::Keyboard5Percent),
            KeyCode::Digit6 => Some(HidCodes::Keyboard6Caret),
            KeyCode::Digit7 => Some(HidCodes::Keyboard7Ampersand),
            KeyCode::Digit8 => Some(HidCodes::Keyboard8Asterisk),
            KeyCode::Digit9 => Some(HidCodes::Keyboard9OpenParens),
            KeyCode::Equal => Some(HidCodes::KeyboardEqualPlus),
            KeyCode::KeyA => Some(HidCodes::KeyboardAa),
            KeyCode::KeyB => Some(HidCodes::KeyboardBb),
            KeyCode::KeyC => Some(HidCodes::KeyboardCc),
            KeyCode::KeyD => Some(HidCodes::KeyboardDd),
            KeyCode::KeyE => Some(HidCodes::KeyboardEe),
            KeyCode::KeyF => Some(HidCodes::KeyboardFf),
            KeyCode::KeyG => Some(HidCodes::KeyboardGg),
            KeyCode::KeyH => Some(HidCodes::KeyboardHh),
            KeyCode::KeyI => Some(HidCodes::KeyboardIi),
            KeyCode::KeyJ => Some(HidCodes::KeyboardJj),
            KeyCode::KeyK => Some(HidCodes::KeyboardKk),
            KeyCode::KeyL => Some(HidCodes::KeyboardLl),
            KeyCode::KeyM => Some(HidCodes::KeyboardMm),
            KeyCode::KeyN => Some(HidCodes::KeyboardNn),
            KeyCode::KeyO => Some(HidCodes::KeyboardOo),
            KeyCode::KeyP => Some(HidCodes::KeyboardPp),
            KeyCode::KeyQ => Some(HidCodes::KeyboardQq),
            KeyCode::KeyR => Some(HidCodes::KeyboardRr),
            KeyCode::KeyS => Some(HidCodes::KeyboardSs),
            KeyCode::KeyT => Some(HidCodes::KeyboardTt),
            KeyCode::KeyU => Some(HidCodes::KeyboardUu),
            KeyCode::KeyV => Some(HidCodes::KeyboardVv),
            KeyCode::KeyW => Some(HidCodes::KeyboardWw),
            KeyCode::KeyX => Some(HidCodes::KeyboardXx),
            KeyCode::KeyY => Some(HidCodes::KeyboardYy),
            KeyCode::KeyZ => Some(HidCodes::KeyboardZz),
            KeyCode::Minus => Some(HidCodes::KeyboardDashUnderscore),
            KeyCode::Period => Some(HidCodes::KeyboardPeriodGreater),
            KeyCode::Quote => Some(HidCodes::KeyboardSingleDoubleQuote),
            KeyCode::Semicolon => Some(HidCodes::KeyboardSemiColon),
            KeyCode::Slash => Some(HidCodes::KeyboardSlashQuestion),
            KeyCode::AltLeft => Some(HidCodes::KeyboardLeftAlt),
            KeyCode::AltRight => Some(HidCodes::KeyboardLeftGUI),
            KeyCode::Backspace => Some(HidCodes::KeyboardBackspace),
            KeyCode::CapsLock => Some(HidCodes::KeyboardCapsLock),
            KeyCode::ControlLeft => Some(HidCodes::KeyboardLeftControl),
            KeyCode::ControlRight => Some(HidCodes::KeyboardRightControl),
            KeyCode::Enter => Some(HidCodes::KeyboardEnter),
            KeyCode::SuperLeft => Some(HidCodes::KeyboardLeftGUI),
            KeyCode::SuperRight => Some(HidCodes::KeyboardRightGUI),
            KeyCode::ShiftLeft => Some(HidCodes::KeyboardLeftShift),
            KeyCode::ShiftRight => Some(HidCodes::KeyboardRightShift),
            KeyCode::Space => Some(HidCodes::KeyboardSpacebar),
            KeyCode::Tab => Some(HidCodes::KeyboardTab),
            KeyCode::Delete => Some(HidCodes::KeyboardDelete),
            KeyCode::End => Some(HidCodes::KeyboardEnd),
            KeyCode::Home => Some(HidCodes::KeyboardHome),
            KeyCode::Insert => Some(HidCodes::KeyboardInsert),
            KeyCode::PageDown => Some(HidCodes::KeyboardPageDown),
            KeyCode::PageUp => Some(HidCodes::KeyboardPageUp),
            KeyCode::ArrowDown => Some(HidCodes::KeyboardDownArrow),
            KeyCode::ArrowLeft => Some(HidCodes::KeyboardLeftArrow),
            KeyCode::ArrowRight => Some(HidCodes::KeyboardRightArrow),
            KeyCode::ArrowUp => Some(HidCodes::KeyboardUpArrow),
            KeyCode::NumLock => Some(HidCodes::KeypadNumLock),
            KeyCode::Numpad0 => Some(HidCodes::Keypad0Insert),
            KeyCode::Numpad1 => Some(HidCodes::Keypad1End),
            KeyCode::Numpad2 => Some(HidCodes::Keypad2DownArrow),
            KeyCode::Numpad3 => Some(HidCodes::Keypad3PageDown),
            KeyCode::Numpad4 => Some(HidCodes::Keypad4LeftArrow),
            KeyCode::Numpad5 => Some(HidCodes::Keypad5),
            KeyCode::Numpad6 => Some(HidCodes::Keypad6RightArrow),
            KeyCode::Numpad7 => Some(HidCodes::Keypad7Home),
            KeyCode::Numpad8 => Some(HidCodes::Keypad8UpArrow),
            KeyCode::Numpad9 => Some(HidCodes::Keypad9PageUp),
            KeyCode::NumpadAdd => Some(HidCodes::KeypadMemoryAdd),
            KeyCode::NumpadBackspace => Some(HidCodes::KeypadBackspace),
            KeyCode::NumpadClear => Some(HidCodes::KeypadClear),
            KeyCode::NumpadClearEntry => Some(HidCodes::KeypadClearEntry),
            KeyCode::NumpadComma => Some(HidCodes::KeypadComma),
            KeyCode::NumpadDecimal => todo!(),
            KeyCode::NumpadDivide => todo!(),
            KeyCode::NumpadEnter => todo!(),
            KeyCode::NumpadEqual => todo!(),
            KeyCode::NumpadHash => todo!(),
            KeyCode::NumpadMemoryAdd => todo!(),
            KeyCode::NumpadMemoryClear => todo!(),
            KeyCode::NumpadMemoryRecall => todo!(),
            KeyCode::NumpadMemoryStore => todo!(),
            KeyCode::NumpadMemorySubtract => todo!(),
            KeyCode::NumpadMultiply => todo!(),
            KeyCode::NumpadParenLeft => todo!(),
            KeyCode::NumpadParenRight => todo!(),
            KeyCode::NumpadStar => todo!(),
            KeyCode::NumpadSubtract => todo!(),
            KeyCode::Escape => Some(HidCodes::KeyboardEscape),
            KeyCode::PrintScreen => Some(HidCodes::KeyboardPrintScreen),
            KeyCode::ScrollLock => Some(HidCodes::KeyboardScrollLock),
            KeyCode::Pause => Some(HidCodes::KeyboardPause),
            KeyCode::AudioVolumeDown => Some(HidCodes::KeyboardVolumeDown),
            KeyCode::AudioVolumeMute => Some(HidCodes::KeyboardMute),
            KeyCode::AudioVolumeUp => Some(HidCodes::KeyboardVolumeUp),
            KeyCode::F1 => Some(HidCodes::KeyboardF1),
            KeyCode::F2 => Some(HidCodes::KeyboardF2),
            KeyCode::F3 => Some(HidCodes::KeyboardF3),
            KeyCode::F4 => Some(HidCodes::KeyboardF4),
            KeyCode::F5 => Some(HidCodes::KeyboardF5),
            KeyCode::F6 => Some(HidCodes::KeyboardF6),
            KeyCode::F7 => Some(HidCodes::KeyboardF7),
            KeyCode::F8 => Some(HidCodes::KeyboardF8),
            KeyCode::F9 => Some(HidCodes::KeyboardF9),
            KeyCode::F10 => Some(HidCodes::KeyboardF10),
            KeyCode::F11 => Some(HidCodes::KeyboardF11),
            KeyCode::F12 => Some(HidCodes::KeyboardF12),
            KeyCode::F13 => todo!(),
            KeyCode::F14 => todo!(),
            KeyCode::F15 => todo!(),
            KeyCode::F16 => todo!(),
            KeyCode::F17 => todo!(),
            KeyCode::F18 => todo!(),
            KeyCode::F19 => todo!(),
            KeyCode::F20 => todo!(),
            KeyCode::F21 => todo!(),
            KeyCode::F22 => todo!(),
            KeyCode::F23 => todo!(),
            KeyCode::F24 => todo!(),
            KeyCode::F25 => todo!(),
            KeyCode::F26 => todo!(),
            KeyCode::F27 => todo!(),
            KeyCode::F28 => todo!(),
            KeyCode::F29 => todo!(),
            KeyCode::F30 => todo!(),
            KeyCode::F31 => todo!(),
            KeyCode::F32 => todo!(),
            KeyCode::F33 => todo!(),
            KeyCode::F34 => todo!(),
            KeyCode::F35 => todo!(),
            _ => panic!("Shouldn't be here"),
        }
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
    pub dir: ElementState,
}

impl MouseButtons {
    pub fn new(button: MouseButton, dir: ElementState) -> Self {
        Self { button, dir }
    }

    pub fn to_hid(&self) -> HidCodes {
        match self.button {
            MouseButton::Left => HidCodes::MouseLeftClick,
            MouseButton::Right => HidCodes::MouseRightClick,
            MouseButton::Middle => HidCodes::MouseMiddleClick,
            MouseButton::Back => HidCodes::Mouse4,
            MouseButton::Forward => HidCodes::Mouse5,
            MouseButton::Other(_) => todo!("Mouse button not handled"),
        }
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
