use pyo3::prelude::*;
use pyo3::wrap_pyfunction; 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;
use macroquad::prelude as mq;
use crate::py_abstractions::Color::*;

use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer,derive::*} ;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;

use std::collections::HashSet;


#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
#[derive(Debug)]
pub struct KeyCodeSet {
    #[pyo3(get, set)]
    pub inner: HashSet<KeyCode>,
}

#[pymethods]
impl KeyCodeSet {
    #[new]
    pub fn new(inner: HashSet<KeyCode>) -> Self {
        Self { inner }
    }

    pub fn __repr__(&self) -> String {
        format!("KeyCodeSet(inner={:?}, )", self.inner)
    }
    
    pub fn __contains__(&self, key: KeyCode) -> bool {
        self.inner.contains(&key)
    }
    /// Get the number of elements in the set
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Return a new KeyCodeSet that is the union of this set and another set
    pub fn union(&self, other: &KeyCodeSet) -> KeyCodeSet {
        let mut new_set = self.inner.clone();
        new_set.extend(&other.inner);
        KeyCodeSet { inner: new_set }
    }

    /// Return a new KeyCodeSet that is the intersection of this set and another set
    pub fn intersection(&self, other: &KeyCodeSet) -> KeyCodeSet {
        let new_set = self.inner.intersection(&other.inner).cloned().collect();
        KeyCodeSet { inner: new_set }
    }

    /// Return a new KeyCodeSet that is the difference of this set and another set
    pub fn difference(&self, other: &KeyCodeSet) -> KeyCodeSet {
        let new_set = self.inner.difference(&other.inner).cloned().collect();
        KeyCodeSet { inner: new_set }
    }

    /// Check if this set is a subset of another set
    pub fn is_subset(&self, other: &KeyCodeSet) -> bool {
        self.inner.is_subset(&other.inner)
    }
    pub fn __eq__(&self, other: &KeyCodeSet) -> bool {
    self.inner == other.inner
    }

    pub fn __ne__(&self, other: &KeyCodeSet) -> bool {
        self.inner != other.inner
    }

    pub fn __le__(&self, other: &KeyCodeSet) -> bool {
        self.inner.is_subset(&other.inner)
    }

    pub fn __lt__(&self, other: &KeyCodeSet) -> bool {
        self.inner.is_subset(&other.inner) && self.inner.len() < other.inner.len()
    }

    pub fn __ge__(&self, other: &KeyCodeSet) -> bool {
        self.inner.is_superset(&other.inner)
    }

    pub fn __gt__(&self, other: &KeyCodeSet) -> bool {
        self.inner.is_superset(&other.inner) && self.inner.len() > other.inner.len()
    }
    pub fn __or__(&self, other: &KeyCodeSet) -> KeyCodeSet {
    self.union(other)
    }

    pub fn __and__(&self, other: &KeyCodeSet) -> KeyCodeSet {
        self.intersection(other)
    }

    pub fn __sub__(&self, other: &KeyCodeSet) -> KeyCodeSet {
        self.difference(other)
    }

    pub fn __xor__(&self, other: &KeyCodeSet) -> KeyCodeSet {
        let new_set: HashSet<KeyCode> = self.inner.symmetric_difference(&other.inner).cloned().collect();
        KeyCodeSet { inner: new_set }
    }

}

#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Space = 0x0020,
    Apostrophe = 0x0027,
    Comma = 0x002c,
    Minus = 0x002d,
    Period = 0x002e,
    Slash = 0x002f,
    Key0 = 0x0030,
    Key1 = 0x0031,
    Key2 = 0x0032,
    Key3 = 0x0033,
    Key4 = 0x0034,
    Key5 = 0x0035,
    Key6 = 0x0036,
    Key7 = 0x0037,
    Key8 = 0x0038,
    Key9 = 0x0039,
    Semicolon = 0x003b,
    Equal = 0x003d,
    A = 0x0041,
    B = 0x0042,
    C = 0x0043,
    D = 0x0044,
    E = 0x0045,
    F = 0x0046,
    G = 0x0047,
    H = 0x0048,
    I = 0x0049,
    J = 0x004a,
    K = 0x004b,
    L = 0x004c,
    M = 0x004d,
    N = 0x004e,
    O = 0x004f,
    P = 0x0050,
    Q = 0x0051,
    R = 0x0052,
    S = 0x0053,
    T = 0x0054,
    U = 0x0055,
    V = 0x0056,
    W = 0x0057,
    X = 0x0058,
    Y = 0x0059,
    Z = 0x005a,
    LeftBracket = 0x005b,
    Backslash = 0x005c,
    RightBracket = 0x005d,
    GraveAccent = 0x0060,
    World1 = 0x0100,
    World2 = 0x0101,
    Escape = 0xff1b,
    Enter = 0xff0d,
    Tab = 0xff09,
    Backspace = 0xff08,
    Insert = 0xff63,
    Delete = 0xffff,
    Right = 0xff53,
    Left = 0xff51,
    Down = 0xff54,
    Up = 0xff52,
    PageUp = 0xff55,
    PageDown = 0xff56,
    Home = 0xff50,
    End = 0xff57,
    CapsLock = 0xffe5,
    ScrollLock = 0xff14,
    NumLock = 0xff7f,
    PrintScreen = 0xfd1d,
    Pause = 0xff13,
    F1 = 0xffbe,
    F2 = 0xffbf,
    F3 = 0xffc0,
    F4 = 0xffc1,
    F5 = 0xffc2,
    F6 = 0xffc3,
    F7 = 0xffc4,
    F8 = 0xffc5,
    F9 = 0xffc6,
    F10 = 0xffc7,
    F11 = 0xffc8,
    F12 = 0xffc9,
    F13 = 0xffca,
    F14 = 0xffcb,
    F15 = 0xffcc,
    F16 = 0xffcd,
    F17 = 0xffce,
    F18 = 0xffcf,
    F19 = 0xffd0,
    F20 = 0xffd1,
    F21 = 0xffd2,
    F22 = 0xffd3,
    F23 = 0xffd4,
    F24 = 0xffd5,
    F25 = 0xffd6,
    Kp0 = 0xffb0,
    Kp1 = 0xffb1,
    Kp2 = 0xffb2,
    Kp3 = 0xffb3,
    Kp4 = 0xffb4,
    Kp5 = 0xffb5,
    Kp6 = 0xffb6,
    Kp7 = 0xffb7,
    Kp8 = 0xffb8,
    Kp9 = 0xffb9,
    KpDecimal = 0xffae,
    KpDivide = 0xffaf,
    KpMultiply = 0xffaa,
    KpSubtract = 0xffad,
    KpAdd = 0xffab,
    KpEnter = 0xff8d,
    KpEqual = 0xffbd,
    LeftShift = 0xffe1,
    LeftControl = 0xffe3,
    LeftAlt = 0xffe9,
    LeftSuper = 0xffeb,
    RightShift = 0xffe2,
    RightControl = 0xffe4,
    RightAlt = 0xffea,
    RightSuper = 0xffec,
    Menu = 0xff67,
    Back = 0xff04,
    Unknown = 0x01ff,
}


impl From<macroquad::input::KeyCode> for KeyCode {


    fn from(mq_key: macroquad::input::KeyCode) -> Self {
        match mq_key {
            macroquad::input::KeyCode::Space => KeyCode::Space,
            macroquad::input::KeyCode::Apostrophe => KeyCode::Apostrophe,
            macroquad::input::KeyCode::Comma => KeyCode::Comma,
            macroquad::input::KeyCode::Minus => KeyCode::Minus,
            macroquad::input::KeyCode::Period => KeyCode::Period,
            macroquad::input::KeyCode::Slash => KeyCode::Slash,
            macroquad::input::KeyCode::Key0 => KeyCode::Key0,
            macroquad::input::KeyCode::Key1 => KeyCode::Key1,
            macroquad::input::KeyCode::Key2 => KeyCode::Key2,
            macroquad::input::KeyCode::Key3 => KeyCode::Key3,
            macroquad::input::KeyCode::Key4 => KeyCode::Key4,
            macroquad::input::KeyCode::Key5 => KeyCode::Key5,
            macroquad::input::KeyCode::Key6 => KeyCode::Key6,
            macroquad::input::KeyCode::Key7 => KeyCode::Key7,
            macroquad::input::KeyCode::Key8 => KeyCode::Key8,
            macroquad::input::KeyCode::Key9 => KeyCode::Key9,
            macroquad::input::KeyCode::Semicolon => KeyCode::Semicolon,
            macroquad::input::KeyCode::Equal => KeyCode::Equal,
            macroquad::input::KeyCode::A => KeyCode::A,
            macroquad::input::KeyCode::B => KeyCode::B,
            macroquad::input::KeyCode::C => KeyCode::C,
            macroquad::input::KeyCode::D => KeyCode::D,
            macroquad::input::KeyCode::E => KeyCode::E,
            macroquad::input::KeyCode::F => KeyCode::F,
            macroquad::input::KeyCode::G => KeyCode::G,
            macroquad::input::KeyCode::H => KeyCode::H,
            macroquad::input::KeyCode::I => KeyCode::I,
            macroquad::input::KeyCode::J => KeyCode::J,
            macroquad::input::KeyCode::K => KeyCode::K,
            macroquad::input::KeyCode::L => KeyCode::L,
            macroquad::input::KeyCode::M => KeyCode::M,
            macroquad::input::KeyCode::N => KeyCode::N,
            macroquad::input::KeyCode::O => KeyCode::O,
            macroquad::input::KeyCode::P => KeyCode::P,
            macroquad::input::KeyCode::Q => KeyCode::Q,
            macroquad::input::KeyCode::R => KeyCode::R,
            macroquad::input::KeyCode::S => KeyCode::S,
            macroquad::input::KeyCode::T => KeyCode::T,
            macroquad::input::KeyCode::U => KeyCode::U,
            macroquad::input::KeyCode::V => KeyCode::V,
            macroquad::input::KeyCode::W => KeyCode::W,
            macroquad::input::KeyCode::X => KeyCode::X,
            macroquad::input::KeyCode::Y => KeyCode::Y,
            macroquad::input::KeyCode::Z => KeyCode::Z,
            macroquad::input::KeyCode::LeftBracket => KeyCode::LeftBracket,
            macroquad::input::KeyCode::Backslash => KeyCode::Backslash,
            macroquad::input::KeyCode::RightBracket => KeyCode::RightBracket,
            macroquad::input::KeyCode::GraveAccent => KeyCode::GraveAccent,
            macroquad::input::KeyCode::World1 => KeyCode::World1,
            macroquad::input::KeyCode::World2 => KeyCode::World2,
            macroquad::input::KeyCode::Escape => KeyCode::Escape,
            macroquad::input::KeyCode::Enter => KeyCode::Enter,
            macroquad::input::KeyCode::Tab => KeyCode::Tab,
            macroquad::input::KeyCode::Backspace => KeyCode::Backspace,
            macroquad::input::KeyCode::Insert => KeyCode::Insert,
            macroquad::input::KeyCode::Delete => KeyCode::Delete,
            macroquad::input::KeyCode::Right => KeyCode::Right,
            macroquad::input::KeyCode::Left => KeyCode::Left,
            macroquad::input::KeyCode::Down => KeyCode::Down,
            macroquad::input::KeyCode::Up => KeyCode::Up,
            macroquad::input::KeyCode::PageUp => KeyCode::PageUp,
            macroquad::input::KeyCode::PageDown => KeyCode::PageDown,
            macroquad::input::KeyCode::Home => KeyCode::Home,
            macroquad::input::KeyCode::End => KeyCode::End,
            macroquad::input::KeyCode::CapsLock => KeyCode::CapsLock,
            macroquad::input::KeyCode::ScrollLock => KeyCode::ScrollLock,
            macroquad::input::KeyCode::NumLock => KeyCode::NumLock,
            macroquad::input::KeyCode::PrintScreen => KeyCode::PrintScreen,
            macroquad::input::KeyCode::Pause => KeyCode::Pause,
            macroquad::input::KeyCode::F1 => KeyCode::F1,
            macroquad::input::KeyCode::F2 => KeyCode::F2,
            macroquad::input::KeyCode::F3 => KeyCode::F3,
            macroquad::input::KeyCode::F4 => KeyCode::F4,
            macroquad::input::KeyCode::F5 => KeyCode::F5,
            macroquad::input::KeyCode::F6 => KeyCode::F6,
            macroquad::input::KeyCode::F7 => KeyCode::F7,
            macroquad::input::KeyCode::F8 => KeyCode::F8,
            macroquad::input::KeyCode::F9 => KeyCode::F9,
            macroquad::input::KeyCode::F10 => KeyCode::F10,
            macroquad::input::KeyCode::F11 => KeyCode::F11,
            macroquad::input::KeyCode::F12 => KeyCode::F12,
            macroquad::input::KeyCode::F13 => KeyCode::F13,
            macroquad::input::KeyCode::F14 => KeyCode::F14,
            macroquad::input::KeyCode::F15 => KeyCode::F15,
            macroquad::input::KeyCode::F16 => KeyCode::F16,
            macroquad::input::KeyCode::F17 => KeyCode::F17,
            macroquad::input::KeyCode::F18 => KeyCode::F18,
            macroquad::input::KeyCode::F19 => KeyCode::F19,
            macroquad::input::KeyCode::F20 => KeyCode::F20,
            macroquad::input::KeyCode::F21 => KeyCode::F21,
            macroquad::input::KeyCode::F22 => KeyCode::F22,
            macroquad::input::KeyCode::F23 => KeyCode::F23,
            macroquad::input::KeyCode::F24 => KeyCode::F24,
            macroquad::input::KeyCode::Kp0 => KeyCode::Kp0,
            macroquad::input::KeyCode::Kp1 => KeyCode::Kp1,
            macroquad::input::KeyCode::Kp2 => KeyCode::Kp2,
            macroquad::input::KeyCode::Kp3 => KeyCode::Kp3,
            macroquad::input::KeyCode::Kp4 => KeyCode::Kp4,
            macroquad::input::KeyCode::Kp5 => KeyCode::Kp5,
            macroquad::input::KeyCode::Kp6 => KeyCode::Kp6,
            macroquad::input::KeyCode::Kp7 => KeyCode::Kp7,
            macroquad::input::KeyCode::Kp8 => KeyCode::Kp8,
            macroquad::input::KeyCode::Kp9 => KeyCode::Kp9,
            macroquad::input::KeyCode::KpDecimal => KeyCode::KpDecimal,
            macroquad::input::KeyCode::KpDivide => KeyCode::KpDivide,
            macroquad::input::KeyCode::KpMultiply => KeyCode::KpMultiply,
            macroquad::input::KeyCode::KpSubtract => KeyCode::KpSubtract,
            macroquad::input::KeyCode::KpAdd => KeyCode::KpAdd,
            macroquad::input::KeyCode::KpEnter => KeyCode::KpEnter,
            macroquad::input::KeyCode::KpEqual => KeyCode::KpEqual,
            macroquad::input::KeyCode::LeftShift => KeyCode::LeftShift,
            macroquad::input::KeyCode::LeftControl => KeyCode::LeftControl,
            macroquad::input::KeyCode::LeftAlt => KeyCode::LeftAlt,
            macroquad::input::KeyCode::LeftSuper => KeyCode::LeftSuper,
            macroquad::input::KeyCode::RightShift => KeyCode::RightShift,
            macroquad::input::KeyCode::RightControl => KeyCode::RightControl,
            macroquad::input::KeyCode::RightAlt => KeyCode::RightAlt,
            macroquad::input::KeyCode::RightSuper => KeyCode::RightSuper,
            macroquad::input::KeyCode::Menu => KeyCode::Menu,
            macroquad::input::KeyCode::Back => KeyCode::Back,
            _ => KeyCode::Unknown,
        }
    }
}
