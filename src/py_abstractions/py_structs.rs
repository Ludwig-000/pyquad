use pyo3::prelude::*;
use pyo3::wrap_pyfunction; 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;
use macroquad::prelude as mq;


use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer,derive::*} ;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;
// macros 
macro_rules! make_basic_pyclass {
    ($name:ident, { $($field:ident : $ty:ty),* $(,)? }) => {
        #[gen_stub_pyclass]
        #[pyclass]
        #[derive(Clone)]
        #[derive(Debug)]
        pub struct $name {
            $(#[pyo3(get, set)]
            pub $field: $ty),*
        }
        #[gen_stub_pymethods]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new($($field: $ty),*) -> Self {
                Self { $($field),* }
            }

            pub fn __repr__(&self) -> String {
                format!(
                    concat!(stringify!($name), "(", $(stringify!($field), "={:?}, ",)* ")"),
                    $(self.$field),*
                )
            }
        }
    };
}


//macro_rules! make_Codeset {
//    ($name:ident, { $($field:ident : $ty:ty),* $(,)? }) => {
//        #[pyclass]
//        #[derive(Clone, Debug)]
//        pub struct $name {
//            $(#[pyo3(get, set)]
//            pub $field: $ty),*
//        }

//        #[pymethods]
//        impl $name {
//            #[new]
//            pub fn new($($field: $ty),*) -> Self {
//                Self { $($field),* }
//            }

//            pub fn __repr__(&self) -> String {
//                format!(
//                    concat!(stringify!($name), "(", $(stringify!($field), "={:?}, ",)* ")"),
//                    $(self.$field),*
//                )
//                .trim_end_matches(", ").to_string() + ")"
//            }

//            $(
//                pub fn __contains__(&self, key: KeyCode) -> bool
//                where
//                    $ty: std::ops::Deref<Target = std::collections::HashSet<KeyCode>>
//                {
//                    self.$field.contains(&key)
//                }
//            )*
//        }
//    };
//}
// classes: 




make_basic_pyclass!(Config, { window_title: String, window_width: i32, window_height: i32, fullscreen: bool, vsync: bool,sample_count: i32, window_resizable: bool, stop_pyton_when_closing_window: bool});



make_basic_pyclass!(DVec2, { x: f32,y: f32});
make_basic_pyclass!(DVec3, { x: f32,y: f32,z: f32});
make_basic_pyclass!(DVec4, { x: f32,y: f32,z: f32,w: f32});


make_basic_pyclass!(Image, { bytes: Vec<u8>,width: u16,height: u16});

make_basic_pyclass!(Circle, {x: f32,y: f32,r: f32});
make_basic_pyclass!(Quat, {x: f32,y: f32,z: f32, w: f32});
make_basic_pyclass!(Rect, {x: f32,y: f32,w: f32, h: f32});

make_basic_pyclass!(Color, {r: f32, g: f32, b: f32, a: f32});




make_basic_pyclass!(DMat2, {x_axis: DVec2, y_axis: DVec2});
make_basic_pyclass!(DMat3, {x_axis: DVec3, y_axis: DVec3, z_axis: DVec3});
make_basic_pyclass!(DMat4, {x_axis: DVec4, y_axis: DVec4, z_axis: DVec4, w_axis: DVec4});



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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] // Crucial for HashSet
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


pub fn all_colors(m: &Bound<'_, PyModule>, py: Python<'_>) -> PyResult<()> {
    
    
    m.add("CLOUDY_BLUE", Color { r: 0.6745098039215687, g: 0.7607843137254902, b: 0.8509803921568627, a: 1.0 })?;
    m.add("DARK_PASTEL_GREEN", Color { r: 0.33725490196078434, g: 0.6823529411764706, b: 0.3411764705882353, a: 1.0 })?;
    m.add("DUST", Color { r: 0.6980392156862745, g: 0.6, b: 0.43137254901960786, a: 1.0 })?;
    m.add("ELECTRIC_LIME", Color { r: 0.6588235294117647, g: 1.0, b: 0.01568627450980392, a: 1.0 })?;
    m.add("FRESH_GREEN", Color { r: 0.4117647058823529, g: 0.8470588235294118, b: 0.30980392156862746, a: 1.0 })?;
    m.add("LIGHT_EGGPLANT", Color { r: 0.5372549019607843, g: 0.27058823529411763, b: 0.5215686274509804, a: 1.0 })?;
    m.add("NASTY_GREEN", Color { r: 0.4392156862745098, g: 0.6980392156862745, b: 0.24705882352941178, a: 1.0 })?;
    m.add("REALLY_LIGHT_BLUE", Color { r: 0.8313725490196079, g: 1.0, b: 1.0, a: 1.0 })?;
    m.add("TEA", Color { r: 0.396078431372549, g: 0.6705882352941176, b: 0.48627450980392156, a: 1.0 })?;
    m.add("WARM_PURPLE", Color { r: 0.5843137254901961, g: 0.1803921568627451, b: 0.5607843137254902, a: 1.0 })?;
    m.add("YELLOWISH_TAN", Color { r: 0.9882352941176471, g: 0.9882352941176471, b: 0.5058823529411764, a: 1.0 })?;
    m.add("CEMENT", Color { r: 0.6470588235294118, g: 0.6392156862745098, b: 0.5686274509803921, a: 1.0 })?;
    m.add("DARK_GRASS_GREEN", Color { r: 0.2196078431372549, g: 0.5019607843137255, b: 0.01568627450980392, a: 1.0 })?;
    m.add("DUSTY_TEAL", Color { r: 0.2980392156862745, g: 0.5647058823529412, b: 0.5215686274509804, a: 1.0 })?;
    m.add("GREY_TEAL", Color { r: 0.3686274509803922, g: 0.6078431372549019, b: 0.5411764705882353, a: 1.0 })?;
    m.add("MACARONI_AND_CHEESE", Color { r: 0.9372549019607843, g: 0.7058823529411765, b: 0.20784313725490197, a: 1.0 })?;
    m.add("PINKISH_TAN", Color { r: 0.8509803921568627, g: 0.6078431372549019, b: 0.5098039215686274, a: 1.0 })?;
    m.add("SPRUCE", Color { r: 0.0392156862745098, g: 0.37254901960784315, b: 0.2196078431372549, a: 1.0 })?;
    m.add("STRONG_BLUE", Color { r: 0.047058823529411764, g: 0.023529411764705882, b: 0.9686274509803922, a: 1.0 })?;
    m.add("TOXIC_GREEN", Color { r: 0.3803921568627451, g: 0.8705882352941177, b: 0.16470588235294117, a: 1.0 })?;
    m.add("WINDOWS_BLUE", Color { r: 0.21568627450980393, g: 0.47058823529411764, b: 0.7490196078431373, a: 1.0 })?;
    m.add("BLUE_BLUE", Color { r: 0.13333333333333333, g: 0.25882352941176473, b: 0.7803921568627451, a: 1.0 })?;
    m.add("BLUE_WITH_A_HINT_OF_PURPLE", Color { r: 0.3254901960784314, g: 0.23529411764705882, b: 0.7764705882352941, a: 1.0 })?;
    m.add("BOOGER", Color { r: 0.6078431372549019, g: 0.7098039215686275, b: 0.23529411764705882, a: 1.0 })?;
    m.add("BRIGHT_SEA_GREEN", Color { r: 0.0196078431372549, g: 1.0, b: 0.6509803921568628, a: 1.0 })?;
    m.add("DARK_GREEN_BLUE", Color { r: 0.12156862745098039, g: 0.38823529411764707, b: 0.3411764705882353, a: 1.0 })?;
    m.add("DEEP_TURQUOISE", Color { r: 0.00392156862745098, g: 0.45098039215686275, b: 0.4549019607843137, a: 1.0 })?;
    m.add("GREEN_TEAL", Color { r: 0.047058823529411764, g: 0.7098039215686275, b: 0.4666666666666667, a: 1.0 })?;
    m.add("STRONG_PINK", Color { r: 1.0, g: 0.027450980392156862, b: 0.5372549019607843, a: 1.0 })?;
    m.add("BLAND", Color { r: 0.6862745098039216, g: 0.6588235294117647, b: 0.5450980392156862, a: 1.0 })?;
    m.add("DEEP_AQUA", Color { r: 0.03137254901960784, g: 0.47058823529411764, b: 0.4980392156862745, a: 1.0 })?;
    m.add("LAVENDER_PINK", Color { r: 0.8666666666666667, g: 0.5215686274509804, b: 0.8431372549019608, a: 1.0 })?;
    m.add("LIGHT_MOSS_GREEN", Color { r: 0.6509803921568628, g: 0.7843137254901961, b: 0.4588235294117647, a: 1.0 })?;
    m.add("LIGHT_SEAFOAM_GREEN", Color { r: 0.6549019607843137, g: 1.0, b: 0.7098039215686275, a: 1.0 })?;
    m.add("OLIVE_YELLOW", Color { r: 0.7607843137254902, g: 0.7176470588235294, b: 0.03529411764705882, a: 1.0 })?;
    m.add("PIG_PINK", Color { r: 0.9058823529411765, g: 0.5568627450980392, b: 0.6470588235294118, a: 1.0 })?;
    m.add("DEEP_LILAC", Color { r: 0.5882352941176471, g: 0.43137254901960786, b: 0.7411764705882353, a: 1.0 })?;
    m.add("DESERT", Color { r: 0.8, g: 0.6784313725490196, b: 0.3764705882352941, a: 1.0 })?;
    m.add("DUSTY_LAVENDER", Color { r: 0.6745098039215687, g: 0.5254901960784314, b: 0.6588235294117647, a: 1.0 })?;
    m.add("PURPLEY_GREY", Color { r: 0.5803921568627451, g: 0.49411764705882355, b: 0.5803921568627451, a: 1.0 })?;
    m.add("PURPLY", Color { r: 0.596078431372549, g: 0.24705882352941178, b: 0.6980392156862745, a: 1.0 })?;
    m.add("CANDY_PINK", Color { r: 1.0, g: 0.38823529411764707, b: 0.9137254901960784, a: 1.0 })?;
    m.add("LIGHT_PASTEL_GREEN", Color { r: 0.6980392156862745, g: 0.984313725490196, b: 0.6470588235294118, a: 1.0 })?;
    m.add("BORING_GREEN", Color { r: 0.38823529411764707, g: 0.7019607843137254, b: 0.396078431372549, a: 1.0 })?;
    m.add("KIWI_GREEN", Color { r: 0.5568627450980392, g: 0.8980392156862745, b: 0.24705882352941178, a: 1.0 })?;
    m.add("LIGHT_GREY_GREEN", Color { r: 0.7176470588235294, g: 0.8823529411764706, b: 0.6313725490196078, a: 1.0 })?;
    m.add("ORANGE_PINK", Color { r: 1.0, g: 0.43529411764705883, b: 0.3215686274509804, a: 1.0 })?;
    m.add("TEA_GREEN", Color { r: 0.7411764705882353, g: 0.9725490196078431, b: 0.6392156862745098, a: 1.0 })?;
    m.add("VERY_LIGHT_BROWN", Color { r: 0.8274509803921568, g: 0.7137254901960784, b: 0.5137254901960784, a: 1.0 })?;
    m.add("EGG_SHELL", Color { r: 1.0, g: 0.9882352941176471, b: 0.7686274509803922, a: 1.0 })?;
    m.add("EGGPLANT_PURPLE", Color { r: 0.2627450980392157, g: 0.0196078431372549, b: 0.2549019607843137, a: 1.0 })?;
    m.add("POWDER_PINK", Color { r: 1.0, g: 0.6980392156862745, b: 0.8156862745098039, a: 1.0 })?;
    m.add("REDDISH_GREY", Color { r: 0.6, g: 0.4588235294117647, b: 0.4392156862745098, a: 1.0 })?;
    m.add("BABY_SHIT_BROWN", Color { r: 0.6784313725490196, g: 0.5647058823529412, b: 0.050980392156862744, a: 1.0 })?;
    m.add("LILIAC", Color { r: 0.7686274509803922, g: 0.5568627450980392, b: 0.9921568627450981, a: 1.0 })?;
    m.add("STORMY_BLUE", Color { r: 0.3137254901960784, g: 0.4823529411764706, b: 0.611764705882353, a: 1.0 })?;
    m.add("UGLY_BROWN", Color { r: 0.49019607843137253, g: 0.44313725490196076, b: 0.011764705882352941, a: 1.0 })?;
    m.add("CUSTARD", Color { r: 1.0, g: 0.9921568627450981, b: 0.47058823529411764, a: 1.0 })?;
    m.add("DARKISH_PINK", Color { r: 0.8549019607843137, g: 0.27450980392156865, b: 0.49019607843137253, a: 1.0 })?;
    m.add("DEEP_BROWN", Color { r: 0.2549019607843137, g: 0.00784313725490196, b: 0.0, a: 1.0 })?;
    m.add("GREENISH_BEIGE", Color { r: 0.788235294117647, g: 0.8196078431372549, b: 0.4745098039215686, a: 1.0 })?;
    m.add("MANILLA", Color { r: 1.0, g: 0.9803921568627451, b: 0.5254901960784314, a: 1.0 })?;
    m.add("OFF_BLUE", Color { r: 0.33725490196078434, g: 0.5176470588235295, b: 0.6823529411764706, a: 1.0 })?;
    m.add("BATTLESHIP_GREY", Color { r: 0.4196078431372549, g: 0.48627450980392156, b: 0.5215686274509804, a: 1.0 })?;
    m.add("BROWNY_GREEN", Color { r: 0.43529411764705883, g: 0.4235294117647059, b: 0.0392156862745098, a: 1.0 })?;
    m.add("BRUISE", Color { r: 0.49411764705882355, g: 0.25098039215686274, b: 0.44313725490196076, a: 1.0 })?;
    m.add("KELLEY_GREEN", Color { r: 0.0, g: 0.5764705882352941, b: 0.21568627450980393, a: 1.0 })?;
    m.add("SICKLY_YELLOW", Color { r: 0.8156862745098039, g: 0.8941176470588236, b: 0.1607843137254902, a: 1.0 })?;
    m.add("SUNNY_YELLOW", Color { r: 1.0, g: 0.9764705882352941, b: 0.09019607843137255, a: 1.0 })?;
    m.add("AZUL", Color { r: 0.11372549019607843, g: 0.36470588235294116, b: 0.9254901960784314, a: 1.0 })?;
    m.add("DARKGREEN", Color { r: 0.0196078431372549, g: 0.28627450980392155, b: 0.027450980392156862, a: 1.0 })?;
    m.add("GREEN_YELLOW", Color { r: 0.7098039215686275, g: 0.807843137254902, b: 0.03137254901960784, a: 1.0 })?;
    m.add("LICHEN", Color { r: 0.5607843137254902, g: 0.7137254901960784, b: 0.4823529411764706, a: 1.0 })?;
    m.add("LIGHT_LIGHT_GREEN", Color { r: 0.7843137254901961, g: 1.0, b: 0.6901960784313725, a: 1.0 })?;
    m.add("PALE_GOLD", Color { r: 0.9921568627450981, g: 0.8705882352941177, b: 0.4235294117647059, a: 1.0 })?;
    m.add("SUN_YELLOW", Color { r: 1.0, g: 0.8745098039215686, b: 0.13333333333333333, a: 1.0 })?;
    m.add("TAN_GREEN", Color { r: 0.6627450980392157, g: 0.7450980392156863, b: 0.4392156862745098, a: 1.0 })?;
    m.add("BURPLE", Color { r: 0.40784313725490196, g: 0.19607843137254902, b: 0.8901960784313725, a: 1.0 })?;
    m.add("BUTTERSCOTCH", Color { r: 0.9921568627450981, g: 0.6941176470588235, b: 0.2784313725490196, a: 1.0 })?;
    m.add("TOUPE", Color { r: 0.7803921568627451, g: 0.6745098039215687, b: 0.49019607843137253, a: 1.0 })?;
    m.add("DARK_CREAM", Color { r: 1.0, g: 0.9529411764705882, b: 0.6039215686274509, a: 1.0 })?;
    m.add("INDIAN_RED", Color { r: 0.5215686274509804, g: 0.054901960784313725, b: 0.01568627450980392, a: 1.0 })?;
    m.add("LIGHT_LAVENDAR", Color { r: 0.9372549019607843, g: 0.7529411764705882, b: 0.996078431372549, a: 1.0 })?;
    m.add("POISON_GREEN", Color { r: 0.25098039215686274, g: 0.9921568627450981, b: 0.0784313725490196, a: 1.0 })?;
    m.add("BABY_PUKE_GREEN", Color { r: 0.7137254901960784, g: 0.7686274509803922, b: 0.023529411764705882, a: 1.0 })?;
    m.add("BRIGHT_YELLOW_GREEN", Color { r: 0.615686274509804, g: 1.0, b: 0.0, a: 1.0 })?;
    m.add("CHARCOAL_GREY", Color { r: 0.23529411764705882, g: 0.2549019607843137, b: 0.25882352941176473, a: 1.0 })?;
    m.add("SQUASH", Color { r: 0.9490196078431372, g: 0.6705882352941176, b: 0.08235294117647059, a: 1.0 })?;
    m.add("CINNAMON", Color { r: 0.6745098039215687, g: 0.30980392156862746, b: 0.023529411764705882, a: 1.0 })?;
    m.add("LIGHT_PEA_GREEN", Color { r: 0.7686274509803922, g: 0.996078431372549, b: 0.5098039215686274, a: 1.0 })?;
    m.add("RADIOACTIVE_GREEN", Color { r: 0.17254901960784313, g: 0.9803921568627451, b: 0.12156862745098039, a: 1.0 })?;
    m.add("RAW_SIENNA", Color { r: 0.6039215686274509, g: 0.3843137254901961, b: 0.0, a: 1.0 })?;
    m.add("BABY_PURPLE", Color { r: 0.792156862745098, g: 0.6078431372549019, b: 0.9686274509803922, a: 1.0 })?;
    m.add("COCOA", Color { r: 0.5294117647058824, g: 0.37254901960784315, b: 0.25882352941176473, a: 1.0 })?;
    m.add("LIGHT_ROYAL_BLUE", Color { r: 0.22745098039215686, g: 0.1803921568627451, b: 0.996078431372549, a: 1.0 })?;
    m.add("ORANGEISH", Color { r: 0.9921568627450981, g: 0.5529411764705883, b: 0.28627450980392155, a: 1.0 })?;
    m.add("RUST_BROWN", Color { r: 0.5450980392156862, g: 0.19215686274509805, b: 0.011764705882352941, a: 1.0 })?;
    m.add("SAND_BROWN", Color { r: 0.796078431372549, g: 0.6470588235294118, b: 0.3764705882352941, a: 1.0 })?;
    m.add("SWAMP", Color { r: 0.4117647058823529, g: 0.5137254901960784, b: 0.2235294117647059, a: 1.0 })?;
    m.add("TEALISH_GREEN", Color { r: 0.047058823529411764, g: 0.8627450980392157, b: 0.45098039215686275, a: 1.0 })?;
    m.add("BURNT_SIENA", Color { r: 0.7176470588235294, g: 0.3215686274509804, b: 0.011764705882352941, a: 1.0 })?;
    m.add("CAMO", Color { r: 0.4980392156862745, g: 0.5607843137254902, b: 0.3058823529411765, a: 1.0 })?;
    m.add("DUSK_BLUE", Color { r: 0.14901960784313725, g: 0.3254901960784314, b: 0.5529411764705883, a: 1.0 })?;
    m.add("FERN", Color { r: 0.38823529411764707, g: 0.6627450980392157, b: 0.3137254901960784, a: 1.0 })?;
    m.add("OLD_ROSE", Color { r: 0.7843137254901961, g: 0.4980392156862745, b: 0.5372549019607843, a: 1.0 })?;
    m.add("PALE_LIGHT_GREEN", Color { r: 0.6941176470588235, g: 0.9882352941176471, b: 0.6, a: 1.0 })?;
    m.add("PEACHY_PINK", Color { r: 1.0, g: 0.6039215686274509, b: 0.5411764705882353, a: 1.0 })?;
    m.add("ROSY_PINK", Color { r: 0.9647058823529412, g: 0.40784313725490196, b: 0.5568627450980392, a: 1.0 })?;
    m.add("LIGHT_BLUISH_GREEN", Color { r: 0.4627450980392157, g: 0.9921568627450981, b: 0.6588235294117647, a: 1.0 })?;
    m.add("LIGHT_BRIGHT_GREEN", Color { r: 0.3254901960784314, g: 0.996078431372549, b: 0.3607843137254902, a: 1.0 })?;
    m.add("LIGHT_NEON_GREEN", Color { r: 0.3058823529411765, g: 0.9921568627450981, b: 0.32941176470588235, a: 1.0 })?;
    m.add("LIGHT_SEAFOAM", Color { r: 0.6274509803921569, g: 0.996078431372549, b: 0.7490196078431373, a: 1.0 })?;
    m.add("TIFFANY_BLUE", Color { r: 0.4823529411764706, g: 0.9490196078431372, b: 0.8549019607843137, a: 1.0 })?;
    m.add("WASHED_OUT_GREEN", Color { r: 0.7372549019607844, g: 0.9607843137254902, b: 0.6509803921568628, a: 1.0 })?;
    m.add("BROWNY_ORANGE", Color { r: 0.792156862745098, g: 0.4196078431372549, b: 0.00784313725490196, a: 1.0 })?;
    m.add("NICE_BLUE", Color { r: 0.06274509803921569, g: 0.47843137254901963, b: 0.6901960784313725, a: 1.0 })?;
    m.add("SAPPHIRE", Color { r: 0.12941176470588237, g: 0.2196078431372549, b: 0.6705882352941176, a: 1.0 })?;
    m.add("GREYISH_TEAL", Color { r: 0.44313725490196076, g: 0.6235294117647059, b: 0.5686274509803921, a: 1.0 })?;
    m.add("ORANGEY_YELLOW", Color { r: 0.9921568627450981, g: 0.7254901960784313, b: 0.08235294117647059, a: 1.0 })?;
    m.add("PARCHMENT", Color { r: 0.996078431372549, g: 0.9882352941176471, b: 0.6862745098039216, a: 1.0 })?;
    m.add("STRAW", Color { r: 0.9882352941176471, g: 0.9647058823529412, b: 0.4745098039215686, a: 1.0 })?;
    m.add("VERY_DARK_BROWN", Color { r: 0.11372549019607843, g: 0.00784313725490196, b: 0.0, a: 1.0 })?;
    m.add("TERRACOTA", Color { r: 0.796078431372549, g: 0.40784313725490196, b: 0.2627450980392157, a: 1.0 })?;
    m.add("UGLY_BLUE", Color { r: 0.19215686274509805, g: 0.4, b: 0.5411764705882353, a: 1.0 })?;
    m.add("CLEAR_BLUE", Color { r: 0.1411764705882353, g: 0.47843137254901963, b: 0.9921568627450981, a: 1.0 })?;
    m.add("CREME", Color { r: 1.0, g: 1.0, b: 0.7137254901960784, a: 1.0 })?;
    m.add("FOAM_GREEN", Color { r: 0.5647058823529412, g: 0.9921568627450981, b: 0.6627450980392157, a: 1.0 })?;
    m.add("GREY_GREEN", Color { r: 0.5254901960784314, g: 0.6313725490196078, b: 0.49019607843137253, a: 1.0 })?;
    m.add("LIGHT_GOLD", Color { r: 0.9921568627450981, g: 0.8627450980392157, b: 0.3607843137254902, a: 1.0 })?;
    m.add("SEAFOAM_BLUE", Color { r: 0.47058823529411764, g: 0.8196078431372549, b: 0.7137254901960784, a: 1.0 })?;
    m.add("TOPAZ", Color { r: 0.07450980392156863, g: 0.7333333333333333, b: 0.6862745098039216, a: 1.0 })?;
    m.add("VIOLET_PINK", Color { r: 0.984313725490196, g: 0.37254901960784315, b: 0.9882352941176471, a: 1.0 })?;
    m.add("WINTERGREEN", Color { r: 0.12549019607843137, g: 0.9764705882352941, b: 0.5254901960784314, a: 1.0 })?;
    m.add("YELLOW_TAN", Color { r: 1.0, g: 0.8901960784313725, b: 0.43137254901960786, a: 1.0 })?;
    m.add("DARK_FUCHSIA", Color { r: 0.615686274509804, g: 0.027450980392156862, b: 0.34901960784313724, a: 1.0 })?;
    m.add("INDIGO_BLUE", Color { r: 0.22745098039215686, g: 0.09411764705882353, b: 0.6941176470588235, a: 1.0 })?;
    m.add("LIGHT_YELLOWISH_GREEN", Color { r: 0.7607843137254902, g: 1.0, b: 0.5372549019607843, a: 1.0 })?;
    m.add("PALE_MAGENTA", Color { r: 0.8431372549019608, g: 0.403921568627451, b: 0.6784313725490196, a: 1.0 })?;
    m.add("RICH_PURPLE", Color { r: 0.4470588235294118, g: 0.0, b: 0.34509803921568627, a: 1.0 })?;
    m.add("SUNFLOWER_YELLOW", Color { r: 1.0, g: 0.8549019607843137, b: 0.011764705882352941, a: 1.0 })?;
    m.add("GREEN_BLUE", Color { r: 0.00392156862745098, g: 0.7529411764705882, b: 0.5529411764705883, a: 1.0 })?;
    m.add("LEATHER", Color { r: 0.6745098039215687, g: 0.4549019607843137, b: 0.20392156862745098, a: 1.0 })?;
    m.add("RACING_GREEN", Color { r: 0.00392156862745098, g: 0.27450980392156865, b: 0.0, a: 1.0 })?;
    m.add("VIVID_PURPLE", Color { r: 0.6, g: 0.0, b: 0.9803921568627451, a: 1.0 })?;
    m.add("DARK_ROYAL_BLUE", Color { r: 0.00784313725490196, g: 0.023529411764705882, b: 0.43529411764705883, a: 1.0 })?;
    m.add("HAZEL", Color { r: 0.5568627450980392, g: 0.4627450980392157, b: 0.09411764705882353, a: 1.0 })?;
    m.add("MUTED_PINK", Color { r: 0.8196078431372549, g: 0.4627450980392157, b: 0.5607843137254902, a: 1.0 })?;
    m.add("BOOGER_GREEN", Color { r: 0.5882352941176471, g: 0.7058823529411765, b: 0.011764705882352941, a: 1.0 })?;
    m.add("CANARY", Color { r: 0.9921568627450981, g: 1.0, b: 0.38823529411764707, a: 1.0 })?;
    m.add("COOL_GREY", Color { r: 0.5843137254901961, g: 0.6392156862745098, b: 0.6509803921568628, a: 1.0 })?;
    m.add("DARK_TAUPE", Color { r: 0.4980392156862745, g: 0.40784313725490196, b: 0.3058823529411765, a: 1.0 })?;
    m.add("DARKISH_PURPLE", Color { r: 0.4588235294117647, g: 0.09803921568627451, b: 0.45098039215686275, a: 1.0 })?;
    m.add("TRUE_GREEN", Color { r: 0.03137254901960784, g: 0.5803921568627451, b: 0.01568627450980392, a: 1.0 })?;
    m.add("CORAL_PINK", Color { r: 1.0, g: 0.3803921568627451, b: 0.38823529411764707, a: 1.0 })?;
    m.add("DARK_SAGE", Color { r: 0.34901960784313724, g: 0.5215686274509804, b: 0.33725490196078434, a: 1.0 })?;
    m.add("DARK_SLATE_BLUE", Color { r: 0.12941176470588237, g: 0.2784313725490196, b: 0.3803921568627451, a: 1.0 })?;
    m.add("FLAT_BLUE", Color { r: 0.23529411764705882, g: 0.45098039215686275, b: 0.6588235294117647, a: 1.0 })?;
    m.add("MUSHROOM", Color { r: 0.7294117647058823, g: 0.6196078431372549, b: 0.5333333333333333, a: 1.0 })?;
    m.add("RICH_BLUE", Color { r: 0.00784313725490196, g: 0.10588235294117647, b: 0.9764705882352941, a: 1.0 })?;
    m.add("DIRTY_PURPLE", Color { r: 0.45098039215686275, g: 0.2901960784313726, b: 0.396078431372549, a: 1.0 })?;
    m.add("GREENBLUE", Color { r: 0.13725490196078433, g: 0.7686274509803922, b: 0.5450980392156862, a: 1.0 })?;
    m.add("ICKY_GREEN", Color { r: 0.5607843137254902, g: 0.6823529411764706, b: 0.13333333333333333, a: 1.0 })?;
    m.add("LIGHT_KHAKI", Color { r: 0.9019607843137255, g: 0.9490196078431372, b: 0.6352941176470588, a: 1.0 })?;
    m.add("WARM_BLUE", Color { r: 0.29411764705882354, g: 0.3411764705882353, b: 0.8588235294117647, a: 1.0 })?;
    m.add("DARK_HOT_PINK", Color { r: 0.8509803921568627, g: 0.00392156862745098, b: 0.4, a: 1.0 })?;
    m.add("DEEP_SEA_BLUE", Color { r: 0.00392156862745098, g: 0.32941176470588235, b: 0.5098039215686274, a: 1.0 })?;
    m.add("CARMINE", Color { r: 0.615686274509804, g: 0.00784313725490196, b: 0.08627450980392157, a: 1.0 })?;
    m.add("DARK_YELLOW_GREEN", Color { r: 0.4470588235294118, g: 0.5607843137254902, b: 0.00784313725490196, a: 1.0 })?;
    m.add("PALE_PEACH", Color { r: 1.0, g: 0.8980392156862745, b: 0.6784313725490196, a: 1.0 })?;
    m.add("PLUM_PURPLE", Color { r: 0.3058823529411765, g: 0.0196078431372549, b: 0.3137254901960784, a: 1.0 })?;
    m.add("GOLDEN_ROD", Color { r: 0.9764705882352941, g: 0.7372549019607844, b: 0.03137254901960784, a: 1.0 })?;
    m.add("NEON_RED", Color { r: 1.0, g: 0.027450980392156862, b: 0.22745098039215686, a: 1.0 })?;
    m.add("OLD_PINK", Color { r: 0.7803921568627451, g: 0.4745098039215686, b: 0.5254901960784314, a: 1.0 })?;
    m.add("VERY_PALE_BLUE", Color { r: 0.8392156862745098, g: 1.0, b: 0.996078431372549, a: 1.0 })?;
    m.add("BLOOD_ORANGE", Color { r: 0.996078431372549, g: 0.29411764705882354, b: 0.011764705882352941, a: 1.0 })?;
    m.add("GRAPEFRUIT", Color { r: 0.9921568627450981, g: 0.34901960784313724, b: 0.33725490196078434, a: 1.0 })?;
    m.add("SAND_YELLOW", Color { r: 0.9882352941176471, g: 0.8823529411764706, b: 0.4, a: 1.0 })?;
    m.add("CLAY_BROWN", Color { r: 0.6980392156862745, g: 0.44313725490196076, b: 0.23921568627450981, a: 1.0 })?;
    m.add("DARK_BLUE_GREY", Color { r: 0.12156862745098039, g: 0.23137254901960785, b: 0.30196078431372547, a: 1.0 })?;
    m.add("FLAT_GREEN", Color { r: 0.4117647058823529, g: 0.615686274509804, b: 0.2980392156862745, a: 1.0 })?;
    m.add("LIGHT_GREEN_BLUE", Color { r: 0.33725490196078434, g: 0.9882352941176471, b: 0.6352941176470588, a: 1.0 })?;
    m.add("WARM_PINK", Color { r: 0.984313725490196, g: 0.3333333333333333, b: 0.5058823529411764, a: 1.0 })?;
    m.add("DODGER_BLUE", Color { r: 0.24313725490196078, g: 0.5098039215686274, b: 0.9882352941176471, a: 1.0 })?;
    m.add("GROSS_GREEN", Color { r: 0.6274509803921569, g: 0.7490196078431373, b: 0.08627450980392157, a: 1.0 })?;
    m.add("ICE", Color { r: 0.8392156862745098, g: 1.0, b: 0.9803921568627451, a: 1.0 })?;
    m.add("METALLIC_BLUE", Color { r: 0.30980392156862746, g: 0.45098039215686275, b: 0.5568627450980392, a: 1.0 })?;
    m.add("PALE_SALMON", Color { r: 1.0, g: 0.6941176470588235, b: 0.6039215686274509, a: 1.0 })?;
    m.add("SAP_GREEN", Color { r: 0.3607843137254902, g: 0.5450980392156862, b: 0.08235294117647059, a: 1.0 })?;
    m.add("ALGAE", Color { r: 0.32941176470588235, g: 0.6745098039215687, b: 0.40784313725490196, a: 1.0 })?;
    m.add("BLUEY_GREY", Color { r: 0.5372549019607843, g: 0.6274509803921569, b: 0.6901960784313725, a: 1.0 })?;
    m.add("GREENY_GREY", Color { r: 0.49411764705882355, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 })?;
    m.add("HIGHLIGHTER_GREEN", Color { r: 0.10588235294117647, g: 0.9882352941176471, b: 0.023529411764705882, a: 1.0 })?;
    m.add("LIGHT_LIGHT_BLUE", Color { r: 0.792156862745098, g: 1.0, b: 0.984313725490196, a: 1.0 })?;
    m.add("LIGHT_MINT", Color { r: 0.7137254901960784, g: 1.0, b: 0.7333333333333333, a: 1.0 })?;
    m.add("RAW_UMBER", Color { r: 0.6549019607843137, g: 0.3686274509803922, b: 0.03529411764705882, a: 1.0 })?;
    m.add("VIVID_BLUE", Color { r: 0.08235294117647059, g: 0.1803921568627451, b: 1.0, a: 1.0 })?;
    m.add("DEEP_LAVENDER", Color { r: 0.5529411764705883, g: 0.3686274509803922, b: 0.7176470588235294, a: 1.0 })?;
    m.add("DULL_TEAL", Color { r: 0.37254901960784315, g: 0.6196078431372549, b: 0.5607843137254902, a: 1.0 })?;
    m.add("LIGHT_GREENISH_BLUE", Color { r: 0.38823529411764707, g: 0.9686274509803922, b: 0.7058823529411765, a: 1.0 })?;
    m.add("MUD_GREEN", Color { r: 0.3764705882352941, g: 0.4, b: 0.00784313725490196, a: 1.0 })?;
    m.add("PINKY", Color { r: 0.9882352941176471, g: 0.5254901960784314, b: 0.6666666666666666, a: 1.0 })?;
    m.add("RED_WINE", Color { r: 0.5490196078431373, g: 0.0, b: 0.20392156862745098, a: 1.0 })?;
    m.add("SHIT_GREEN", Color { r: 0.4588235294117647, g: 0.5019607843137255, b: 0.0, a: 1.0 })?;
    m.add("TAN_BROWN", Color { r: 0.6705882352941176, g: 0.49411764705882355, b: 0.2980392156862745, a: 1.0 })?;
    m.add("DARKBLUE", Color { r: 0.011764705882352941, g: 0.027450980392156862, b: 0.39215686274509803, a: 1.0 })?;
    m.add("ROSA", Color { r: 0.996078431372549, g: 0.5254901960784314, b: 0.6431372549019608, a: 1.0 })?;
    m.add("LIPSTICK", Color { r: 0.8352941176470589, g: 0.09019607843137255, b: 0.3058823529411765, a: 1.0 })?;
    m.add("PALE_MAUVE", Color { r: 0.996078431372549, g: 0.8156862745098039, b: 0.9882352941176471, a: 1.0 })?;
    m.add("CLARET", Color { r: 0.40784313725490196, g: 0.0, b: 0.09411764705882353, a: 1.0 })?;
    m.add("DANDELION", Color { r: 0.996078431372549, g: 0.8745098039215686, b: 0.03137254901960784, a: 1.0 })?;
    m.add("ORANGERED", Color { r: 0.996078431372549, g: 0.25882352941176473, b: 0.058823529411764705, a: 1.0 })?;
    m.add("POOP_GREEN", Color { r: 0.43529411764705883, g: 0.48627450980392156, b: 0.0, a: 1.0 })?;
    m.add("RUBY", Color { r: 0.792156862745098, g: 0.00392156862745098, b: 0.2784313725490196, a: 1.0 })?;
    m.add("DARK", Color { r: 0.10588235294117647, g: 0.1411764705882353, b: 0.19215686274509805, a: 1.0 })?;
    m.add("GREENISH_TURQUOISE", Color { r: 0.0, g: 0.984313725490196, b: 0.6901960784313725, a: 1.0 })?;
    m.add("PASTEL_RED", Color { r: 0.8588235294117647, g: 0.34509803921568627, b: 0.33725490196078434, a: 1.0 })?;
    m.add("PISS_YELLOW", Color { r: 0.8666666666666667, g: 0.8392156862745098, b: 0.09411764705882353, a: 1.0 })?;
    m.add("BRIGHT_CYAN", Color { r: 0.2549019607843137, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 })?;
    m.add("DARK_CORAL", Color { r: 0.8117647058823529, g: 0.3215686274509804, b: 0.3058823529411765, a: 1.0 })?;
    m.add("ALGAE_GREEN", Color { r: 0.12941176470588237, g: 0.7647058823529411, b: 0.43529411764705883, a: 1.0 })?;
    m.add("DARKISH_RED", Color { r: 0.6627450980392157, g: 0.011764705882352941, b: 0.03137254901960784, a: 1.0 })?;
    m.add("REDDY_BROWN", Color { r: 0.43137254901960786, g: 0.06274509803921569, b: 0.0196078431372549, a: 1.0 })?;
    m.add("BLUSH_PINK", Color { r: 0.996078431372549, g: 0.5098039215686274, b: 0.5490196078431373, a: 1.0 })?;
    m.add("CAMOUFLAGE_GREEN", Color { r: 0.29411764705882354, g: 0.3803921568627451, b: 0.07450980392156863, a: 1.0 })?;
    m.add("LAWN_GREEN", Color { r: 0.30196078431372547, g: 0.6431372549019608, b: 0.03529411764705882, a: 1.0 })?;
    m.add("PUTTY", Color { r: 0.7450980392156863, g: 0.6823529411764706, b: 0.5411764705882353, a: 1.0 })?;
    m.add("VIBRANT_BLUE", Color { r: 0.011764705882352941, g: 0.2235294117647059, b: 0.9725490196078431, a: 1.0 })?;
    m.add("DARK_SAND", Color { r: 0.6588235294117647, g: 0.5607843137254902, b: 0.34901960784313724, a: 1.0 })?;
    m.add("PURPLE_BLUE", Color { r: 0.36470588235294116, g: 0.12941176470588237, b: 0.8156862745098039, a: 1.0 })?;
    m.add("SAFFRON", Color { r: 0.996078431372549, g: 0.6980392156862745, b: 0.03529411764705882, a: 1.0 })?;
    m.add("TWILIGHT", Color { r: 0.3058823529411765, g: 0.3176470588235294, b: 0.5450980392156862, a: 1.0 })?;
    m.add("WARM_BROWN", Color { r: 0.5882352941176471, g: 0.3058823529411765, b: 0.00784313725490196, a: 1.0 })?;
    m.add("BLUEGREY", Color { r: 0.5215686274509804, g: 0.6392156862745098, b: 0.6980392156862745, a: 1.0 })?;
    m.add("BUBBLE_GUM_PINK", Color { r: 1.0, g: 0.4117647058823529, b: 0.6862745098039216, a: 1.0 })?;
    m.add("DUCK_EGG_BLUE", Color { r: 0.7647058823529411, g: 0.984313725490196, b: 0.9568627450980393, a: 1.0 })?;
    m.add("GREENISH_CYAN", Color { r: 0.16470588235294117, g: 0.996078431372549, b: 0.7176470588235294, a: 1.0 })?;
    m.add("PETROL", Color { r: 0.0, g: 0.37254901960784315, b: 0.41568627450980394, a: 1.0 })?;
    m.add("ROYAL", Color { r: 0.047058823529411764, g: 0.09019607843137255, b: 0.5764705882352941, a: 1.0 })?;
    m.add("BUTTER", Color { r: 1.0, g: 1.0, b: 0.5058823529411764, a: 1.0 })?;
    m.add("DUSTY_ORANGE", Color { r: 0.9411764705882353, g: 0.5137254901960784, b: 0.22745098039215686, a: 1.0 })?;
    m.add("OFF_YELLOW", Color { r: 0.9450980392156862, g: 0.9529411764705882, b: 0.24705882352941178, a: 1.0 })?;
    m.add("PALE_OLIVE_GREEN", Color { r: 0.6941176470588235, g: 0.8235294117647058, b: 0.4823529411764706, a: 1.0 })?;
    m.add("ORANGISH", Color { r: 0.9882352941176471, g: 0.5098039215686274, b: 0.2901960784313726, a: 1.0 })?;
    m.add("LEAF", Color { r: 0.44313725490196076, g: 0.6666666666666666, b: 0.20392156862745098, a: 1.0 })?;
    m.add("LIGHT_BLUE_GREY", Color { r: 0.7176470588235294, g: 0.788235294117647, b: 0.8862745098039215, a: 1.0 })?;
    m.add("DRIED_BLOOD", Color { r: 0.29411764705882354, g: 0.00392156862745098, b: 0.00392156862745098, a: 1.0 })?;
    m.add("LIGHTISH_PURPLE", Color { r: 0.6470588235294118, g: 0.3215686274509804, b: 0.9019607843137255, a: 1.0 })?;
    m.add("RUSTY_RED", Color { r: 0.6862745098039216, g: 0.1843137254901961, b: 0.050980392156862744, a: 1.0 })?;
    m.add("LAVENDER_BLUE", Color { r: 0.5450980392156862, g: 0.5333333333333333, b: 0.9725490196078431, a: 1.0 })?;
    m.add("LIGHT_GRASS_GREEN", Color { r: 0.6039215686274509, g: 0.9686274509803922, b: 0.39215686274509803, a: 1.0 })?;
    m.add("LIGHT_MINT_GREEN", Color { r: 0.6509803921568628, g: 0.984313725490196, b: 0.6980392156862745, a: 1.0 })?;
    m.add("SUNFLOWER", Color { r: 1.0, g: 0.7725490196078432, b: 0.07058823529411765, a: 1.0 })?;
    m.add("VELVET", Color { r: 0.4588235294117647, g: 0.03137254901960784, b: 0.3176470588235294, a: 1.0 })?;
    m.add("BRICK_ORANGE", Color { r: 0.7568627450980392, g: 0.2901960784313726, b: 0.03529411764705882, a: 1.0 })?;
    m.add("LIGHTISH_RED", Color { r: 0.996078431372549, g: 0.1843137254901961, b: 0.2901960784313726, a: 1.0 })?;
    m.add("PURE_BLUE", Color { r: 0.00784313725490196, g: 0.011764705882352941, b: 0.8862745098039215, a: 1.0 })?;
    m.add("TWILIGHT_BLUE", Color { r: 0.0392156862745098, g: 0.2627450980392157, b: 0.47843137254901963, a: 1.0 })?;
    m.add("VIOLET_RED", Color { r: 0.6470588235294118, g: 0.0, b: 0.3333333333333333, a: 1.0 })?;
    m.add("YELLOWY_BROWN", Color { r: 0.6823529411764706, g: 0.5450980392156862, b: 0.047058823529411764, a: 1.0 })?;
    m.add("CARNATION", Color { r: 0.9921568627450981, g: 0.4745098039215686, b: 0.5607843137254902, a: 1.0 })?;
    m.add("MUDDY_YELLOW", Color { r: 0.7490196078431373, g: 0.6745098039215687, b: 0.0196078431372549, a: 1.0 })?;
    m.add("DARK_SEAFOAM_GREEN", Color { r: 0.24313725490196078, g: 0.6862745098039216, b: 0.4627450980392157, a: 1.0 })?;
    m.add("DEEP_ROSE", Color { r: 0.7803921568627451, g: 0.2784313725490196, b: 0.403921568627451, a: 1.0 })?;
    m.add("DUSTY_RED", Color { r: 0.7254901960784313, g: 0.2823529411764706, b: 0.3058823529411765, a: 1.0 })?;
    m.add("GREY_BLUE", Color { r: 0.39215686274509803, g: 0.49019607843137253, b: 0.5568627450980392, a: 1.0 })?;
    m.add("LEMON_LIME", Color { r: 0.7490196078431373, g: 0.996078431372549, b: 0.1568627450980392, a: 1.0 })?;
    m.add("PURPLE_PINK", Color { r: 0.8431372549019608, g: 0.1450980392156863, b: 0.8705882352941177, a: 1.0 })?;
    m.add("BROWN_YELLOW", Color { r: 0.6980392156862745, g: 0.592156862745098, b: 0.0196078431372549, a: 1.0 })?;
    m.add("PURPLE_BROWN", Color { r: 0.403921568627451, g: 0.22745098039215686, b: 0.24705882352941178, a: 1.0 })?;
    m.add("WISTERIA", Color { r: 0.6588235294117647, g: 0.49019607843137253, b: 0.7607843137254902, a: 1.0 })?;
    m.add("BANANA_YELLOW", Color { r: 0.9803921568627451, g: 0.996078431372549, b: 0.29411764705882354, a: 1.0 })?;
    m.add("LIPSTICK_RED", Color { r: 0.7529411764705882, g: 0.00784313725490196, b: 0.1843137254901961, a: 1.0 })?;
    m.add("WATER_BLUE", Color { r: 0.054901960784313725, g: 0.5294117647058824, b: 0.8, a: 1.0 })?;
    m.add("BROWN_GREY", Color { r: 0.5529411764705883, g: 0.5176470588235295, b: 0.40784313725490196, a: 1.0 })?;
    m.add("VIBRANT_PURPLE", Color { r: 0.6784313725490196, g: 0.011764705882352941, b: 0.8705882352941177, a: 1.0 })?;
    m.add("BABY_GREEN", Color { r: 0.5490196078431373, g: 1.0, b: 0.6196078431372549, a: 1.0 })?;
    m.add("BARF_GREEN", Color { r: 0.5803921568627451, g: 0.6745098039215687, b: 0.00784313725490196, a: 1.0 })?;
    m.add("EGGSHELL_BLUE", Color { r: 0.7686274509803922, g: 1.0, b: 0.9686274509803922, a: 1.0 })?;
    m.add("SANDY_YELLOW", Color { r: 0.9921568627450981, g: 0.9333333333333333, b: 0.45098039215686275, a: 1.0 })?;
    m.add("COOL_GREEN", Color { r: 0.2, g: 0.7215686274509804, b: 0.39215686274509803, a: 1.0 })?;
    m.add("PALE", Color { r: 1.0, g: 0.9764705882352941, b: 0.8156862745098039, a: 1.0 })?;
    m.add("BLUE_GREY", Color { r: 0.4588235294117647, g: 0.5529411764705883, b: 0.6392156862745098, a: 1.0 })?;
    m.add("HOT_MAGENTA", Color { r: 0.9607843137254902, g: 0.01568627450980392, b: 0.788235294117647, a: 1.0 })?;
    m.add("GREYBLUE", Color { r: 0.4666666666666667, g: 0.6313725490196078, b: 0.7098039215686275, a: 1.0 })?;
    m.add("PURPLEY", Color { r: 0.5294117647058824, g: 0.33725490196078434, b: 0.8941176470588236, a: 1.0 })?;
    m.add("BABY_SHIT_GREEN", Color { r: 0.5333333333333333, g: 0.592156862745098, b: 0.09019607843137255, a: 1.0 })?;
    m.add("BROWNISH_PINK", Color { r: 0.7607843137254902, g: 0.49411764705882355, b: 0.4745098039215686, a: 1.0 })?;
    m.add("DARK_AQUAMARINE", Color { r: 0.00392156862745098, g: 0.45098039215686275, b: 0.44313725490196076, a: 1.0 })?;
    m.add("DIARRHEA", Color { r: 0.6235294117647059, g: 0.5137254901960784, b: 0.011764705882352941, a: 1.0 })?;
    m.add("LIGHT_MUSTARD", Color { r: 0.9686274509803922, g: 0.8352941176470589, b: 0.3764705882352941, a: 1.0 })?;
    m.add("PALE_SKY_BLUE", Color { r: 0.7411764705882353, g: 0.9647058823529412, b: 0.996078431372549, a: 1.0 })?;
    m.add("TURTLE_GREEN", Color { r: 0.4588235294117647, g: 0.7215686274509804, b: 0.30980392156862746, a: 1.0 })?;
    m.add("BRIGHT_OLIVE", Color { r: 0.611764705882353, g: 0.7333333333333333, b: 0.01568627450980392, a: 1.0 })?;
    m.add("DARK_GREY_BLUE", Color { r: 0.1607843137254902, g: 0.27450980392156865, b: 0.3568627450980392, a: 1.0 })?;
    m.add("GREENY_BROWN", Color { r: 0.4117647058823529, g: 0.3764705882352941, b: 0.023529411764705882, a: 1.0 })?;
    m.add("LEMON_GREEN", Color { r: 0.6784313725490196, g: 0.9725490196078431, b: 0.00784313725490196, a: 1.0 })?;
    m.add("LIGHT_PERIWINKLE", Color { r: 0.7568627450980392, g: 0.7764705882352941, b: 0.9882352941176471, a: 1.0 })?;
    m.add("SEAWEED_GREEN", Color { r: 0.20784313725490197, g: 0.6784313725490196, b: 0.4196078431372549, a: 1.0 })?;
    m.add("SUNSHINE_YELLOW", Color { r: 1.0, g: 0.9921568627450981, b: 0.21568627450980393, a: 1.0 })?;
    m.add("UGLY_PURPLE", Color { r: 0.6431372549019608, g: 0.25882352941176473, b: 0.6274509803921569, a: 1.0 })?;
    m.add("MEDIUM_PINK", Color { r: 0.9529411764705882, g: 0.3803921568627451, b: 0.5882352941176471, a: 1.0 })?;
    m.add("PUKE_BROWN", Color { r: 0.5803921568627451, g: 0.4666666666666667, b: 0.023529411764705882, a: 1.0 })?;
    m.add("VERY_LIGHT_PINK", Color { r: 1.0, g: 0.9568627450980393, b: 0.9490196078431372, a: 1.0 })?;
    m.add("VIRIDIAN", Color { r: 0.11764705882352941, g: 0.5686274509803921, b: 0.403921568627451, a: 1.0 })?;
    m.add("BILE", Color { r: 0.7098039215686275, g: 0.7647058823529411, b: 0.023529411764705882, a: 1.0 })?;
    m.add("FADED_YELLOW", Color { r: 0.996078431372549, g: 1.0, b: 0.4980392156862745, a: 1.0 })?;
    m.add("VERY_PALE_GREEN", Color { r: 0.8117647058823529, g: 0.9921568627450981, b: 0.7372549019607844, a: 1.0 })?;
    m.add("VIBRANT_GREEN", Color { r: 0.0392156862745098, g: 0.8666666666666667, b: 0.03137254901960784, a: 1.0 })?;
    m.add("BRIGHT_LIME", Color { r: 0.5294117647058824, g: 0.9921568627450981, b: 0.0196078431372549, a: 1.0 })?;
    m.add("SPEARMINT", Color { r: 0.11764705882352941, g: 0.9725490196078431, b: 0.4627450980392157, a: 1.0 })?;
    m.add("LIGHT_AQUAMARINE", Color { r: 0.4823529411764706, g: 0.9921568627450981, b: 0.7803921568627451, a: 1.0 })?;
    m.add("LIGHT_SAGE", Color { r: 0.7372549019607844, g: 0.9254901960784314, b: 0.6745098039215687, a: 1.0 })?;
    m.add("YELLOWGREEN", Color { r: 0.7333333333333333, g: 0.9764705882352941, b: 0.058823529411764705, a: 1.0 })?;
    m.add("BABY_POO", Color { r: 0.6705882352941176, g: 0.5647058823529412, b: 0.01568627450980392, a: 1.0 })?;
    m.add("DARK_SEAFOAM", Color { r: 0.12156862745098039, g: 0.7098039215686275, b: 0.47843137254901963, a: 1.0 })?;
    m.add("DEEP_TEAL", Color { r: 0.0, g: 0.3333333333333333, b: 0.35294117647058826, a: 1.0 })?;
    m.add("HEATHER", Color { r: 0.6431372549019608, g: 0.5176470588235295, b: 0.6745098039215687, a: 1.0 })?;
    m.add("RUST_ORANGE", Color { r: 0.7686274509803922, g: 0.3333333333333333, b: 0.03137254901960784, a: 1.0 })?;
    m.add("DIRTY_BLUE", Color { r: 0.24705882352941178, g: 0.5098039215686274, b: 0.615686274509804, a: 1.0 })?;
    m.add("FERN_GREEN", Color { r: 0.32941176470588235, g: 0.5529411764705883, b: 0.26666666666666666, a: 1.0 })?;
    m.add("BRIGHT_LILAC", Color { r: 0.788235294117647, g: 0.3686274509803922, b: 0.984313725490196, a: 1.0 })?;
    m.add("WEIRD_GREEN", Color { r: 0.22745098039215686, g: 0.8980392156862745, b: 0.4980392156862745, a: 1.0 })?;
    m.add("PEACOCK_BLUE", Color { r: 0.00392156862745098, g: 0.403921568627451, b: 0.5843137254901961, a: 1.0 })?;
    m.add("AVOCADO_GREEN", Color { r: 0.5294117647058824, g: 0.6627450980392157, b: 0.13333333333333333, a: 1.0 })?;
    m.add("FADED_ORANGE", Color { r: 0.9411764705882353, g: 0.5803921568627451, b: 0.30196078431372547, a: 1.0 })?;
    m.add("GRAPE_PURPLE", Color { r: 0.36470588235294116, g: 0.0784313725490196, b: 0.3176470588235294, a: 1.0 })?;
    m.add("HOT_GREEN", Color { r: 0.1450980392156863, g: 1.0, b: 0.1607843137254902, a: 1.0 })?;
    m.add("LIME_YELLOW", Color { r: 0.8156862745098039, g: 0.996078431372549, b: 0.11372549019607843, a: 1.0 })?;
    m.add("MANGO", Color { r: 1.0, g: 0.6509803921568628, b: 0.16862745098039217, a: 1.0 })?;
    m.add("SHAMROCK", Color { r: 0.00392156862745098, g: 0.7058823529411765, b: 0.2980392156862745, a: 1.0 })?;
    m.add("BUBBLEGUM", Color { r: 1.0, g: 0.4235294117647059, b: 0.7098039215686275, a: 1.0 })?;
    m.add("PURPLISH_BROWN", Color { r: 0.4196078431372549, g: 0.25882352941176473, b: 0.2784313725490196, a: 1.0 })?;
    m.add("VOMIT_YELLOW", Color { r: 0.7803921568627451, g: 0.7568627450980392, b: 0.047058823529411764, a: 1.0 })?;
    m.add("PALE_CYAN", Color { r: 0.7176470588235294, g: 1.0, b: 0.9803921568627451, a: 1.0 })?;
    m.add("KEY_LIME", Color { r: 0.6823529411764706, g: 1.0, b: 0.43137254901960786, a: 1.0 })?;
    m.add("TOMATO_RED", Color { r: 0.9254901960784314, g: 0.17647058823529413, b: 0.00392156862745098, a: 1.0 })?;
    m.add("LIGHTGREEN", Color { r: 0.4627450980392157, g: 1.0, b: 0.4823529411764706, a: 1.0 })?;
    m.add("MERLOT", Color { r: 0.45098039215686275, g: 0.0, b: 0.2235294117647059, a: 1.0 })?;
    m.add("NIGHT_BLUE", Color { r: 0.01568627450980392, g: 0.011764705882352941, b: 0.2823529411764706, a: 1.0 })?;
    m.add("PURPLEISH_PINK", Color { r: 0.8745098039215686, g: 0.3058823529411765, b: 0.7843137254901961, a: 1.0 })?;
    m.add("APPLE", Color { r: 0.43137254901960786, g: 0.796078431372549, b: 0.23529411764705882, a: 1.0 })?;
    m.add("BABY_POOP_GREEN", Color { r: 0.5607843137254902, g: 0.596078431372549, b: 0.0196078431372549, a: 1.0 })?;
    m.add("GREEN_APPLE", Color { r: 0.3686274509803922, g: 0.8627450980392157, b: 0.12156862745098039, a: 1.0 })?;
    m.add("HELIOTROPE", Color { r: 0.8509803921568627, g: 0.30980392156862746, b: 0.9607843137254902, a: 1.0 })?;
    m.add("YELLOW_GREEN", Color { r: 0.7843137254901961, g: 0.9921568627450981, b: 0.23921568627450981, a: 1.0 })?;
    m.add("ALMOST_BLACK", Color { r: 0.027450980392156862, g: 0.050980392156862744, b: 0.050980392156862744, a: 1.0 })?;
    m.add("COOL_BLUE", Color { r: 0.28627450980392155, g: 0.5176470588235295, b: 0.7215686274509804, a: 1.0 })?;
    m.add("LEAFY_GREEN", Color { r: 0.3176470588235294, g: 0.7176470588235294, b: 0.23137254901960785, a: 1.0 })?;
    m.add("MUSTARD_BROWN", Color { r: 0.6745098039215687, g: 0.49411764705882355, b: 0.01568627450980392, a: 1.0 })?;
    m.add("DUSK", Color { r: 0.3058823529411765, g: 0.32941176470588235, b: 0.5058823529411764, a: 1.0 })?;
    m.add("DULL_BROWN", Color { r: 0.5294117647058824, g: 0.43137254901960786, b: 0.29411764705882354, a: 1.0 })?;
    m.add("FROG_GREEN", Color { r: 0.34509803921568627, g: 0.7372549019607844, b: 0.03137254901960784, a: 1.0 })?;
    m.add("VIVID_GREEN", Color { r: 0.1843137254901961, g: 0.9372549019607843, b: 0.06274509803921569, a: 1.0 })?;
    m.add("BRIGHT_LIGHT_GREEN", Color { r: 0.17647058823529413, g: 0.996078431372549, b: 0.32941176470588235, a: 1.0 })?;
    m.add("FLURO_GREEN", Color { r: 0.0392156862745098, g: 1.0, b: 0.00784313725490196, a: 1.0 })?;
    m.add("KIWI", Color { r: 0.611764705882353, g: 0.9372549019607843, b: 0.2627450980392157, a: 1.0 })?;
    m.add("SEAWEED", Color { r: 0.09411764705882353, g: 0.8196078431372549, b: 0.4823529411764706, a: 1.0 })?;
    m.add("NAVY_GREEN", Color { r: 0.20784313725490197, g: 0.3254901960784314, b: 0.0392156862745098, a: 1.0 })?;
    m.add("ULTRAMARINE_BLUE", Color { r: 0.09411764705882353, g: 0.0196078431372549, b: 0.8588235294117647, a: 1.0 })?;
    m.add("IRIS", Color { r: 0.3843137254901961, g: 0.34509803921568627, b: 0.7686274509803922, a: 1.0 })?;
    m.add("PASTEL_ORANGE", Color { r: 1.0, g: 0.5882352941176471, b: 0.30980392156862746, a: 1.0 })?;
    m.add("YELLOWISH_ORANGE", Color { r: 1.0, g: 0.6705882352941176, b: 0.058823529411764705, a: 1.0 })?;
    m.add("PERRYWINKLE", Color { r: 0.5607843137254902, g: 0.5490196078431373, b: 0.9058823529411765, a: 1.0 })?;
    m.add("TEALISH", Color { r: 0.1411764705882353, g: 0.7372549019607844, b: 0.6588235294117647, a: 1.0 })?;
    m.add("DARK_PLUM", Color { r: 0.24705882352941178, g: 0.00392156862745098, b: 0.17254901960784313, a: 1.0 })?;
    m.add("PEAR", Color { r: 0.796078431372549, g: 0.9725490196078431, b: 0.37254901960784315, a: 1.0 })?;
    m.add("PINKISH_ORANGE", Color { r: 1.0, g: 0.4470588235294118, b: 0.2980392156862745, a: 1.0 })?;
    m.add("MIDNIGHT_PURPLE", Color { r: 0.1568627450980392, g: 0.00392156862745098, b: 0.21568627450980393, a: 1.0 })?;
    m.add("LIGHT_URPLE", Color { r: 0.7019607843137254, g: 0.43529411764705883, b: 0.9647058823529412, a: 1.0 })?;
    m.add("DARK_MINT", Color { r: 0.2823529411764706, g: 0.7529411764705882, b: 0.4470588235294118, a: 1.0 })?;
    m.add("GREENISH_TAN", Color { r: 0.7372549019607844, g: 0.796078431372549, b: 0.47843137254901963, a: 1.0 })?;
    m.add("LIGHT_BURGUNDY", Color { r: 0.6588235294117647, g: 0.2549019607843137, b: 0.3568627450980392, a: 1.0 })?;
    m.add("TURQUOISE_BLUE", Color { r: 0.023529411764705882, g: 0.6941176470588235, b: 0.7686274509803922, a: 1.0 })?;
    m.add("UGLY_PINK", Color { r: 0.803921568627451, g: 0.4588235294117647, b: 0.5176470588235295, a: 1.0 })?;
    m.add("SANDY", Color { r: 0.9450980392156862, g: 0.8549019607843137, b: 0.47843137254901963, a: 1.0 })?;
    m.add("ELECTRIC_PINK", Color { r: 1.0, g: 0.01568627450980392, b: 0.5647058823529412, a: 1.0 })?;
    m.add("MUTED_PURPLE", Color { r: 0.5019607843137255, g: 0.3568627450980392, b: 0.5294117647058824, a: 1.0 })?;
    m.add("MID_GREEN", Color { r: 0.3137254901960784, g: 0.6549019607843137, b: 0.2784313725490196, a: 1.0 })?;
    m.add("GREYISH", Color { r: 0.6588235294117647, g: 0.6431372549019608, b: 0.5843137254901961, a: 1.0 })?;
    m.add("NEON_YELLOW", Color { r: 0.8117647058823529, g: 1.0, b: 0.01568627450980392, a: 1.0 })?;
    m.add("BANANA", Color { r: 1.0, g: 1.0, b: 0.49411764705882355, a: 1.0 })?;
    m.add("CARNATION_PINK", Color { r: 1.0, g: 0.4980392156862745, b: 0.6549019607843137, a: 1.0 })?;
    m.add("TOMATO", Color { r: 0.9372549019607843, g: 0.25098039215686274, b: 0.14901960784313725, a: 1.0 })?;
    m.add("SEA", Color { r: 0.23529411764705882, g: 0.6, b: 0.5725490196078431, a: 1.0 })?;
    m.add("MUDDY_BROWN", Color { r: 0.5333333333333333, g: 0.40784313725490196, b: 0.023529411764705882, a: 1.0 })?;
    m.add("TURQUOISE_GREEN", Color { r: 0.01568627450980392, g: 0.9568627450980393, b: 0.5372549019607843, a: 1.0 })?;
    m.add("BUFF", Color { r: 0.996078431372549, g: 0.9647058823529412, b: 0.6196078431372549, a: 1.0 })?;
    m.add("FAWN", Color { r: 0.8117647058823529, g: 0.6862745098039216, b: 0.4823529411764706, a: 1.0 })?;
    m.add("MUTED_BLUE", Color { r: 0.23137254901960785, g: 0.44313725490196076, b: 0.6235294117647059, a: 1.0 })?;
    m.add("PALE_ROSE", Color { r: 0.9921568627450981, g: 0.7568627450980392, b: 0.7725490196078432, a: 1.0 })?;
    m.add("DARK_MINT_GREEN", Color { r: 0.12549019607843137, g: 0.7529411764705882, b: 0.45098039215686275, a: 1.0 })?;
    m.add("AMETHYST", Color { r: 0.6078431372549019, g: 0.37254901960784315, b: 0.7529411764705882, a: 1.0 })?;
    m.add("BLUE_GREEN", Color { r: 0.058823529411764705, g: 0.6078431372549019, b: 0.5568627450980392, a: 1.0 })?;
    m.add("CHESTNUT", Color { r: 0.4549019607843137, g: 0.1568627450980392, b: 0.00784313725490196, a: 1.0 })?;
    m.add("SICK_GREEN", Color { r: 0.615686274509804, g: 0.7254901960784313, b: 0.17254901960784313, a: 1.0 })?;
    m.add("PEA", Color { r: 0.6431372549019608, g: 0.7490196078431373, b: 0.12549019607843137, a: 1.0 })?;
    m.add("RUSTY_ORANGE", Color { r: 0.803921568627451, g: 0.34901960784313724, b: 0.03529411764705882, a: 1.0 })?;
    m.add("STONE", Color { r: 0.6784313725490196, g: 0.6470588235294118, b: 0.5294117647058824, a: 1.0 })?;
    m.add("ROSE_RED", Color { r: 0.7450980392156863, g: 0.00392156862745098, b: 0.23529411764705882, a: 1.0 })?;
    m.add("PALE_AQUA", Color { r: 0.7215686274509804, g: 1.0, b: 0.9215686274509803, a: 1.0 })?;
    m.add("DEEP_ORANGE", Color { r: 0.8627450980392157, g: 0.30196078431372547, b: 0.00392156862745098, a: 1.0 })?;
    m.add("EARTH", Color { r: 0.6352941176470588, g: 0.396078431372549, b: 0.24313725490196078, a: 1.0 })?;
    m.add("MOSSY_GREEN", Color { r: 0.38823529411764707, g: 0.5450980392156862, b: 0.15294117647058825, a: 1.0 })?;
    m.add("GRASSY_GREEN", Color { r: 0.2549019607843137, g: 0.611764705882353, b: 0.011764705882352941, a: 1.0 })?;
    m.add("PALE_LIME_GREEN", Color { r: 0.6941176470588235, g: 1.0, b: 0.396078431372549, a: 1.0 })?;
    m.add("LIGHT_GREY_BLUE", Color { r: 0.615686274509804, g: 0.7372549019607844, b: 0.8313725490196079, a: 1.0 })?;
    m.add("PALE_GREY", Color { r: 0.9921568627450981, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 })?;
    m.add("ASPARAGUS", Color { r: 0.4666666666666667, g: 0.6705882352941176, b: 0.33725490196078434, a: 1.0 })?;
    m.add("BLUEBERRY", Color { r: 0.27450980392156865, g: 0.2549019607843137, b: 0.5882352941176471, a: 1.0 })?;
    m.add("PURPLE_RED", Color { r: 0.6, g: 0.00392156862745098, b: 0.2784313725490196, a: 1.0 })?;
    m.add("PALE_LIME", Color { r: 0.7450980392156863, g: 0.9921568627450981, b: 0.45098039215686275, a: 1.0 })?;
    m.add("GREENISH_TEAL", Color { r: 0.19607843137254902, g: 0.7490196078431373, b: 0.5176470588235295, a: 1.0 })?;
    m.add("CARAMEL", Color { r: 0.6862745098039216, g: 0.43529411764705883, b: 0.03529411764705882, a: 1.0 })?;
    m.add("DEEP_MAGENTA", Color { r: 0.6274509803921569, g: 0.00784313725490196, b: 0.3607843137254902, a: 1.0 })?;
    m.add("LIGHT_PEACH", Color { r: 1.0, g: 0.8470588235294118, b: 0.6941176470588235, a: 1.0 })?;
    m.add("MILK_CHOCOLATE", Color { r: 0.4980392156862745, g: 0.3058823529411765, b: 0.11764705882352941, a: 1.0 })?;
    m.add("OCHER", Color { r: 0.7490196078431373, g: 0.6078431372549019, b: 0.047058823529411764, a: 1.0 })?;
    m.add("OFF_GREEN", Color { r: 0.4196078431372549, g: 0.6392156862745098, b: 0.3254901960784314, a: 1.0 })?;
    m.add("PURPLY_PINK", Color { r: 0.9411764705882353, g: 0.4588235294117647, b: 0.9019607843137255, a: 1.0 })?;
    m.add("LIGHTBLUE", Color { r: 0.4823529411764706, g: 0.7843137254901961, b: 0.9647058823529412, a: 1.0 })?;
    m.add("DUSKY_BLUE", Color { r: 0.2784313725490196, g: 0.37254901960784315, b: 0.5803921568627451, a: 1.0 })?;
    m.add("GOLDEN", Color { r: 0.9607843137254902, g: 0.7490196078431373, b: 0.011764705882352941, a: 1.0 })?;
    m.add("LIGHT_BEIGE", Color { r: 1.0, g: 0.996078431372549, b: 0.7137254901960784, a: 1.0 })?;
    m.add("BUTTER_YELLOW", Color { r: 1.0, g: 0.9921568627450981, b: 0.4549019607843137, a: 1.0 })?;
    m.add("DUSKY_PURPLE", Color { r: 0.5372549019607843, g: 0.3568627450980392, b: 0.4823529411764706, a: 1.0 })?;
    m.add("FRENCH_BLUE", Color { r: 0.2627450980392157, g: 0.4196078431372549, b: 0.6784313725490196, a: 1.0 })?;
    m.add("UGLY_YELLOW", Color { r: 0.8156862745098039, g: 0.7568627450980392, b: 0.00392156862745098, a: 1.0 })?;
    m.add("GREENY_YELLOW", Color { r: 0.7764705882352941, g: 0.9725490196078431, b: 0.03137254901960784, a: 1.0 })?;
    m.add("ORANGISH_RED", Color { r: 0.9568627450980393, g: 0.21176470588235294, b: 0.0196078431372549, a: 1.0 })?;
    m.add("SHAMROCK_GREEN", Color { r: 0.00784313725490196, g: 0.7568627450980392, b: 0.30196078431372547, a: 1.0 })?;
    m.add("ORANGISH_BROWN", Color { r: 0.6980392156862745, g: 0.37254901960784315, b: 0.011764705882352941, a: 1.0 })?;
    m.add("TREE_GREEN", Color { r: 0.16470588235294117, g: 0.49411764705882355, b: 0.09803921568627451, a: 1.0 })?;
    m.add("DEEP_VIOLET", Color { r: 0.28627450980392155, g: 0.023529411764705882, b: 0.2823529411764706, a: 1.0 })?;
    m.add("GUNMETAL", Color { r: 0.3254901960784314, g: 0.3843137254901961, b: 0.403921568627451, a: 1.0 })?;
    m.add("BLUE_PURPLE", Color { r: 0.35294117647058826, g: 0.023529411764705882, b: 0.9372549019607843, a: 1.0 })?;
    m.add("CHERRY", Color { r: 0.8117647058823529, g: 0.00784313725490196, b: 0.20392156862745098, a: 1.0 })?;
    m.add("SANDY_BROWN", Color { r: 0.7686274509803922, g: 0.6509803921568628, b: 0.3803921568627451, a: 1.0 })?;
    m.add("WARM_GREY", Color { r: 0.592156862745098, g: 0.5411764705882353, b: 0.5176470588235295, a: 1.0 })?;
    m.add("DARK_INDIGO", Color { r: 0.12156862745098039, g: 0.03529411764705882, b: 0.32941176470588235, a: 1.0 })?;
    m.add("MIDNIGHT", Color { r: 0.011764705882352941, g: 0.00392156862745098, b: 0.17647058823529413, a: 1.0 })?;
    m.add("BLUEY_GREEN", Color { r: 0.16862745098039217, g: 0.6941176470588235, b: 0.4745098039215686, a: 1.0 })?;
    m.add("GREY_PINK", Color { r: 0.7647058823529411, g: 0.5647058823529412, b: 0.6078431372549019, a: 1.0 })?;
    m.add("SOFT_PURPLE", Color { r: 0.6509803921568628, g: 0.43529411764705883, b: 0.7098039215686275, a: 1.0 })?;
    m.add("BLOOD", Color { r: 0.4666666666666667, g: 0.0, b: 0.00392156862745098, a: 1.0 })?;
    m.add("BROWN_RED", Color { r: 0.5725490196078431, g: 0.16862745098039217, b: 0.0196078431372549, a: 1.0 })?;
    m.add("MEDIUM_GREY", Color { r: 0.49019607843137253, g: 0.4980392156862745, b: 0.48627450980392156, a: 1.0 })?;
    m.add("BERRY", Color { r: 0.6, g: 0.058823529411764705, b: 0.29411764705882354, a: 1.0 })?;
    m.add("POO", Color { r: 0.5607843137254902, g: 0.45098039215686275, b: 0.011764705882352941, a: 1.0 })?;
    m.add("PURPLEY_PINK", Color { r: 0.7843137254901961, g: 0.23529411764705882, b: 0.7254901960784313, a: 1.0 })?;
    m.add("LIGHT_SALMON", Color { r: 0.996078431372549, g: 0.6627450980392157, b: 0.5764705882352941, a: 1.0 })?;
    m.add("SNOT", Color { r: 0.6745098039215687, g: 0.7333333333333333, b: 0.050980392156862744, a: 1.0 })?;
    m.add("EASTER_PURPLE", Color { r: 0.7529411764705882, g: 0.44313725490196076, b: 0.996078431372549, a: 1.0 })?;
    m.add("LIGHT_YELLOW_GREEN", Color { r: 0.8, g: 0.9921568627450981, b: 0.4980392156862745, a: 1.0 })?;
    m.add("DARK_NAVY_BLUE", Color { r: 0.0, g: 0.00784313725490196, b: 0.1803921568627451, a: 1.0 })?;
    m.add("DRAB", Color { r: 0.5098039215686274, g: 0.5137254901960784, b: 0.26666666666666666, a: 1.0 })?;
    m.add("LIGHT_ROSE", Color { r: 1.0, g: 0.7725490196078432, b: 0.796078431372549, a: 1.0 })?;
    m.add("ROUGE", Color { r: 0.6705882352941176, g: 0.07058823529411765, b: 0.2235294117647059, a: 1.0 })?;
    m.add("PURPLISH_RED", Color { r: 0.6901960784313725, g: 0.0196078431372549, b: 0.29411764705882354, a: 1.0 })?;
    m.add("SLIME_GREEN", Color { r: 0.6, g: 0.8, b: 0.01568627450980392, a: 1.0 })?;
    m.add("BABY_POOP", Color { r: 0.5764705882352941, g: 0.48627450980392156, b: 0.0, a: 1.0 })?;
    m.add("IRISH_GREEN", Color { r: 0.00392156862745098, g: 0.5843137254901961, b: 0.1607843137254902, a: 1.0 })?;
    m.add("PINK_PURPLE", Color { r: 0.9372549019607843, g: 0.11372549019607843, b: 0.9058823529411765, a: 1.0 })?;
    m.add("DARK_NAVY", Color { r: 0.0, g: 0.01568627450980392, b: 0.20784313725490197, a: 1.0 })?;
    m.add("GREENY_BLUE", Color { r: 0.25882352941176473, g: 0.7019607843137254, b: 0.5843137254901961, a: 1.0 })?;
    m.add("LIGHT_PLUM", Color { r: 0.615686274509804, g: 0.3411764705882353, b: 0.5137254901960784, a: 1.0 })?;
    m.add("PINKISH_GREY", Color { r: 0.7843137254901961, g: 0.6745098039215687, b: 0.6627450980392157, a: 1.0 })?;
    m.add("DIRTY_ORANGE", Color { r: 0.7843137254901961, g: 0.4627450980392157, b: 0.023529411764705882, a: 1.0 })?;
    m.add("RUST_RED", Color { r: 0.6666666666666666, g: 0.15294117647058825, b: 0.01568627450980392, a: 1.0 })?;
    m.add("PALE_LILAC", Color { r: 0.8941176470588236, g: 0.796078431372549, b: 1.0, a: 1.0 })?;
    m.add("ORANGEY_RED", Color { r: 0.9803921568627451, g: 0.25882352941176473, b: 0.1411764705882353, a: 1.0 })?;
    m.add("PRIMARY_BLUE", Color { r: 0.03137254901960784, g: 0.01568627450980392, b: 0.9764705882352941, a: 1.0 })?;
    m.add("KERMIT_GREEN", Color { r: 0.3607843137254902, g: 0.6980392156862745, b: 0.0, a: 1.0 })?;
    m.add("BROWNISH_PURPLE", Color { r: 0.4627450980392157, g: 0.25882352941176473, b: 0.3058823529411765, a: 1.0 })?;
    m.add("MURKY_GREEN", Color { r: 0.4235294117647059, g: 0.47843137254901963, b: 0.054901960784313725, a: 1.0 })?;
    m.add("WHEAT", Color { r: 0.984313725490196, g: 0.8666666666666667, b: 0.49411764705882355, a: 1.0 })?;
    m.add("VERY_DARK_PURPLE", Color { r: 0.16470588235294117, g: 0.00392156862745098, b: 0.20392156862745098, a: 1.0 })?;
    m.add("BOTTLE_GREEN", Color { r: 0.01568627450980392, g: 0.2901960784313726, b: 0.0196078431372549, a: 1.0 })?;
    m.add("WATERMELON", Color { r: 0.9921568627450981, g: 0.27450980392156865, b: 0.34901960784313724, a: 1.0 })?;
    m.add("DEEP_SKY_BLUE", Color { r: 0.050980392156862744, g: 0.4588235294117647, b: 0.9725490196078431, a: 1.0 })?;
    m.add("FIRE_ENGINE_RED", Color { r: 0.996078431372549, g: 0.0, b: 0.00784313725490196, a: 1.0 })?;
    m.add("YELLOW_OCHRE", Color { r: 0.796078431372549, g: 0.615686274509804, b: 0.023529411764705882, a: 1.0 })?;
    m.add("PUMPKIN_ORANGE", Color { r: 0.984313725490196, g: 0.49019607843137253, b: 0.027450980392156862, a: 1.0 })?;
    m.add("PALE_OLIVE", Color { r: 0.7254901960784313, g: 0.8, b: 0.5058823529411764, a: 1.0 })?;
    m.add("LIGHT_LILAC", Color { r: 0.9294117647058824, g: 0.7843137254901961, b: 1.0, a: 1.0 })?;
    m.add("LIGHTISH_GREEN", Color { r: 0.3803921568627451, g: 0.8823529411764706, b: 0.3764705882352941, a: 1.0 })?;
    m.add("CAROLINA_BLUE", Color { r: 0.5411764705882353, g: 0.7215686274509804, b: 0.996078431372549, a: 1.0 })?;
    m.add("MULBERRY", Color { r: 0.5725490196078431, g: 0.0392156862745098, b: 0.3058823529411765, a: 1.0 })?;
    m.add("SHOCKING_PINK", Color { r: 0.996078431372549, g: 0.00784313725490196, b: 0.6352941176470588, a: 1.0 })?;
    m.add("AUBURN", Color { r: 0.6039215686274509, g: 0.18823529411764706, b: 0.00392156862745098, a: 1.0 })?;
    m.add("BRIGHT_LIME_GREEN", Color { r: 0.396078431372549, g: 0.996078431372549, b: 0.03137254901960784, a: 1.0 })?;
    m.add("CELADON", Color { r: 0.7450980392156863, g: 0.9921568627450981, b: 0.7176470588235294, a: 1.0 })?;
    m.add("PINKISH_BROWN", Color { r: 0.6941176470588235, g: 0.4470588235294118, b: 0.3803921568627451, a: 1.0 })?;
    m.add("POO_BROWN", Color { r: 0.5333333333333333, g: 0.37254901960784315, b: 0.00392156862745098, a: 1.0 })?;
    m.add("BRIGHT_SKY_BLUE", Color { r: 0.00784313725490196, g: 0.8, b: 0.996078431372549, a: 1.0 })?;
    m.add("CELERY", Color { r: 0.7568627450980392, g: 0.9921568627450981, b: 0.5843137254901961, a: 1.0 })?;
    m.add("DIRT_BROWN", Color { r: 0.5137254901960784, g: 0.396078431372549, b: 0.2235294117647059, a: 1.0 })?;
    m.add("STRAWBERRY", Color { r: 0.984313725490196, g: 0.1607843137254902, b: 0.2627450980392157, a: 1.0 })?;
    m.add("DARK_LIME", Color { r: 0.5176470588235295, g: 0.7176470588235294, b: 0.00392156862745098, a: 1.0 })?;
    m.add("COPPER", Color { r: 0.7137254901960784, g: 0.38823529411764707, b: 0.1450980392156863, a: 1.0 })?;
    m.add("MEDIUM_BROWN", Color { r: 0.4980392156862745, g: 0.3176470588235294, b: 0.07058823529411765, a: 1.0 })?;
    m.add("MUTED_GREEN", Color { r: 0.37254901960784315, g: 0.6274509803921569, b: 0.3215686274509804, a: 1.0 })?;
    m.add("ROBINS_EGG", Color { r: 0.42745098039215684, g: 0.9294117647058824, b: 0.9921568627450981, a: 1.0 })?;
    m.add("BRIGHT_AQUA", Color { r: 0.043137254901960784, g: 0.9764705882352941, b: 0.9176470588235294, a: 1.0 })?;
    m.add("BRIGHT_LAVENDER", Color { r: 0.7803921568627451, g: 0.3764705882352941, b: 1.0, a: 1.0 })?;
    m.add("IVORY", Color { r: 1.0, g: 1.0, b: 0.796078431372549, a: 1.0 })?;
    m.add("VERY_LIGHT_PURPLE", Color { r: 0.9647058823529412, g: 0.807843137254902, b: 0.9882352941176471, a: 1.0 })?;
    m.add("LIGHT_NAVY", Color { r: 0.08235294117647059, g: 0.3137254901960784, b: 0.5176470588235295, a: 1.0 })?;
    m.add("PINK_RED", Color { r: 0.9607843137254902, g: 0.0196078431372549, b: 0.30980392156862746, a: 1.0 })?;
    m.add("OLIVE_BROWN", Color { r: 0.39215686274509803, g: 0.32941176470588235, b: 0.011764705882352941, a: 1.0 })?;
    m.add("POOP_BROWN", Color { r: 0.47843137254901963, g: 0.34901960784313724, b: 0.00392156862745098, a: 1.0 })?;
    m.add("MUSTARD_GREEN", Color { r: 0.6588235294117647, g: 0.7098039215686275, b: 0.01568627450980392, a: 1.0 })?;
    m.add("OCEAN_GREEN", Color { r: 0.23921568627450981, g: 0.6, b: 0.45098039215686275, a: 1.0 })?;
    m.add("VERY_DARK_BLUE", Color { r: 0.0, g: 0.00392156862745098, b: 0.2, a: 1.0 })?;
    m.add("DUSTY_GREEN", Color { r: 0.4627450980392157, g: 0.6627450980392157, b: 0.45098039215686275, a: 1.0 })?;
    m.add("LIGHT_NAVY_BLUE", Color { r: 0.1803921568627451, g: 0.35294117647058826, b: 0.5333333333333333, a: 1.0 })?;
    m.add("MINTY_GREEN", Color { r: 0.043137254901960784, g: 0.9686274509803922, b: 0.49019607843137253, a: 1.0 })?;
    m.add("ADOBE", Color { r: 0.7411764705882353, g: 0.4235294117647059, b: 0.2823529411764706, a: 1.0 })?;
    m.add("BARNEY", Color { r: 0.6745098039215687, g: 0.11372549019607843, b: 0.7215686274509804, a: 1.0 })?;
    m.add("JADE_GREEN", Color { r: 0.16862745098039217, g: 0.6862745098039216, b: 0.41568627450980394, a: 1.0 })?;
    m.add("BRIGHT_LIGHT_BLUE", Color { r: 0.14901960784313725, g: 0.9686274509803922, b: 0.9921568627450981, a: 1.0 })?;
    m.add("LIGHT_LIME", Color { r: 0.6823529411764706, g: 0.9921568627450981, b: 0.4235294117647059, a: 1.0 })?;
    m.add("DARK_KHAKI", Color { r: 0.6078431372549019, g: 0.5607843137254902, b: 0.3333333333333333, a: 1.0 })?;
    m.add("ORANGE_YELLOW", Color { r: 1.0, g: 0.6784313725490196, b: 0.00392156862745098, a: 1.0 })?;
    m.add("OCRE", Color { r: 0.7764705882352941, g: 0.611764705882353, b: 0.01568627450980392, a: 1.0 })?;
    m.add("MAIZE", Color { r: 0.9568627450980393, g: 0.8156862745098039, b: 0.32941176470588235, a: 1.0 })?;
    m.add("FADED_PINK", Color { r: 0.8705882352941177, g: 0.615686274509804, b: 0.6745098039215687, a: 1.0 })?;
    m.add("BRITISH_RACING_GREEN", Color { r: 0.0196078431372549, g: 0.2823529411764706, b: 0.050980392156862744, a: 1.0 })?;
    m.add("SANDSTONE", Color { r: 0.788235294117647, g: 0.6823529411764706, b: 0.4549019607843137, a: 1.0 })?;
    m.add("MUD_BROWN", Color { r: 0.3764705882352941, g: 0.27450980392156865, b: 0.058823529411764705, a: 1.0 })?;
    m.add("LIGHT_SEA_GREEN", Color { r: 0.596078431372549, g: 0.9647058823529412, b: 0.6901960784313725, a: 1.0 })?;
    m.add("ROBIN_EGG_BLUE", Color { r: 0.5411764705882353, g: 0.9450980392156862, b: 0.996078431372549, a: 1.0 })?;
    m.add("AQUA_MARINE", Color { r: 0.1803921568627451, g: 0.9098039215686274, b: 0.7333333333333333, a: 1.0 })?;
    m.add("DARK_SEA_GREEN", Color { r: 0.06666666666666667, g: 0.5294117647058824, b: 0.36470588235294116, a: 1.0 })?;
    m.add("SOFT_PINK", Color { r: 0.9921568627450981, g: 0.6901960784313725, b: 0.7529411764705882, a: 1.0 })?;
    m.add("ORANGEY_BROWN", Color { r: 0.6941176470588235, g: 0.3764705882352941, b: 0.00784313725490196, a: 1.0 })?;
    m.add("CHERRY_RED", Color { r: 0.9686274509803922, g: 0.00784313725490196, b: 0.16470588235294117, a: 1.0 })?;
    m.add("BURNT_YELLOW", Color { r: 0.8352941176470589, g: 0.6705882352941176, b: 0.03529411764705882, a: 1.0 })?;
    m.add("BROWNISH_GREY", Color { r: 0.5254901960784314, g: 0.4666666666666667, b: 0.37254901960784315, a: 1.0 })?;
    m.add("CAMEL", Color { r: 0.7764705882352941, g: 0.6235294117647059, b: 0.34901960784313724, a: 1.0 })?;
    m.add("PURPLISH_GREY", Color { r: 0.47843137254901963, g: 0.40784313725490196, b: 0.4980392156862745, a: 1.0 })?;
    m.add("MARINE", Color { r: 0.01568627450980392, g: 0.1803921568627451, b: 0.3764705882352941, a: 1.0 })?;
    m.add("GREYISH_PINK", Color { r: 0.7843137254901961, g: 0.5529411764705883, b: 0.5803921568627451, a: 1.0 })?;
    m.add("PALE_TURQUOISE", Color { r: 0.6470588235294118, g: 0.984313725490196, b: 0.8352941176470589, a: 1.0 })?;
    m.add("PASTEL_YELLOW", Color { r: 1.0, g: 0.996078431372549, b: 0.44313725490196076, a: 1.0 })?;
    m.add("BLUEY_PURPLE", Color { r: 0.3843137254901961, g: 0.2549019607843137, b: 0.7803921568627451, a: 1.0 })?;
    m.add("CANARY_YELLOW", Color { r: 1.0, g: 0.996078431372549, b: 0.25098039215686274, a: 1.0 })?;
    m.add("FADED_RED", Color { r: 0.8274509803921568, g: 0.28627450980392155, b: 0.3058823529411765, a: 1.0 })?;
    m.add("SEPIA", Color { r: 0.596078431372549, g: 0.3686274509803922, b: 0.16862745098039217, a: 1.0 })?;
    m.add("COFFEE", Color { r: 0.6509803921568628, g: 0.5058823529411764, b: 0.2980392156862745, a: 1.0 })?;
    m.add("BRIGHT_MAGENTA", Color { r: 1.0, g: 0.03137254901960784, b: 0.9098039215686274, a: 1.0 })?;
    m.add("MOCHA", Color { r: 0.615686274509804, g: 0.4627450980392157, b: 0.3176470588235294, a: 1.0 })?;
    m.add("ECRU", Color { r: 0.996078431372549, g: 1.0, b: 0.792156862745098, a: 1.0 })?;
    m.add("PURPLEISH", Color { r: 0.596078431372549, g: 0.33725490196078434, b: 0.5529411764705883, a: 1.0 })?;
    m.add("CRANBERRY", Color { r: 0.6196078431372549, g: 0.0, b: 0.22745098039215686, a: 1.0 })?;
    m.add("DARKISH_GREEN", Color { r: 0.1568627450980392, g: 0.48627450980392156, b: 0.21568627450980393, a: 1.0 })?;
    m.add("BROWN_ORANGE", Color { r: 0.7254901960784313, g: 0.4117647058823529, b: 0.00784313725490196, a: 1.0 })?;
    m.add("DUSKY_ROSE", Color { r: 0.7294117647058823, g: 0.40784313725490196, b: 0.45098039215686275, a: 1.0 })?;
    m.add("MELON", Color { r: 1.0, g: 0.47058823529411764, b: 0.3333333333333333, a: 1.0 })?;
    m.add("SICKLY_GREEN", Color { r: 0.5803921568627451, g: 0.6980392156862745, b: 0.10980392156862745, a: 1.0 })?;
    m.add("SILVER", Color { r: 0.7725490196078432, g: 0.788235294117647, b: 0.7803921568627451, a: 1.0 })?;
    m.add("PURPLY_BLUE", Color { r: 0.4, g: 0.10196078431372549, b: 0.9333333333333333, a: 1.0 })?;
    m.add("PURPLEISH_BLUE", Color { r: 0.3803921568627451, g: 0.25098039215686274, b: 0.9372549019607843, a: 1.0 })?;
    m.add("HOSPITAL_GREEN", Color { r: 0.6078431372549019, g: 0.8980392156862745, b: 0.6666666666666666, a: 1.0 })?;
    m.add("SHIT_BROWN", Color { r: 0.4823529411764706, g: 0.34509803921568627, b: 0.01568627450980392, a: 1.0 })?;
    m.add("MID_BLUE", Color { r: 0.15294117647058825, g: 0.41568627450980394, b: 0.7019607843137254, a: 1.0 })?;
    m.add("AMBER", Color { r: 0.996078431372549, g: 0.7019607843137254, b: 0.03137254901960784, a: 1.0 })?;
    m.add("EASTER_GREEN", Color { r: 0.5490196078431373, g: 0.9921568627450981, b: 0.49411764705882355, a: 1.0 })?;
    m.add("SOFT_BLUE", Color { r: 0.39215686274509803, g: 0.5333333333333333, b: 0.9176470588235294, a: 1.0 })?;
    m.add("CERULEAN_BLUE", Color { r: 0.0196078431372549, g: 0.43137254901960786, b: 0.9333333333333333, a: 1.0 })?;
    m.add("GOLDEN_BROWN", Color { r: 0.6980392156862745, g: 0.47843137254901963, b: 0.00392156862745098, a: 1.0 })?;
    m.add("BRIGHT_TURQUOISE", Color { r: 0.058823529411764705, g: 0.996078431372549, b: 0.9764705882352941, a: 1.0 })?;
    m.add("RED_PINK", Color { r: 0.9803921568627451, g: 0.16470588235294117, b: 0.3333333333333333, a: 1.0 })?;
    m.add("RED_PURPLE", Color { r: 0.5098039215686274, g: 0.027450980392156862, b: 0.2784313725490196, a: 1.0 })?;
    m.add("GREYISH_BROWN", Color { r: 0.47843137254901963, g: 0.41568627450980394, b: 0.30980392156862746, a: 1.0 })?;
    m.add("VERMILLION", Color { r: 0.9568627450980393, g: 0.19607843137254902, b: 0.047058823529411764, a: 1.0 })?;
    m.add("RUSSET", Color { r: 0.6313725490196078, g: 0.2235294117647059, b: 0.0196078431372549, a: 1.0 })?;
    m.add("STEEL_GREY", Color { r: 0.43529411764705883, g: 0.5098039215686274, b: 0.5411764705882353, a: 1.0 })?;
    m.add("LIGHTER_PURPLE", Color { r: 0.6470588235294118, g: 0.35294117647058826, b: 0.9568627450980393, a: 1.0 })?;
    m.add("BRIGHT_VIOLET", Color { r: 0.6784313725490196, g: 0.0392156862745098, b: 0.9921568627450981, a: 1.0 })?;
    m.add("PRUSSIAN_BLUE", Color { r: 0.0, g: 0.27058823529411763, b: 0.4666666666666667, a: 1.0 })?;
    m.add("SLATE_GREEN", Color { r: 0.396078431372549, g: 0.5529411764705883, b: 0.42745098039215684, a: 1.0 })?;
    m.add("DIRTY_PINK", Color { r: 0.792156862745098, g: 0.4823529411764706, b: 0.5019607843137255, a: 1.0 })?;
    m.add("DARK_BLUE_GREEN", Color { r: 0.0, g: 0.3215686274509804, b: 0.28627450980392155, a: 1.0 })?;
    m.add("PINE", Color { r: 0.16862745098039217, g: 0.36470588235294116, b: 0.20392156862745098, a: 1.0 })?;
    m.add("YELLOWY_GREEN", Color { r: 0.7490196078431373, g: 0.9450980392156862, b: 0.1568627450980392, a: 1.0 })?;
    m.add("DARK_GOLD", Color { r: 0.7098039215686275, g: 0.5803921568627451, b: 0.06274509803921569, a: 1.0 })?;
    m.add("BLUISH", Color { r: 0.1607843137254902, g: 0.4627450980392157, b: 0.7333333333333333, a: 1.0 })?;
    m.add("DARKISH_BLUE", Color { r: 0.00392156862745098, g: 0.2549019607843137, b: 0.5098039215686274, a: 1.0 })?;
    m.add("DULL_RED", Color { r: 0.7333333333333333, g: 0.24705882352941178, b: 0.24705882352941178, a: 1.0 })?;
    m.add("PINKY_RED", Color { r: 0.9882352941176471, g: 0.14901960784313725, b: 0.2784313725490196, a: 1.0 })?;
    m.add("BRONZE", Color { r: 0.6588235294117647, g: 0.4745098039215686, b: 0.0, a: 1.0 })?;
    m.add("PALE_TEAL", Color { r: 0.5098039215686274, g: 0.796078431372549, b: 0.6980392156862745, a: 1.0 })?;
    m.add("MILITARY_GREEN", Color { r: 0.4, g: 0.48627450980392156, b: 0.24313725490196078, a: 1.0 })?;
    m.add("BARBIE_PINK", Color { r: 0.996078431372549, g: 0.27450980392156865, b: 0.6470588235294118, a: 1.0 })?;
    m.add("BUBBLEGUM_PINK", Color { r: 0.996078431372549, g: 0.5137254901960784, b: 0.8, a: 1.0 })?;
    m.add("PEA_SOUP_GREEN", Color { r: 0.5803921568627451, g: 0.6509803921568628, b: 0.09019607843137255, a: 1.0 })?;
    m.add("DARK_MUSTARD", Color { r: 0.6588235294117647, g: 0.5372549019607843, b: 0.0196078431372549, a: 1.0 })?;
    m.add("SHIT", Color { r: 0.4980392156862745, g: 0.37254901960784315, b: 0.0, a: 1.0 })?;
    m.add("MEDIUM_PURPLE", Color { r: 0.6196078431372549, g: 0.2627450980392157, b: 0.6352941176470588, a: 1.0 })?;
    m.add("VERY_DARK_GREEN", Color { r: 0.023529411764705882, g: 0.1803921568627451, b: 0.011764705882352941, a: 1.0 })?;
    m.add("DIRT", Color { r: 0.5411764705882353, g: 0.43137254901960786, b: 0.27058823529411763, a: 1.0 })?;
    m.add("DUSKY_PINK", Color { r: 0.8, g: 0.47843137254901963, b: 0.5450980392156862, a: 1.0 })?;
    m.add("RED_VIOLET", Color { r: 0.6196078431372549, g: 0.00392156862745098, b: 0.40784313725490196, a: 1.0 })?;
    m.add("LEMON_YELLOW", Color { r: 0.9921568627450981, g: 1.0, b: 0.2196078431372549, a: 1.0 })?;
    m.add("PISTACHIO", Color { r: 0.7529411764705882, g: 0.9803921568627451, b: 0.5450980392156862, a: 1.0 })?;
    m.add("DULL_YELLOW", Color { r: 0.9333333333333333, g: 0.8627450980392157, b: 0.3568627450980392, a: 1.0 })?;
    m.add("DARK_LIME_GREEN", Color { r: 0.49411764705882355, g: 0.7411764705882353, b: 0.00392156862745098, a: 1.0 })?;
    m.add("DENIM_BLUE", Color { r: 0.23137254901960785, g: 0.3568627450980392, b: 0.5725490196078431, a: 1.0 })?;
    m.add("TEAL_BLUE", Color { r: 0.00392156862745098, g: 0.5333333333333333, b: 0.6235294117647059, a: 1.0 })?;
    m.add("LIGHTISH_BLUE", Color { r: 0.23921568627450981, g: 0.47843137254901963, b: 0.9921568627450981, a: 1.0 })?;
    m.add("PURPLEY_BLUE", Color { r: 0.37254901960784315, g: 0.20392156862745098, b: 0.9058823529411765, a: 1.0 })?;
    m.add("LIGHT_INDIGO", Color { r: 0.42745098039215684, g: 0.35294117647058826, b: 0.8117647058823529, a: 1.0 })?;
    m.add("SWAMP_GREEN", Color { r: 0.4549019607843137, g: 0.5215686274509804, b: 0.0, a: 1.0 })?;
    m.add("BROWN_GREEN", Color { r: 0.4392156862745098, g: 0.4235294117647059, b: 0.06666666666666667, a: 1.0 })?;
    m.add("DARK_MAROON", Color { r: 0.23529411764705882, g: 0.0, b: 0.03137254901960784, a: 1.0 })?;
    m.add("HOT_PURPLE", Color { r: 0.796078431372549, g: 0.0, b: 0.9607843137254902, a: 1.0 })?;
    m.add("DARK_FOREST_GREEN", Color { r: 0.0, g: 0.17647058823529413, b: 0.01568627450980392, a: 1.0 })?;
    m.add("FADED_BLUE", Color { r: 0.396078431372549, g: 0.5490196078431373, b: 0.7333333333333333, a: 1.0 })?;
    m.add("DRAB_GREEN", Color { r: 0.4549019607843137, g: 0.5843137254901961, b: 0.3176470588235294, a: 1.0 })?;
    m.add("LIGHT_LIME_GREEN", Color { r: 0.7254901960784313, g: 1.0, b: 0.4, a: 1.0 })?;
    m.add("SNOT_GREEN", Color { r: 0.615686274509804, g: 0.7568627450980392, b: 0.0, a: 1.0 })?;
    m.add("YELLOWISH", Color { r: 0.9803921568627451, g: 0.9333333333333333, b: 0.4, a: 1.0 })?;
    m.add("LIGHT_BLUE_GREEN", Color { r: 0.49411764705882355, g: 0.984313725490196, b: 0.7019607843137254, a: 1.0 })?;
    m.add("BORDEAUX", Color { r: 0.4823529411764706, g: 0.0, b: 0.17254901960784313, a: 1.0 })?;
    m.add("LIGHT_MAUVE", Color { r: 0.7607843137254902, g: 0.5725490196078431, b: 0.6313725490196078, a: 1.0 })?;
    m.add("OCEAN", Color { r: 0.00392156862745098, g: 0.4823529411764706, b: 0.5725490196078431, a: 1.0 })?;
    m.add("MARIGOLD", Color { r: 0.9882352941176471, g: 0.7529411764705882, b: 0.023529411764705882, a: 1.0 })?;
    m.add("MUDDY_GREEN", Color { r: 0.396078431372549, g: 0.4549019607843137, b: 0.19607843137254902, a: 1.0 })?;
    m.add("DULL_ORANGE", Color { r: 0.8470588235294118, g: 0.5254901960784314, b: 0.23137254901960785, a: 1.0 })?;
    m.add("STEEL", Color { r: 0.45098039215686275, g: 0.5215686274509804, b: 0.5843137254901961, a: 1.0 })?;
    m.add("ELECTRIC_PURPLE", Color { r: 0.6666666666666666, g: 0.13725490196078433, b: 1.0, a: 1.0 })?;
    m.add("FLUORESCENT_GREEN", Color { r: 0.03137254901960784, g: 1.0, b: 0.03137254901960784, a: 1.0 })?;
    m.add("YELLOWISH_BROWN", Color { r: 0.6078431372549019, g: 0.47843137254901963, b: 0.00392156862745098, a: 1.0 })?;
    m.add("BLUSH", Color { r: 0.9490196078431372, g: 0.6196078431372549, b: 0.5568627450980392, a: 1.0 })?;
    m.add("SOFT_GREEN", Color { r: 0.43529411764705883, g: 0.7607843137254902, b: 0.4627450980392157, a: 1.0 })?;
    m.add("BRIGHT_ORANGE", Color { r: 1.0, g: 0.3568627450980392, b: 0.0, a: 1.0 })?;
    m.add("LEMON", Color { r: 0.9921568627450981, g: 1.0, b: 0.3215686274509804, a: 1.0 })?;
    m.add("PURPLE_GREY", Color { r: 0.5254901960784314, g: 0.43529411764705883, b: 0.5215686274509804, a: 1.0 })?;
    m.add("ACID_GREEN", Color { r: 0.5607843137254902, g: 0.996078431372549, b: 0.03529411764705882, a: 1.0 })?;
    m.add("PALE_LAVENDER", Color { r: 0.9333333333333333, g: 0.8117647058823529, b: 0.996078431372549, a: 1.0 })?;
    m.add("VIOLET_BLUE", Color { r: 0.3176470588235294, g: 0.0392156862745098, b: 0.788235294117647, a: 1.0 })?;
    m.add("LIGHT_FOREST_GREEN", Color { r: 0.30980392156862746, g: 0.5686274509803921, b: 0.3254901960784314, a: 1.0 })?;
    m.add("BURNT_RED", Color { r: 0.6235294117647059, g: 0.13725490196078433, b: 0.0196078431372549, a: 1.0 })?;
    m.add("KHAKI_GREEN", Color { r: 0.4470588235294118, g: 0.5254901960784314, b: 0.2235294117647059, a: 1.0 })?;
    m.add("CERISE", Color { r: 0.8705882352941177, g: 0.047058823529411764, b: 0.3843137254901961, a: 1.0 })?;
    m.add("FADED_PURPLE", Color { r: 0.5686274509803921, g: 0.43137254901960786, b: 0.6, a: 1.0 })?;
    m.add("APRICOT", Color { r: 1.0, g: 0.6941176470588235, b: 0.42745098039215684, a: 1.0 })?;
    m.add("DARK_OLIVE_GREEN", Color { r: 0.23529411764705882, g: 0.30196078431372547, b: 0.011764705882352941, a: 1.0 })?;
    m.add("GREY_BROWN", Color { r: 0.4980392156862745, g: 0.4392156862745098, b: 0.3254901960784314, a: 1.0 })?;
    m.add("GREEN_GREY", Color { r: 0.4666666666666667, g: 0.5725490196078431, b: 0.43529411764705883, a: 1.0 })?;
    m.add("TRUE_BLUE", Color { r: 0.00392156862745098, g: 0.058823529411764705, b: 0.8, a: 1.0 })?;
    m.add("PALE_VIOLET", Color { r: 0.807843137254902, g: 0.6823529411764706, b: 0.9803921568627451, a: 1.0 })?;
    m.add("PERIWINKLE_BLUE", Color { r: 0.5607843137254902, g: 0.6, b: 0.984313725490196, a: 1.0 })?;
    m.add("LIGHT_SKY_BLUE", Color { r: 0.7764705882352941, g: 0.9882352941176471, b: 1.0, a: 1.0 })?;
    m.add("BLURPLE", Color { r: 0.3333333333333333, g: 0.2235294117647059, b: 0.8, a: 1.0 })?;
    m.add("GREEN_BROWN", Color { r: 0.32941176470588235, g: 0.3058823529411765, b: 0.011764705882352941, a: 1.0 })?;
    m.add("BLUEGREEN", Color { r: 0.00392156862745098, g: 0.47843137254901963, b: 0.4745098039215686, a: 1.0 })?;
    m.add("BRIGHT_TEAL", Color { r: 0.00392156862745098, g: 0.9764705882352941, b: 0.7764705882352941, a: 1.0 })?;
    m.add("BROWNISH_YELLOW", Color { r: 0.788235294117647, g: 0.6901960784313725, b: 0.011764705882352941, a: 1.0 })?;
    m.add("PEA_SOUP", Color { r: 0.5725490196078431, g: 0.6, b: 0.00392156862745098, a: 1.0 })?;
    m.add("FOREST", Color { r: 0.043137254901960784, g: 0.3333333333333333, b: 0.03529411764705882, a: 1.0 })?;
    m.add("BARNEY_PURPLE", Color { r: 0.6274509803921569, g: 0.01568627450980392, b: 0.596078431372549, a: 1.0 })?;
    m.add("ULTRAMARINE", Color { r: 0.12549019607843137, g: 0.0, b: 0.6941176470588235, a: 1.0 })?;
    m.add("PURPLISH", Color { r: 0.5803921568627451, g: 0.33725490196078434, b: 0.5490196078431373, a: 1.0 })?;
    m.add("PUKE_YELLOW", Color { r: 0.7607843137254902, g: 0.7450980392156863, b: 0.054901960784313725, a: 1.0 })?;
    m.add("BLUISH_GREY", Color { r: 0.4549019607843137, g: 0.5450980392156862, b: 0.592156862745098, a: 1.0 })?;
    m.add("DARK_PERIWINKLE", Color { r: 0.4, g: 0.37254901960784315, b: 0.8196078431372549, a: 1.0 })?;
    m.add("DARK_LILAC", Color { r: 0.611764705882353, g: 0.42745098039215684, b: 0.6470588235294118, a: 1.0 })?;
    m.add("REDDISH", Color { r: 0.7686274509803922, g: 0.25882352941176473, b: 0.25098039215686274, a: 1.0 })?;
    m.add("LIGHT_MAROON", Color { r: 0.6352941176470588, g: 0.2823529411764706, b: 0.3411764705882353, a: 1.0 })?;
    m.add("DUSTY_PURPLE", Color { r: 0.5098039215686274, g: 0.37254901960784315, b: 0.5294117647058824, a: 1.0 })?;
    m.add("TERRA_COTTA", Color { r: 0.788235294117647, g: 0.39215686274509803, b: 0.23137254901960785, a: 1.0 })?;
    m.add("AVOCADO", Color { r: 0.5647058823529412, g: 0.6941176470588235, b: 0.20392156862745098, a: 1.0 })?;
    m.add("MARINE_BLUE", Color { r: 0.00392156862745098, g: 0.2196078431372549, b: 0.41568627450980394, a: 1.0 })?;
    m.add("TEAL_GREEN", Color { r: 0.1450980392156863, g: 0.6392156862745098, b: 0.43529411764705883, a: 1.0 })?;
    m.add("SLATE_GREY", Color { r: 0.34901960784313724, g: 0.396078431372549, b: 0.42745098039215684, a: 1.0 })?;
    m.add("LIGHTER_GREEN", Color { r: 0.4588235294117647, g: 0.9921568627450981, b: 0.38823529411764707, a: 1.0 })?;
    m.add("ELECTRIC_GREEN", Color { r: 0.12941176470588237, g: 0.9882352941176471, b: 0.050980392156862744, a: 1.0 })?;
    m.add("DUSTY_BLUE", Color { r: 0.35294117647058826, g: 0.5254901960784314, b: 0.6784313725490196, a: 1.0 })?;
    m.add("GOLDEN_YELLOW", Color { r: 0.996078431372549, g: 0.7764705882352941, b: 0.08235294117647059, a: 1.0 })?;
    m.add("BRIGHT_YELLOW", Color { r: 1.0, g: 0.9921568627450981, b: 0.00392156862745098, a: 1.0 })?;
    m.add("LIGHT_LAVENDER", Color { r: 0.8745098039215686, g: 0.7725490196078432, b: 0.996078431372549, a: 1.0 })?;
    m.add("UMBER", Color { r: 0.6980392156862745, g: 0.39215686274509803, b: 0.0, a: 1.0 })?;
    m.add("POOP", Color { r: 0.4980392156862745, g: 0.3686274509803922, b: 0.0, a: 1.0 })?;
    m.add("DARK_PEACH", Color { r: 0.8705882352941177, g: 0.49411764705882355, b: 0.36470588235294116, a: 1.0 })?;
    m.add("JUNGLE_GREEN", Color { r: 0.01568627450980392, g: 0.5098039215686274, b: 0.2627450980392157, a: 1.0 })?;
    m.add("EGGSHELL", Color { r: 1.0, g: 1.0, b: 0.8313725490196079, a: 1.0 })?;
    m.add("DENIM", Color { r: 0.23137254901960785, g: 0.38823529411764707, b: 0.5490196078431373, a: 1.0 })?;
    m.add("YELLOW_BROWN", Color { r: 0.7176470588235294, g: 0.5803921568627451, b: 0.0, a: 1.0 })?;
    m.add("DULL_PURPLE", Color { r: 0.5176470588235295, g: 0.34901960784313724, b: 0.49411764705882355, a: 1.0 })?;
    m.add("CHOCOLATE_BROWN", Color { r: 0.2549019607843137, g: 0.09803921568627451, b: 0.0, a: 1.0 })?;
    m.add("WINE_RED", Color { r: 0.4823529411764706, g: 0.011764705882352941, b: 0.13725490196078433, a: 1.0 })?;
    m.add("NEON_BLUE", Color { r: 0.01568627450980392, g: 0.8509803921568627, b: 1.0, a: 1.0 })?;
    m.add("DIRTY_GREEN", Color { r: 0.4, g: 0.49411764705882355, b: 0.17254901960784313, a: 1.0 })?;
    m.add("LIGHT_TAN", Color { r: 0.984313725490196, g: 0.9333333333333333, b: 0.6745098039215687, a: 1.0 })?;
    m.add("ICE_BLUE", Color { r: 0.8431372549019608, g: 1.0, b: 0.996078431372549, a: 1.0 })?;
    m.add("CADET_BLUE", Color { r: 0.3058823529411765, g: 0.4549019607843137, b: 0.5882352941176471, a: 1.0 })?;
    m.add("DARK_MAUVE", Color { r: 0.5294117647058824, g: 0.2980392156862745, b: 0.3843137254901961, a: 1.0 })?;
    m.add("VERY_LIGHT_BLUE", Color { r: 0.8352941176470589, g: 1.0, b: 1.0, a: 1.0 })?;
    m.add("GREY_PURPLE", Color { r: 0.5098039215686274, g: 0.42745098039215684, b: 0.5490196078431373, a: 1.0 })?;
    m.add("PASTEL_PINK", Color { r: 1.0, g: 0.7294117647058823, b: 0.803921568627451, a: 1.0 })?;
    m.add("VERY_LIGHT_GREEN", Color { r: 0.8196078431372549, g: 1.0, b: 0.7411764705882353, a: 1.0 })?;
    m.add("DARK_SKY_BLUE", Color { r: 0.26666666666666666, g: 0.5568627450980392, b: 0.8941176470588236, a: 1.0 })?;
    m.add("EVERGREEN", Color { r: 0.0196078431372549, g: 0.2784313725490196, b: 0.16470588235294117, a: 1.0 })?;
    m.add("DULL_PINK", Color { r: 0.8352941176470589, g: 0.5254901960784314, b: 0.615686274509804, a: 1.0 })?;
    m.add("AUBERGINE", Color { r: 0.23921568627450981, g: 0.027450980392156862, b: 0.20392156862745098, a: 1.0 })?;
    m.add("MAHOGANY", Color { r: 0.2901960784313726, g: 0.00392156862745098, b: 0.0, a: 1.0 })?;
    m.add("REDDISH_ORANGE", Color { r: 0.9725490196078431, g: 0.2823529411764706, b: 0.10980392156862745, a: 1.0 })?;
    m.add("DEEP_GREEN", Color { r: 0.00784313725490196, g: 0.34901960784313724, b: 0.058823529411764705, a: 1.0 })?;
    m.add("VOMIT_GREEN", Color { r: 0.5372549019607843, g: 0.6352941176470588, b: 0.011764705882352941, a: 1.0 })?;
    m.add("PURPLE_PINK", Color { r: 0.8784313725490196, g: 0.24705882352941178, b: 0.8470588235294118, a: 1.0 })?;
    m.add("DUSTY_PINK", Color { r: 0.8352941176470589, g: 0.5411764705882353, b: 0.5803921568627451, a: 1.0 })?;
    m.add("FADED_GREEN", Color { r: 0.4823529411764706, g: 0.6980392156862745, b: 0.4549019607843137, a: 1.0 })?;
    m.add("CAMO_GREEN", Color { r: 0.3215686274509804, g: 0.396078431372549, b: 0.1450980392156863, a: 1.0 })?;
    m.add("PINKY_PURPLE", Color { r: 0.788235294117647, g: 0.2980392156862745, b: 0.7450980392156863, a: 1.0 })?;
    m.add("PINK_PURPLE", Color { r: 0.8588235294117647, g: 0.29411764705882354, b: 0.8549019607843137, a: 1.0 })?;
    m.add("BROWNISH_RED", Color { r: 0.6196078431372549, g: 0.21176470588235294, b: 0.13725490196078433, a: 1.0 })?;
    m.add("DARK_ROSE", Color { r: 0.7098039215686275, g: 0.2823529411764706, b: 0.36470588235294116, a: 1.0 })?;
    m.add("MUD", Color { r: 0.45098039215686275, g: 0.3607843137254902, b: 0.07058823529411765, a: 1.0 })?;
    m.add("BROWNISH", Color { r: 0.611764705882353, g: 0.42745098039215684, b: 0.3411764705882353, a: 1.0 })?;
    m.add("EMERALD_GREEN", Color { r: 0.00784313725490196, g: 0.5607843137254902, b: 0.11764705882352941, a: 1.0 })?;
    m.add("PALE_BROWN", Color { r: 0.6941176470588235, g: 0.5686274509803921, b: 0.43137254901960786, a: 1.0 })?;
    m.add("DULL_BLUE", Color { r: 0.28627450980392155, g: 0.4588235294117647, b: 0.611764705882353, a: 1.0 })?;
    m.add("BURNT_UMBER", Color { r: 0.6274509803921569, g: 0.27058823529411763, b: 0.054901960784313725, a: 1.0 })?;
    m.add("MEDIUM_GREEN", Color { r: 0.2235294117647059, g: 0.6784313725490196, b: 0.2823529411764706, a: 1.0 })?;
    m.add("CLAY", Color { r: 0.7137254901960784, g: 0.41568627450980394, b: 0.3137254901960784, a: 1.0 })?;
    m.add("LIGHT_AQUA", Color { r: 0.5490196078431373, g: 1.0, b: 0.8588235294117647, a: 1.0 })?;
    m.add("LIGHT_OLIVE_GREEN", Color { r: 0.6431372549019608, g: 0.7450980392156863, b: 0.3607843137254902, a: 1.0 })?;
    m.add("BROWNISH_ORANGE", Color { r: 0.796078431372549, g: 0.4666666666666667, b: 0.13725490196078433, a: 1.0 })?;
    m.add("DARK_AQUA", Color { r: 0.0196078431372549, g: 0.4117647058823529, b: 0.4196078431372549, a: 1.0 })?;
    m.add("PURPLISH_PINK", Color { r: 0.807843137254902, g: 0.36470588235294116, b: 0.6823529411764706, a: 1.0 })?;
    m.add("DARK_SALMON", Color { r: 0.7843137254901961, g: 0.35294117647058826, b: 0.3254901960784314, a: 1.0 })?;
    m.add("GREENISH_GREY", Color { r: 0.5882352941176471, g: 0.6823529411764706, b: 0.5529411764705883, a: 1.0 })?;
    m.add("JADE", Color { r: 0.12156862745098039, g: 0.6549019607843137, b: 0.4549019607843137, a: 1.0 })?;
    m.add("UGLY_GREEN", Color { r: 0.47843137254901963, g: 0.592156862745098, b: 0.011764705882352941, a: 1.0 })?;
    m.add("DARK_BEIGE", Color { r: 0.6745098039215687, g: 0.5764705882352941, b: 0.3843137254901961, a: 1.0 })?;
    m.add("EMERALD", Color { r: 0.00392156862745098, g: 0.6274509803921569, b: 0.28627450980392155, a: 1.0 })?;
    m.add("PALE_RED", Color { r: 0.8509803921568627, g: 0.32941176470588235, b: 0.30196078431372547, a: 1.0 })?;
    m.add("LIGHT_MAGENTA", Color { r: 0.9803921568627451, g: 0.37254901960784315, b: 0.9686274509803922, a: 1.0 })?;
    m.add("SKY", Color { r: 0.5098039215686274, g: 0.792156862745098, b: 0.9882352941176471, a: 1.0 })?;
    m.add("LIGHT_CYAN", Color { r: 0.6745098039215687, g: 1.0, b: 0.9882352941176471, a: 1.0 })?;
    m.add("YELLOW_ORANGE", Color { r: 0.9882352941176471, g: 0.6901960784313725, b: 0.00392156862745098, a: 1.0 })?;
    m.add("REDDISH_PURPLE", Color { r: 0.5686274509803921, g: 0.03529411764705882, b: 0.3176470588235294, a: 1.0 })?;
    m.add("REDDISH_PINK", Color { r: 0.996078431372549, g: 0.17254901960784313, b: 0.32941176470588235, a: 1.0 })?;
    m.add("ORCHID", Color { r: 0.7843137254901961, g: 0.4588235294117647, b: 0.7686274509803922, a: 1.0 })?;
    m.add("DIRTY_YELLOW", Color { r: 0.803921568627451, g: 0.7725490196078432, b: 0.0392156862745098, a: 1.0 })?;
    m.add("ORANGE_RED", Color { r: 0.9921568627450981, g: 0.2549019607843137, b: 0.11764705882352941, a: 1.0 })?;
    m.add("DEEP_RED", Color { r: 0.6039215686274509, g: 0.00784313725490196, b: 0.0, a: 1.0 })?;
    m.add("ORANGE_BROWN", Color { r: 0.7450980392156863, g: 0.39215686274509803, b: 0.0, a: 1.0 })?;
    m.add("COBALT_BLUE", Color { r: 0.011764705882352941, g: 0.0392156862745098, b: 0.6549019607843137, a: 1.0 })?;
    m.add("NEON_PINK", Color { r: 0.996078431372549, g: 0.00392156862745098, b: 0.6039215686274509, a: 1.0 })?;
    m.add("ROSE_PINK", Color { r: 0.9686274509803922, g: 0.5294117647058824, b: 0.6039215686274509, a: 1.0 })?;
    m.add("GREYISH_PURPLE", Color { r: 0.5333333333333333, g: 0.44313725490196076, b: 0.5686274509803921, a: 1.0 })?;
    m.add("RASPBERRY", Color { r: 0.6901960784313725, g: 0.00392156862745098, b: 0.28627450980392155, a: 1.0 })?;
    m.add("AQUA_GREEN", Color { r: 0.07058823529411765, g: 0.8823529411764706, b: 0.5764705882352941, a: 1.0 })?;
    m.add("SALMON_PINK", Color { r: 0.996078431372549, g: 0.4823529411764706, b: 0.48627450980392156, a: 1.0 })?;
    m.add("TANGERINE", Color { r: 1.0, g: 0.5803921568627451, b: 0.03137254901960784, a: 1.0 })?;
    m.add("BROWNISH_GREEN", Color { r: 0.41568627450980394, g: 0.43137254901960786, b: 0.03529411764705882, a: 1.0 })?;
    m.add("RED_BROWN", Color { r: 0.5450980392156862, g: 0.1803921568627451, b: 0.08627450980392157, a: 1.0 })?;
    m.add("GREENISH_BROWN", Color { r: 0.4117647058823529, g: 0.3803921568627451, b: 0.07058823529411765, a: 1.0 })?;
    m.add("PUMPKIN", Color { r: 0.8823529411764706, g: 0.4666666666666667, b: 0.00392156862745098, a: 1.0 })?;
    m.add("PINE_GREEN", Color { r: 0.0392156862745098, g: 0.2823529411764706, b: 0.11764705882352941, a: 1.0 })?;
    m.add("CHARCOAL", Color { r: 0.20392156862745098, g: 0.2196078431372549, b: 0.21568627450980393, a: 1.0 })?;
    m.add("BABY_PINK", Color { r: 1.0, g: 0.7176470588235294, b: 0.807843137254902, a: 1.0 })?;
    m.add("CORNFLOWER", Color { r: 0.41568627450980394, g: 0.4745098039215686, b: 0.9686274509803922, a: 1.0 })?;
    m.add("BLUE_VIOLET", Color { r: 0.36470588235294116, g: 0.023529411764705882, b: 0.9137254901960784, a: 1.0 })?;
    m.add("CHOCOLATE", Color { r: 0.23921568627450981, g: 0.10980392156862745, b: 0.00784313725490196, a: 1.0 })?;
    m.add("GREYISH_GREEN", Color { r: 0.5098039215686274, g: 0.6509803921568628, b: 0.49019607843137253, a: 1.0 })?;
    m.add("SCARLET", Color { r: 0.7450980392156863, g: 0.00392156862745098, b: 0.09803921568627451, a: 1.0 })?;
    m.add("GREEN_YELLOW", Color { r: 0.788235294117647, g: 1.0, b: 0.15294117647058825, a: 1.0 })?;
    m.add("DARK_OLIVE", Color { r: 0.21568627450980393, g: 0.24313725490196078, b: 0.00784313725490196, a: 1.0 })?;
    m.add("SIENNA", Color { r: 0.6627450980392157, g: 0.33725490196078434, b: 0.11764705882352941, a: 1.0 })?;
    m.add("PASTEL_PURPLE", Color { r: 0.792156862745098, g: 0.6274509803921569, b: 1.0, a: 1.0 })?;
    m.add("TERRACOTTA", Color { r: 0.792156862745098, g: 0.4, b: 0.2549019607843137, a: 1.0 })?;
    m.add("AQUA_BLUE", Color { r: 0.00784313725490196, g: 0.8470588235294118, b: 0.9137254901960784, a: 1.0 })?;
    m.add("SAGE_GREEN", Color { r: 0.5333333333333333, g: 0.7019607843137254, b: 0.47058823529411764, a: 1.0 })?;
    m.add("BLOOD_RED", Color { r: 0.596078431372549, g: 0.0, b: 0.00784313725490196, a: 1.0 })?;
    m.add("DEEP_PINK", Color { r: 0.796078431372549, g: 0.00392156862745098, b: 0.3843137254901961, a: 1.0 })?;
    m.add("GRASS", Color { r: 0.3607843137254902, g: 0.6745098039215687, b: 0.17647058823529413, a: 1.0 })?;
    m.add("MOSS", Color { r: 0.4627450980392157, g: 0.6, b: 0.34509803921568627, a: 1.0 })?;
    m.add("PASTEL_BLUE", Color { r: 0.6352941176470588, g: 0.7490196078431373, b: 0.996078431372549, a: 1.0 })?;
    m.add("BLUISH_GREEN", Color { r: 0.06274509803921569, g: 0.6509803921568628, b: 0.4549019607843137, a: 1.0 })?;
    m.add("GREEN_BLUE", Color { r: 0.023529411764705882, g: 0.7058823529411765, b: 0.5450980392156862, a: 1.0 })?;
    m.add("DARK_TAN", Color { r: 0.6862745098039216, g: 0.5333333333333333, b: 0.2901960784313726, a: 1.0 })?;
    m.add("GREENISH_BLUE", Color { r: 0.043137254901960784, g: 0.5450980392156862, b: 0.5294117647058824, a: 1.0 })?;
    m.add("PALE_ORANGE", Color { r: 1.0, g: 0.6549019607843137, b: 0.33725490196078434, a: 1.0 })?;
    m.add("VOMIT", Color { r: 0.6352941176470588, g: 0.6431372549019608, b: 0.08235294117647059, a: 1.0 })?;
    m.add("FORREST_GREEN", Color { r: 0.08235294117647059, g: 0.26666666666666666, b: 0.023529411764705882, a: 1.0 })?;
    m.add("DARK_LAVENDER", Color { r: 0.5215686274509804, g: 0.403921568627451, b: 0.596078431372549, a: 1.0 })?;
    m.add("DARK_VIOLET", Color { r: 0.20392156862745098, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 })?;
    m.add("PURPLE_BLUE", Color { r: 0.38823529411764707, g: 0.17647058823529413, b: 0.9137254901960784, a: 1.0 })?;
    m.add("DARK_CYAN", Color { r: 0.0392156862745098, g: 0.5333333333333333, b: 0.5411764705882353, a: 1.0 })?;
    m.add("OLIVE_DRAB", Color { r: 0.43529411764705883, g: 0.4627450980392157, b: 0.19607843137254902, a: 1.0 })?;
    m.add("PINKISH", Color { r: 0.8313725490196079, g: 0.41568627450980394, b: 0.49411764705882355, a: 1.0 })?;
    m.add("COBALT", Color { r: 0.11764705882352941, g: 0.2823529411764706, b: 0.5607843137254902, a: 1.0 })?;
    m.add("NEON_PURPLE", Color { r: 0.7372549019607844, g: 0.07450980392156863, b: 0.996078431372549, a: 1.0 })?;
    m.add("LIGHT_TURQUOISE", Color { r: 0.49411764705882355, g: 0.9568627450980393, b: 0.8, a: 1.0 })?;
    m.add("APPLE_GREEN", Color { r: 0.4627450980392157, g: 0.803921568627451, b: 0.14901960784313725, a: 1.0 })?;
    m.add("DULL_GREEN", Color { r: 0.4549019607843137, g: 0.6509803921568628, b: 0.3843137254901961, a: 1.0 })?;
    m.add("WINE", Color { r: 0.5019607843137255, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 })?;
    m.add("POWDER_BLUE", Color { r: 0.6941176470588235, g: 0.8196078431372549, b: 0.9882352941176471, a: 1.0 })?;
    m.add("OFF_WHITE", Color { r: 1.0, g: 1.0, b: 0.8941176470588236, a: 1.0 })?;
    m.add("ELECTRIC_BLUE", Color { r: 0.023529411764705882, g: 0.3215686274509804, b: 1.0, a: 1.0 })?;
    m.add("DARK_TURQUOISE", Color { r: 0.01568627450980392, g: 0.3607843137254902, b: 0.35294117647058826, a: 1.0 })?;
    m.add("BLUE_PURPLE", Color { r: 0.3411764705882353, g: 0.1607843137254902, b: 0.807843137254902, a: 1.0 })?;
    m.add("AZURE", Color { r: 0.023529411764705882, g: 0.6039215686274509, b: 0.9529411764705882, a: 1.0 })?;
    m.add("BRIGHT_RED", Color { r: 1.0, g: 0.0, b: 0.050980392156862744, a: 1.0 })?;
    m.add("PINKISH_RED", Color { r: 0.9450980392156862, g: 0.047058823529411764, b: 0.27058823529411763, a: 1.0 })?;
    m.add("CORNFLOWER_BLUE", Color { r: 0.3176470588235294, g: 0.4392156862745098, b: 0.8431372549019608, a: 1.0 })?;
    m.add("LIGHT_OLIVE", Color { r: 0.6745098039215687, g: 0.7490196078431373, b: 0.4117647058823529, a: 1.0 })?;
    m.add("GRAPE", Color { r: 0.4235294117647059, g: 0.20392156862745098, b: 0.3803921568627451, a: 1.0 })?;
    m.add("GREYISH_BLUE", Color { r: 0.3686274509803922, g: 0.5058823529411764, b: 0.615686274509804, a: 1.0 })?;
    m.add("PURPLISH_BLUE", Color { r: 0.3764705882352941, g: 0.11764705882352941, b: 0.9764705882352941, a: 1.0 })?;
    m.add("YELLOWISH_GREEN", Color { r: 0.6901960784313725, g: 0.8666666666666667, b: 0.08627450980392157, a: 1.0 })?;
    m.add("GREENISH_YELLOW", Color { r: 0.803921568627451, g: 0.9921568627450981, b: 0.00784313725490196, a: 1.0 })?;
    m.add("MEDIUM_BLUE", Color { r: 0.17254901960784313, g: 0.43529411764705883, b: 0.7333333333333333, a: 1.0 })?;
    m.add("DUSTY_ROSE", Color { r: 0.7529411764705882, g: 0.45098039215686275, b: 0.47843137254901963, a: 1.0 })?;
    m.add("LIGHT_VIOLET", Color { r: 0.8392156862745098, g: 0.7058823529411765, b: 0.9882352941176471, a: 1.0 })?;
    m.add("MIDNIGHT_BLUE", Color { r: 0.00784313725490196, g: 0.0, b: 0.20784313725490197, a: 1.0 })?;
    m.add("BLUISH_PURPLE", Color { r: 0.4392156862745098, g: 0.23137254901960785, b: 0.9058823529411765, a: 1.0 })?;
    m.add("RED_ORANGE", Color { r: 0.9921568627450981, g: 0.23529411764705882, b: 0.023529411764705882, a: 1.0 })?;
    m.add("DARK_MAGENTA", Color { r: 0.5882352941176471, g: 0.0, b: 0.33725490196078434, a: 1.0 })?;
    m.add("GREENISH", Color { r: 0.25098039215686274, g: 0.6392156862745098, b: 0.40784313725490196, a: 1.0 })?;
    m.add("OCEAN_BLUE", Color { r: 0.011764705882352941, g: 0.44313725490196076, b: 0.611764705882353, a: 1.0 })?;
    m.add("CORAL", Color { r: 0.9882352941176471, g: 0.35294117647058826, b: 0.3137254901960784, a: 1.0 })?;
    m.add("CREAM", Color { r: 1.0, g: 1.0, b: 0.7607843137254902, a: 1.0 })?;
    m.add("REDDISH_BROWN", Color { r: 0.4980392156862745, g: 0.16862745098039217, b: 0.0392156862745098, a: 1.0 })?;
    m.add("BURNT_SIENNA", Color { r: 0.6901960784313725, g: 0.3058823529411765, b: 0.058823529411764705, a: 1.0 })?;
    m.add("BRICK", Color { r: 0.6274509803921569, g: 0.21176470588235294, b: 0.13725490196078433, a: 1.0 })?;
    m.add("SAGE", Color { r: 0.5294117647058824, g: 0.6823529411764706, b: 0.45098039215686275, a: 1.0 })?;
    m.add("GREY_GREEN", Color { r: 0.47058823529411764, g: 0.6078431372549019, b: 0.45098039215686275, a: 1.0 })?;
    m.add("WHITE", Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })?;
    m.add("ROBINS_EGG_BLUE", Color { r: 0.596078431372549, g: 0.9372549019607843, b: 0.9764705882352941, a: 1.0 })?;
    m.add("MOSS_GREEN", Color { r: 0.396078431372549, g: 0.5450980392156862, b: 0.2196078431372549, a: 1.0 })?;
    m.add("STEEL_BLUE", Color { r: 0.35294117647058826, g: 0.49019607843137253, b: 0.6039215686274509, a: 1.0 })?;
    m.add("EGGPLANT", Color { r: 0.2196078431372549, g: 0.03137254901960784, b: 0.20784313725490197, a: 1.0 })?;
    m.add("LIGHT_YELLOW", Color { r: 1.0, g: 0.996078431372549, b: 0.47843137254901963, a: 1.0 })?;
    m.add("LEAF_GREEN", Color { r: 0.3607843137254902, g: 0.6627450980392157, b: 0.01568627450980392, a: 1.0 })?;
    m.add("LIGHT_GREY", Color { r: 0.8470588235294118, g: 0.8627450980392157, b: 0.8392156862745098, a: 1.0 })?;
    m.add("PUKE", Color { r: 0.6470588235294118, g: 0.6470588235294118, b: 0.00784313725490196, a: 1.0 })?;
    m.add("PINKISH_PURPLE", Color { r: 0.8392156862745098, g: 0.2823529411764706, b: 0.8431372549019608, a: 1.0 })?;
    m.add("SEA_BLUE", Color { r: 0.01568627450980392, g: 0.4549019607843137, b: 0.5843137254901961, a: 1.0 })?;
    m.add("PALE_PURPLE", Color { r: 0.7176470588235294, g: 0.5647058823529412, b: 0.8313725490196079, a: 1.0 })?;
    m.add("SLATE_BLUE", Color { r: 0.3568627450980392, g: 0.48627450980392156, b: 0.6, a: 1.0 })?;
    m.add("BLUE_GREY", Color { r: 0.3764705882352941, g: 0.48627450980392156, b: 0.5568627450980392, a: 1.0 })?;
    m.add("HUNTER_GREEN", Color { r: 0.043137254901960784, g: 0.25098039215686274, b: 0.03137254901960784, a: 1.0 })?;
    m.add("FUCHSIA", Color { r: 0.9294117647058824, g: 0.050980392156862744, b: 0.8509803921568627, a: 1.0 })?;
    m.add("CRIMSON", Color { r: 0.5490196078431373, g: 0.0, b: 0.058823529411764705, a: 1.0 })?;
    m.add("PALE_YELLOW", Color { r: 1.0, g: 1.0, b: 0.5176470588235295, a: 1.0 })?;
    m.add("OCHRE", Color { r: 0.7490196078431373, g: 0.5647058823529412, b: 0.0196078431372549, a: 1.0 })?;
    m.add("MUSTARD_YELLOW", Color { r: 0.8235294117647058, g: 0.7411764705882353, b: 0.0392156862745098, a: 1.0 })?;
    m.add("LIGHT_RED", Color { r: 1.0, g: 0.2784313725490196, b: 0.2980392156862745, a: 1.0 })?;
    m.add("CERULEAN", Color { r: 0.01568627450980392, g: 0.5215686274509804, b: 0.8196078431372549, a: 1.0 })?;
    m.add("PALE_PINK", Color { r: 1.0, g: 0.8117647058823529, b: 0.8627450980392157, a: 1.0 })?;
    m.add("DEEP_BLUE", Color { r: 0.01568627450980392, g: 0.00784313725490196, b: 0.45098039215686275, a: 1.0 })?;
    m.add("RUST", Color { r: 0.6588235294117647, g: 0.23529411764705882, b: 0.03529411764705882, a: 1.0 })?;
    m.add("LIGHT_TEAL", Color { r: 0.5647058823529412, g: 0.8941176470588236, b: 0.7568627450980392, a: 1.0 })?;
    m.add("SLATE", Color { r: 0.3176470588235294, g: 0.396078431372549, b: 0.4470588235294118, a: 1.0 })?;
    m.add("GOLDENROD", Color { r: 0.9803921568627451, g: 0.7607843137254902, b: 0.0196078431372549, a: 1.0 })?;
    m.add("DARK_YELLOW", Color { r: 0.8352941176470589, g: 0.7137254901960784, b: 0.0392156862745098, a: 1.0 })?;
    m.add("DARK_GREY", Color { r: 0.21176470588235294, g: 0.21568627450980393, b: 0.21568627450980393, a: 1.0 })?;
    m.add("ARMY_GREEN", Color { r: 0.29411764705882354, g: 0.36470588235294116, b: 0.08627450980392157, a: 1.0 })?;
    m.add("GREY_BLUE", Color { r: 0.4196078431372549, g: 0.5450980392156862, b: 0.6431372549019608, a: 1.0 })?;
    m.add("SEAFOAM", Color { r: 0.5019607843137255, g: 0.9764705882352941, b: 0.6784313725490196, a: 1.0 })?;
    m.add("PUCE", Color { r: 0.6470588235294118, g: 0.49411764705882355, b: 0.3215686274509804, a: 1.0 })?;
    m.add("SPRING_GREEN", Color { r: 0.6627450980392157, g: 0.9764705882352941, b: 0.44313725490196076, a: 1.0 })?;
    m.add("DARK_ORANGE", Color { r: 0.7764705882352941, g: 0.3176470588235294, b: 0.00784313725490196, a: 1.0 })?;
    m.add("SAND", Color { r: 0.8862745098039215, g: 0.792156862745098, b: 0.4627450980392157, a: 1.0 })?;
    m.add("PASTEL_GREEN", Color { r: 0.6901960784313725, g: 1.0, b: 0.615686274509804, a: 1.0 })?;
    m.add("MINT", Color { r: 0.6235294117647059, g: 0.996078431372549, b: 0.6901960784313725, a: 1.0 })?;
    m.add("LIGHT_ORANGE", Color { r: 0.9921568627450981, g: 0.6666666666666666, b: 0.2823529411764706, a: 1.0 })?;
    m.add("BRIGHT_PINK", Color { r: 0.996078431372549, g: 0.00392156862745098, b: 0.6941176470588235, a: 1.0 })?;
    m.add("CHARTREUSE", Color { r: 0.7568627450980392, g: 0.9725490196078431, b: 0.0392156862745098, a: 1.0 })?;
    m.add("DEEP_PURPLE", Color { r: 0.21176470588235294, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 })?;
    m.add("DARK_BROWN", Color { r: 0.20392156862745098, g: 0.10980392156862745, b: 0.00784313725490196, a: 1.0 })?;
    m.add("TAUPE", Color { r: 0.7254901960784313, g: 0.6352941176470588, b: 0.5058823529411764, a: 1.0 })?;
    m.add("PEA_GREEN", Color { r: 0.5568627450980392, g: 0.6705882352941176, b: 0.07058823529411765, a: 1.0 })?;
    m.add("PUKE_GREEN", Color { r: 0.6039215686274509, g: 0.6823529411764706, b: 0.027450980392156862, a: 1.0 })?;
    m.add("KELLY_GREEN", Color { r: 0.00784313725490196, g: 0.6705882352941176, b: 0.1803921568627451, a: 1.0 })?;
    m.add("SEAFOAM_GREEN", Color { r: 0.47843137254901963, g: 0.9764705882352941, b: 0.6705882352941176, a: 1.0 })?;
    m.add("BLUE_GREEN", Color { r: 0.07450980392156863, g: 0.49411764705882355, b: 0.42745098039215684, a: 1.0 })?;
    m.add("KHAKI", Color { r: 0.6666666666666666, g: 0.6509803921568628, b: 0.3843137254901961, a: 1.0 })?;
    m.add("BURGUNDY", Color { r: 0.3803921568627451, g: 0.0, b: 0.13725490196078433, a: 1.0 })?;
    m.add("DARK_TEAL", Color { r: 0.00392156862745098, g: 0.30196078431372547, b: 0.3058823529411765, a: 1.0 })?;
    m.add("BRICK_RED", Color { r: 0.5607843137254902, g: 0.0784313725490196, b: 0.00784313725490196, a: 1.0 })?;
    m.add("ROYAL_PURPLE", Color { r: 0.29411764705882354, g: 0.0, b: 0.43137254901960786, a: 1.0 })?;
    m.add("PLUM", Color { r: 0.34509803921568627, g: 0.058823529411764705, b: 0.2549019607843137, a: 1.0 })?;
    m.add("MINT_GREEN", Color { r: 0.5607843137254902, g: 1.0, b: 0.6235294117647059, a: 1.0 })?;
    m.add("GOLD", Color { r: 0.8588235294117647, g: 0.7058823529411765, b: 0.047058823529411764, a: 1.0 })?;
    m.add("BABY_BLUE", Color { r: 0.6352941176470588, g: 0.8117647058823529, b: 0.996078431372549, a: 1.0 })?;
    m.add("YELLOW_GREEN", Color { r: 0.7529411764705882, g: 0.984313725490196, b: 0.17647058823529413, a: 1.0 })?;
    m.add("BRIGHT_PURPLE", Color { r: 0.7450980392156863, g: 0.011764705882352941, b: 0.9921568627450981, a: 1.0 })?;
    m.add("DARK_RED", Color { r: 0.5176470588235295, g: 0.0, b: 0.0, a: 1.0 })?;
    m.add("PALE_BLUE", Color { r: 0.8156862745098039, g: 0.996078431372549, b: 0.996078431372549, a: 1.0 })?;
    m.add("GRASS_GREEN", Color { r: 0.24705882352941178, g: 0.6078431372549019, b: 0.043137254901960784, a: 1.0 })?;
    m.add("NAVY", Color { r: 0.00392156862745098, g: 0.08235294117647059, b: 0.24313725490196078, a: 1.0 })?;
    m.add("AQUAMARINE", Color { r: 0.01568627450980392, g: 0.8470588235294118, b: 0.6980392156862745, a: 1.0 })?;
    m.add("BURNT_ORANGE", Color { r: 0.7529411764705882, g: 0.3058823529411765, b: 0.00392156862745098, a: 1.0 })?;
    m.add("NEON_GREEN", Color { r: 0.047058823529411764, g: 1.0, b: 0.047058823529411764, a: 1.0 })?;
    m.add("BRIGHT_BLUE", Color { r: 0.00392156862745098, g: 0.396078431372549, b: 0.9882352941176471, a: 1.0 })?;
    m.add("ROSE", Color { r: 0.8117647058823529, g: 0.3843137254901961, b: 0.4588235294117647, a: 1.0 })?;
    m.add("LIGHT_PINK", Color { r: 1.0, g: 0.8196078431372549, b: 0.8745098039215686, a: 1.0 })?;
    m.add("MUSTARD", Color { r: 0.807843137254902, g: 0.7019607843137254, b: 0.00392156862745098, a: 1.0 })?;
    m.add("INDIGO", Color { r: 0.2196078431372549, g: 0.00784313725490196, b: 0.5098039215686274, a: 1.0 })?;
    m.add("LIME", Color { r: 0.6666666666666666, g: 1.0, b: 0.19607843137254902, a: 1.0 })?;
    m.add("SEA_GREEN", Color { r: 0.3254901960784314, g: 0.9882352941176471, b: 0.6313725490196078, a: 1.0 })?;
    m.add("PERIWINKLE", Color { r: 0.5568627450980392, g: 0.5098039215686274, b: 0.996078431372549, a: 1.0 })?;
    m.add("DARK_PINK", Color { r: 0.796078431372549, g: 0.2549019607843137, b: 0.4196078431372549, a: 1.0 })?;
    m.add("OLIVE_GREEN", Color { r: 0.403921568627451, g: 0.47843137254901963, b: 0.01568627450980392, a: 1.0 })?;
    m.add("PEACH", Color { r: 1.0, g: 0.6901960784313725, b: 0.48627450980392156, a: 1.0 })?;
    m.add("PALE_GREEN", Color { r: 0.7803921568627451, g: 0.9921568627450981, b: 0.7098039215686275, a: 1.0 })?;
    m.add("LIGHT_BROWN", Color { r: 0.6784313725490196, g: 0.5058823529411764, b: 0.3137254901960784, a: 1.0 })?;
    m.add("HOT_PINK", Color { r: 1.0, g: 0.00784313725490196, b: 0.5529411764705883, a: 1.0 })?;
    m.add("BLACK", Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 })?;
    m.add("LILAC", Color { r: 0.807843137254902, g: 0.6352941176470588, b: 0.9921568627450981, a: 1.0 })?;
    m.add("NAVY_BLUE", Color { r: 0.0, g: 0.06666666666666667, b: 0.27450980392156865, a: 1.0 })?;
    m.add("ROYAL_BLUE", Color { r: 0.0196078431372549, g: 0.01568627450980392, b: 0.6666666666666666, a: 1.0 })?;
    m.add("BEIGE", Color { r: 0.9019607843137255, g: 0.8549019607843137, b: 0.6509803921568628, a: 1.0 })?;
    m.add("SALMON", Color { r: 1.0, g: 0.4745098039215686, b: 0.4235294117647059, a: 1.0 })?;
    m.add("OLIVE", Color { r: 0.43137254901960786, g: 0.4588235294117647, b: 0.054901960784313725, a: 1.0 })?;
    m.add("MAROON", Color { r: 0.396078431372549, g: 0.0, b: 0.12941176470588237, a: 1.0 })?;
    m.add("BRIGHT_GREEN", Color { r: 0.00392156862745098, g: 1.0, b: 0.027450980392156862, a: 1.0 })?;
    m.add("DARK_PURPLE", Color { r: 0.20784313725490197, g: 0.023529411764705882, b: 0.24313725490196078, a: 1.0 })?;
    m.add("MAUVE", Color { r: 0.6823529411764706, g: 0.44313725490196076, b: 0.5058823529411764, a: 1.0 })?;
    m.add("FOREST_GREEN", Color { r: 0.023529411764705882, g: 0.2784313725490196, b: 0.047058823529411764, a: 1.0 })?;
    m.add("AQUA", Color { r: 0.07450980392156863, g: 0.9176470588235294, b: 0.788235294117647, a: 1.0 })?;
    m.add("CYAN", Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 })?;
    m.add("TAN", Color { r: 0.8196078431372549, g: 0.6980392156862745, b: 0.43529411764705883, a: 1.0 })?;
    m.add("DARK_BLUE", Color { r: 0.0, g: 0.011764705882352941, b: 0.3568627450980392, a: 1.0 })?;
    m.add("LAVENDER", Color { r: 0.7803921568627451, g: 0.6235294117647059, b: 0.9372549019607843, a: 1.0 })?;
    m.add("TURQUOISE", Color { r: 0.023529411764705882, g: 0.7607843137254902, b: 0.6745098039215687, a: 1.0 })?;
    m.add("DARK_GREEN", Color { r: 0.011764705882352941, g: 0.20784313725490197, b: 0.0, a: 1.0 })?;
    m.add("VIOLET", Color { r: 0.6039215686274509, g: 0.054901960784313725, b: 0.9176470588235294, a: 1.0 })?;
    m.add("LIGHT_PURPLE", Color { r: 0.7490196078431373, g: 0.4666666666666667, b: 0.9647058823529412, a: 1.0 })?;
    m.add("LIME_GREEN", Color { r: 0.5372549019607843, g: 0.996078431372549, b: 0.0196078431372549, a: 1.0 })?;
    m.add("GREY", Color { r: 0.5725490196078431, g: 0.5843137254901961, b: 0.5686274509803921, a: 1.0 })?;
    m.add("SKY_BLUE", Color { r: 0.4588235294117647, g: 0.7333333333333333, b: 0.9921568627450981, a: 1.0 })?;
    m.add("YELLOW", Color { r: 1.0, g: 1.0, b: 0.0784313725490196, a: 1.0 })?;
    m.add("MAGENTA", Color { r: 0.7607843137254902, g: 0.0, b: 0.47058823529411764, a: 1.0 })?;
    m.add("LIGHT_GREEN", Color { r: 0.5882352941176471, g: 0.9764705882352941, b: 0.4823529411764706, a: 1.0 })?;
    m.add("ORANGE", Color { r: 0.9764705882352941, g: 0.45098039215686275, b: 0.023529411764705882, a: 1.0 })?;
    m.add("TEAL", Color { r: 0.00784313725490196, g: 0.5764705882352941, b: 0.5254901960784314, a: 1.0 })?;
    m.add("LIGHT_BLUE", Color { r: 0.5843137254901961, g: 0.8156862745098039, b: 0.9882352941176471, a: 1.0 })?;
    m.add("RED", Color { r: 0.8980392156862745, g: 0.0, b: 0.0, a: 1.0 })?;
    m.add("BROWN", Color { r: 0.396078431372549, g: 0.21568627450980393, b: 0.0, a: 1.0 })?;
    m.add("PINK", Color { r: 1.0, g: 0.5058823529411764, b: 0.7529411764705882, a: 1.0 })?;
    m.add("BLUE", Color { r: 0.011764705882352941, g: 0.2627450980392157, b: 0.8745098039215686, a: 1.0 })?;
    m.add("GREEN", Color { r: 0.08235294117647059, g: 0.6901960784313725, b: 0.10196078431372549, a: 1.0 })?;
    m.add("PURPLE", Color { r: 0.49411764705882355, g: 0.11764705882352941, b: 0.611764705882353, a: 1.0 })?;
    m.add("GRAY_TEAL", Color { r: 0.3686274509803922, g: 0.6078431372549019, b: 0.5411764705882353, a: 1.0 })?;
    m.add("PURPLEY_GRAY", Color { r: 0.5803921568627451, g: 0.49411764705882355, b: 0.5803921568627451, a: 1.0 })?;
    m.add("LIGHT_GRAY_GREEN", Color { r: 0.7176470588235294, g: 0.8823529411764706, b: 0.6313725490196078, a: 1.0 })?;
    m.add("REDDISH_GRAY", Color { r: 0.6, g: 0.4588235294117647, b: 0.4392156862745098, a: 1.0 })?;
    m.add("BATTLESHIP_GRAY", Color { r: 0.4196078431372549, g: 0.48627450980392156, b: 0.5215686274509804, a: 1.0 })?;
    m.add("CHARCOAL_GRAY", Color { r: 0.23529411764705882, g: 0.2549019607843137, b: 0.25882352941176473, a: 1.0 })?;
    m.add("GRAYISH_TEAL", Color { r: 0.44313725490196076, g: 0.6235294117647059, b: 0.5686274509803921, a: 1.0 })?;
    m.add("GRAY_GREEN", Color { r: 0.5254901960784314, g: 0.6313725490196078, b: 0.49019607843137253, a: 1.0 })?;
    m.add("COOL_GRAY", Color { r: 0.5843137254901961, g: 0.6392156862745098, b: 0.6509803921568628, a: 1.0 })?;
    m.add("DARK_BLUE_GRAY", Color { r: 0.12156862745098039, g: 0.23137254901960785, b: 0.30196078431372547, a: 1.0 })?;
    m.add("BLUEY_GRAY", Color { r: 0.5372549019607843, g: 0.6274509803921569, b: 0.6901960784313725, a: 1.0 })?;
    m.add("GREENY_GRAY", Color { r: 0.49411764705882355, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 })?;
    m.add("BLUEGRAY", Color { r: 0.5215686274509804, g: 0.6392156862745098, b: 0.6980392156862745, a: 1.0 })?;
    m.add("LIGHT_BLUE_GRAY", Color { r: 0.7176470588235294, g: 0.788235294117647, b: 0.8862745098039215, a: 1.0 })?;
    m.add("GRAY_BLUE", Color { r: 0.39215686274509803, g: 0.49019607843137253, b: 0.5568627450980392, a: 1.0 })?;
    m.add("BROWN_GRAY", Color { r: 0.5529411764705883, g: 0.5176470588235295, b: 0.40784313725490196, a: 1.0 })?;
    m.add("BLUE_GRAY", Color { r: 0.4588235294117647, g: 0.5529411764705883, b: 0.6392156862745098, a: 1.0 })?;
    m.add("GRAYBLUE", Color { r: 0.4666666666666667, g: 0.6313725490196078, b: 0.7098039215686275, a: 1.0 })?;
    m.add("DARK_GRAY_BLUE", Color { r: 0.1607843137254902, g: 0.27450980392156865, b: 0.3568627450980392, a: 1.0 })?;
    m.add("GRAYISH", Color { r: 0.6588235294117647, g: 0.6431372549019608, b: 0.5843137254901961, a: 1.0 })?;
    m.add("LIGHT_GRAY_BLUE", Color { r: 0.615686274509804, g: 0.7372549019607844, b: 0.8313725490196079, a: 1.0 })?;
    m.add("PALE_GRAY", Color { r: 0.9921568627450981, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 })?;
    m.add("WARM_GRAY", Color { r: 0.592156862745098, g: 0.5411764705882353, b: 0.5176470588235295, a: 1.0 })?;
    m.add("GRAY_PINK", Color { r: 0.7647058823529411, g: 0.5647058823529412, b: 0.6078431372549019, a: 1.0 })?;
    m.add("MEDIUM_GRAY", Color { r: 0.49019607843137253, g: 0.4980392156862745, b: 0.48627450980392156, a: 1.0 })?;
    m.add("PINKISH_GRAY", Color { r: 0.7843137254901961, g: 0.6745098039215687, b: 0.6627450980392157, a: 1.0 })?;
    m.add("BROWNISH_GRAY", Color { r: 0.5254901960784314, g: 0.4666666666666667, b: 0.37254901960784315, a: 1.0 })?;
    m.add("PURPLISH_GRAY", Color { r: 0.47843137254901963, g: 0.40784313725490196, b: 0.4980392156862745, a: 1.0 })?;
    m.add("GRAYISH_PINK", Color { r: 0.7843137254901961, g: 0.5529411764705883, b: 0.5803921568627451, a: 1.0 })?;
    m.add("GRAYISH_BROWN", Color { r: 0.47843137254901963, g: 0.41568627450980394, b: 0.30980392156862746, a: 1.0 })?;
    m.add("STEEL_GRAY", Color { r: 0.43529411764705883, g: 0.5098039215686274, b: 0.5411764705882353, a: 1.0 })?;
    m.add("PURPLE_GRAY", Color { r: 0.5254901960784314, g: 0.43529411764705883, b: 0.5215686274509804, a: 1.0 })?;
    m.add("GRAY_BROWN", Color { r: 0.4980392156862745, g: 0.4392156862745098, b: 0.3254901960784314, a: 1.0 })?;
    m.add("GREEN_GRAY", Color { r: 0.4666666666666667, g: 0.5725490196078431, b: 0.43529411764705883, a: 1.0 })?;
    m.add("BLUISH_GRAY", Color { r: 0.4549019607843137, g: 0.5450980392156862, b: 0.592156862745098, a: 1.0 })?;
    m.add("SLATE_GRAY", Color { r: 0.34901960784313724, g: 0.396078431372549, b: 0.42745098039215684, a: 1.0 })?;
    m.add("GRAY_PURPLE", Color { r: 0.5098039215686274, g: 0.42745098039215684, b: 0.5490196078431373, a: 1.0 })?;
    m.add("GREENISH_GRAY", Color { r: 0.5882352941176471, g: 0.6823529411764706, b: 0.5529411764705883, a: 1.0 })?;
    m.add("GRAYISH_PURPLE", Color { r: 0.5333333333333333, g: 0.44313725490196076, b: 0.5686274509803921, a: 1.0 })?;
    m.add("GRAYISH_GREEN", Color { r: 0.5098039215686274, g: 0.6509803921568628, b: 0.49019607843137253, a: 1.0 })?;
    m.add("GRAYISH_BLUE", Color { r: 0.3686274509803922, g: 0.5058823529411764, b: 0.615686274509804, a: 1.0 })?;
    m.add("GRAY_GREEN", Color { r: 0.47058823529411764, g: 0.6078431372549019, b: 0.45098039215686275, a: 1.0 })?;
    m.add("LIGHT_GRAY", Color { r: 0.8470588235294118, g: 0.8627450980392157, b: 0.8392156862745098, a: 1.0 })?;
    m.add("BLUE_GRAY", Color { r: 0.3764705882352941, g: 0.48627450980392156, b: 0.5568627450980392, a: 1.0 })?;
    m.add("DARK_GRAY", Color { r: 0.21176470588235294, g: 0.21568627450980393, b: 0.21568627450980393, a: 1.0 })?;
    m.add("GRAY_BLUE", Color { r: 0.4196078431372549, g: 0.5450980392156862, b: 0.6431372549019608, a: 1.0 })?;
    m.add("GRAY", Color { r: 0.5725490196078431, g: 0.5843137254901961, b: 0.5686274509803921, a: 1.0 })?;
    m.add("ALICEBLUE", Color { r: 0.9411764705882353, g: 0.9725490196078431, b: 1.0, a: 1.0 })?;
    m.add("ANTIQUEWHITE", Color { r: 0.9803921568627451, g: 0.9215686274509803, b: 0.8431372549019608, a: 1.0 })?;
    m.add("AQUA", Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 })?;
    m.add("AQUAMARINE", Color { r: 0.4980392156862745, g: 1.0, b: 0.8313725490196079, a: 1.0 })?;
    m.add("AZURE", Color { r: 0.9411764705882353, g: 1.0, b: 1.0, a: 1.0 })?;
    m.add("BEIGE", Color { r: 0.9607843137254902, g: 0.9607843137254902, b: 0.8627450980392157, a: 1.0 })?;
    m.add("BISQUE", Color { r: 1.0, g: 0.8941176470588236, b: 0.7686274509803922, a: 1.0 })?;
    m.add("BLACK", Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 })?;
    m.add("BLANCHEDALMOND", Color { r: 1.0, g: 0.9215686274509803, b: 0.803921568627451, a: 1.0 })?;
    m.add("BLUE", Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 })?;
    m.add("BLUEVIOLET", Color { r: 0.5411764705882353, g: 0.16862745098039217, b: 0.8862745098039215, a: 1.0 })?;
    m.add("BROWN", Color { r: 0.6470588235294118, g: 0.16470588235294117, b: 0.16470588235294117, a: 1.0 })?;
    m.add("BURLYWOOD", Color { r: 0.8705882352941177, g: 0.7215686274509804, b: 0.5294117647058824, a: 1.0 })?;
    m.add("CADETBLUE", Color { r: 0.37254901960784315, g: 0.6196078431372549, b: 0.6274509803921569, a: 1.0 })?;
    m.add("CHARTREUSE", Color { r: 0.4980392156862745, g: 1.0, b: 0.0, a: 1.0 })?;
    m.add("CHOCOLATE", Color { r: 0.8235294117647058, g: 0.4117647058823529, b: 0.11764705882352941, a: 1.0 })?;
    m.add("CORAL", Color { r: 1.0, g: 0.4980392156862745, b: 0.3137254901960784, a: 1.0 })?;
    m.add("CORNFLOWERBLUE", Color { r: 0.39215686274509803, g: 0.5843137254901961, b: 0.9294117647058824, a: 1.0 })?;
    m.add("CORNSILK", Color { r: 1.0, g: 0.9725490196078431, b: 0.8627450980392157, a: 1.0 })?;
    m.add("CRIMSON", Color { r: 0.8627450980392157, g: 0.0784313725490196, b: 0.23529411764705882, a: 1.0 })?;
    m.add("CYAN", Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 })?;
    m.add("DARKBLUE", Color { r: 0.0, g: 0.0, b: 0.5450980392156862, a: 1.0 })?;
    m.add("DARKCYAN", Color { r: 0.0, g: 0.5450980392156862, b: 0.5450980392156862, a: 1.0 })?;
    m.add("DARKGOLDENROD", Color { r: 0.7215686274509804, g: 0.5254901960784314, b: 0.043137254901960784, a: 1.0 })?;
    m.add("DARKGRAY", Color { r: 0.6627450980392157, g: 0.6627450980392157, b: 0.6627450980392157, a: 1.0 })?;
    m.add("DARKGREEN", Color { r: 0.0, g: 0.39215686274509803, b: 0.0, a: 1.0 })?;
    m.add("DARKGREY", Color { r: 0.6627450980392157, g: 0.6627450980392157, b: 0.6627450980392157, a: 1.0 })?;
    m.add("DARKKHAKI", Color { r: 0.7411764705882353, g: 0.7176470588235294, b: 0.4196078431372549, a: 1.0 })?;
    m.add("DARKMAGENTA", Color { r: 0.5450980392156862, g: 0.0, b: 0.5450980392156862, a: 1.0 })?;
    m.add("DARKOLIVEGREEN", Color { r: 0.3333333333333333, g: 0.4196078431372549, b: 0.1843137254901961, a: 1.0 })?;
    m.add("DARKORANGE", Color { r: 1.0, g: 0.5490196078431373, b: 0.0, a: 1.0 })?;
    m.add("DARKORCHID", Color { r: 0.6, g: 0.19607843137254902, b: 0.8, a: 1.0 })?;
    m.add("DARKRED", Color { r: 0.5450980392156862, g: 0.0, b: 0.0, a: 1.0 })?;
    m.add("DARKSALMON", Color { r: 0.9137254901960784, g: 0.5882352941176471, b: 0.47843137254901963, a: 1.0 })?;
    m.add("DARKSEAGREEN", Color { r: 0.5607843137254902, g: 0.7372549019607844, b: 0.5607843137254902, a: 1.0 })?;
    m.add("DARKSLATEBLUE", Color { r: 0.2823529411764706, g: 0.23921568627450981, b: 0.5450980392156862, a: 1.0 })?;
    m.add("DARKSLATEGRAY", Color { r: 0.1843137254901961, g: 0.30980392156862746, b: 0.30980392156862746, a: 1.0 })?;
    m.add("DARKSLATEGREY", Color { r: 0.1843137254901961, g: 0.30980392156862746, b: 0.30980392156862746, a: 1.0 })?;
    m.add("DARKTURQUOISE", Color { r: 0.0, g: 0.807843137254902, b: 0.8196078431372549, a: 1.0 })?;
    m.add("DARKVIOLET", Color { r: 0.5803921568627451, g: 0.0, b: 0.8274509803921568, a: 1.0 })?;
    m.add("DEEPPINK", Color { r: 1.0, g: 0.0784313725490196, b: 0.5764705882352941, a: 1.0 })?;
    m.add("DEEPSKYBLUE", Color { r: 0.0, g: 0.7490196078431373, b: 1.0, a: 1.0 })?;
    m.add("DIMGRAY", Color { r: 0.4117647058823529, g: 0.4117647058823529, b: 0.4117647058823529, a: 1.0 })?;
    m.add("DIMGREY", Color { r: 0.4117647058823529, g: 0.4117647058823529, b: 0.4117647058823529, a: 1.0 })?;
    m.add("DODGERBLUE", Color { r: 0.11764705882352941, g: 0.5647058823529412, b: 1.0, a: 1.0 })?;
    m.add("FIREBRICK", Color { r: 0.6980392156862745, g: 0.13333333333333333, b: 0.13333333333333333, a: 1.0 })?;
    m.add("FLORALWHITE", Color { r: 1.0, g: 0.9803921568627451, b: 0.9411764705882353, a: 1.0 })?;
    m.add("FORESTGREEN", Color { r: 0.13333333333333333, g: 0.5450980392156862, b: 0.13333333333333333, a: 1.0 })?;
    m.add("FUCHSIA", Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 })?;
    m.add("GAINSBORO", Color { r: 0.8627450980392157, g: 0.8627450980392157, b: 0.8627450980392157, a: 1.0 })?;
    m.add("GHOSTWHITE", Color { r: 0.9725490196078431, g: 0.9725490196078431, b: 1.0, a: 1.0 })?;
    m.add("GOLD", Color { r: 1.0, g: 0.8431372549019608, b: 0.0, a: 1.0 })?;
    m.add("GOLDENROD", Color { r: 0.8549019607843137, g: 0.6470588235294118, b: 0.12549019607843137, a: 1.0 })?;
    m.add("GRAY", Color { r: 0.5019607843137255, g: 0.5019607843137255, b: 0.5019607843137255, a: 1.0 })?;
    m.add("GREEN", Color { r: 0.0, g: 0.5019607843137255, b: 0.0, a: 1.0 })?;
    m.add("GREENYELLOW", Color { r: 0.6784313725490196, g: 1.0, b: 0.1843137254901961, a: 1.0 })?;
    m.add("GREY", Color { r: 0.5019607843137255, g: 0.5019607843137255, b: 0.5019607843137255, a: 1.0 })?;
    m.add("HONEYDEW", Color { r: 0.9411764705882353, g: 1.0, b: 0.9411764705882353, a: 1.0 })?;
    m.add("HOTPINK", Color { r: 1.0, g: 0.4117647058823529, b: 0.7058823529411765, a: 1.0 })?;
    m.add("INDIANRED", Color { r: 0.803921568627451, g: 0.3607843137254902, b: 0.3607843137254902, a: 1.0 })?;
    m.add("INDIGO", Color { r: 0.29411764705882354, g: 0.0, b: 0.5098039215686274, a: 1.0 })?;
    m.add("IVORY", Color { r: 1.0, g: 1.0, b: 0.9411764705882353, a: 1.0 })?;
    m.add("KHAKI", Color { r: 0.9411764705882353, g: 0.9019607843137255, b: 0.5490196078431373, a: 1.0 })?;
    m.add("LAVENDER", Color { r: 0.9019607843137255, g: 0.9019607843137255, b: 0.9803921568627451, a: 1.0 })?;
    m.add("LAVENDERBLUSH", Color { r: 1.0, g: 0.9411764705882353, b: 0.9607843137254902, a: 1.0 })?;
    m.add("LAWNGREEN", Color { r: 0.48627450980392156, g: 0.9882352941176471, b: 0.0, a: 1.0 })?;
    m.add("LEMONCHIFFON", Color { r: 1.0, g: 0.9803921568627451, b: 0.803921568627451, a: 1.0 })?;
    m.add("LIGHTBLUE", Color { r: 0.6784313725490196, g: 0.8470588235294118, b: 0.9019607843137255, a: 1.0 })?;
    m.add("LIGHTCORAL", Color { r: 0.9411764705882353, g: 0.5019607843137255, b: 0.5019607843137255, a: 1.0 })?;
    m.add("LIGHTCYAN", Color { r: 0.8784313725490196, g: 1.0, b: 1.0, a: 1.0 })?;
    m.add("LIGHTGOLDENRODYELLOW", Color { r: 0.9803921568627451, g: 0.9803921568627451, b: 0.8235294117647058, a: 1.0 })?;
    m.add("LIGHTGRAY", Color { r: 0.8274509803921568, g: 0.8274509803921568, b: 0.8274509803921568, a: 1.0 })?;
    m.add("LIGHTGREEN", Color { r: 0.5647058823529412, g: 0.9333333333333333, b: 0.5647058823529412, a: 1.0 })?;
    m.add("LIGHTGREY", Color { r: 0.8274509803921568, g: 0.8274509803921568, b: 0.8274509803921568, a: 1.0 })?;
    m.add("LIGHTPINK", Color { r: 1.0, g: 0.7137254901960784, b: 0.7568627450980392, a: 1.0 })?;
    m.add("LIGHTSALMON", Color { r: 1.0, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 })?;
    m.add("LIGHTSEAGREEN", Color { r: 0.12549019607843137, g: 0.6980392156862745, b: 0.6666666666666666, a: 1.0 })?;
    m.add("LIGHTSKYBLUE", Color { r: 0.5294117647058824, g: 0.807843137254902, b: 0.9803921568627451, a: 1.0 })?;
    m.add("LIGHTSLATEGRAY", Color { r: 0.4666666666666667, g: 0.5333333333333333, b: 0.6, a: 1.0 })?;
    m.add("LIGHTSLATEGREY", Color { r: 0.4666666666666667, g: 0.5333333333333333, b: 0.6, a: 1.0 })?;
    m.add("LIGHTSTEELBLUE", Color { r: 0.6901960784313725, g: 0.7686274509803922, b: 0.8705882352941177, a: 1.0 })?;
    m.add("LIGHTYELLOW", Color { r: 1.0, g: 1.0, b: 0.8784313725490196, a: 1.0 })?;
    m.add("LIME", Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 })?;
    m.add("LIMEGREEN", Color { r: 0.19607843137254902, g: 0.803921568627451, b: 0.19607843137254902, a: 1.0 })?;
    m.add("LINEN", Color { r: 0.9803921568627451, g: 0.9411764705882353, b: 0.9019607843137255, a: 1.0 })?;
    m.add("MAGENTA", Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 })?;
    m.add("MAROON", Color { r: 0.5019607843137255, g: 0.0, b: 0.0, a: 1.0 })?;
    m.add("MEDIUMAQUAMARINE", Color { r: 0.4, g: 0.803921568627451, b: 0.6666666666666666, a: 1.0 })?;
    m.add("MEDIUMBLUE", Color { r: 0.0, g: 0.0, b: 0.803921568627451, a: 1.0 })?;
    m.add("MEDIUMORCHID", Color { r: 0.7294117647058823, g: 0.3333333333333333, b: 0.8274509803921568, a: 1.0 })?;
    m.add("MEDIUMPURPLE", Color { r: 0.5764705882352941, g: 0.4392156862745098, b: 0.8588235294117647, a: 1.0 })?;
    m.add("MEDIUMSEAGREEN", Color { r: 0.23529411764705882, g: 0.7019607843137254, b: 0.44313725490196076, a: 1.0 })?;
    m.add("MEDIUMSLATEBLUE", Color { r: 0.4823529411764706, g: 0.40784313725490196, b: 0.9333333333333333, a: 1.0 })?;
    m.add("MEDIUMSPRINGGREEN", Color { r: 0.0, g: 0.9803921568627451, b: 0.6039215686274509, a: 1.0 })?;
    m.add("MEDIUMTURQUOISE", Color { r: 0.2823529411764706, g: 0.8196078431372549, b: 0.8, a: 1.0 })?;
    m.add("MEDIUMVIOLETRED", Color { r: 0.7803921568627451, g: 0.08235294117647059, b: 0.5215686274509804, a: 1.0 })?;
    m.add("MIDNIGHTBLUE", Color { r: 0.09803921568627451, g: 0.09803921568627451, b: 0.4392156862745098, a: 1.0 })?;
    m.add("MINTCREAM", Color { r: 0.9607843137254902, g: 1.0, b: 0.9803921568627451, a: 1.0 })?;
    m.add("MISTYROSE", Color { r: 1.0, g: 0.8941176470588236, b: 0.8823529411764706, a: 1.0 })?;
    m.add("MOCCASIN", Color { r: 1.0, g: 0.8941176470588236, b: 0.7098039215686275, a: 1.0 })?;
    m.add("NAVAJOWHITE", Color { r: 1.0, g: 0.8705882352941177, b: 0.6784313725490196, a: 1.0 })?;
    m.add("NAVY", Color { r: 0.0, g: 0.0, b: 0.5019607843137255, a: 1.0 })?;
    m.add("OLDLACE", Color { r: 0.9921568627450981, g: 0.9607843137254902, b: 0.9019607843137255, a: 1.0 })?;
    m.add("OLIVE", Color { r: 0.5019607843137255, g: 0.5019607843137255, b: 0.0, a: 1.0 })?;
    m.add("OLIVEDRAB", Color { r: 0.4196078431372549, g: 0.5568627450980392, b: 0.13725490196078433, a: 1.0 })?;
    m.add("ORANGE", Color { r: 1.0, g: 0.6470588235294118, b: 0.0, a: 1.0 })?;
    m.add("ORANGERED", Color { r: 1.0, g: 0.27058823529411763, b: 0.0, a: 1.0 })?;
    m.add("ORCHID", Color { r: 0.8549019607843137, g: 0.4392156862745098, b: 0.8392156862745098, a: 1.0 })?;
    m.add("PALEGOLDENROD", Color { r: 0.9333333333333333, g: 0.9098039215686274, b: 0.6666666666666666, a: 1.0 })?;
    m.add("PALEGREEN", Color { r: 0.596078431372549, g: 0.984313725490196, b: 0.596078431372549, a: 1.0 })?;
    m.add("PALETURQUOISE", Color { r: 0.6862745098039216, g: 0.9333333333333333, b: 0.9333333333333333, a: 1.0 })?;
    m.add("PALEVIOLETRED", Color { r: 0.8588235294117647, g: 0.4392156862745098, b: 0.5764705882352941, a: 1.0 })?;
    m.add("PAPAYAWHIP", Color { r: 1.0, g: 0.9372549019607843, b: 0.8352941176470589, a: 1.0 })?;
    m.add("PEACHPUFF", Color { r: 1.0, g: 0.8549019607843137, b: 0.7254901960784313, a: 1.0 })?;
    m.add("PERU", Color { r: 0.803921568627451, g: 0.5215686274509804, b: 0.24705882352941178, a: 1.0 })?;
    m.add("PINK", Color { r: 1.0, g: 0.7529411764705882, b: 0.796078431372549, a: 1.0 })?;
    m.add("PLUM", Color { r: 0.8666666666666667, g: 0.6274509803921569, b: 0.8666666666666667, a: 1.0 })?;
    m.add("POWDERBLUE", Color { r: 0.6901960784313725, g: 0.8784313725490196, b: 0.9019607843137255, a: 1.0 })?;
    m.add("PURPLE", Color { r: 0.5019607843137255, g: 0.0, b: 0.5019607843137255, a: 1.0 })?;
    m.add("REBECCAPURPLE", Color { r: 0.4, g: 0.2, b: 0.6, a: 1.0 })?;
    m.add("RED", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 })?;
    m.add("ROSYBROWN", Color { r: 0.7372549019607844, g: 0.5607843137254902, b: 0.5607843137254902, a: 1.0 })?;
    m.add("ROYALBLUE", Color { r: 0.2549019607843137, g: 0.4117647058823529, b: 0.8823529411764706, a: 1.0 })?;
    m.add("SADDLEBROWN", Color { r: 0.5450980392156862, g: 0.27058823529411763, b: 0.07450980392156863, a: 1.0 })?;
    m.add("SALMON", Color { r: 0.9803921568627451, g: 0.5019607843137255, b: 0.4470588235294118, a: 1.0 })?;
    m.add("SANDYBROWN", Color { r: 0.9568627450980393, g: 0.6431372549019608, b: 0.3764705882352941, a: 1.0 })?;
    m.add("SEAGREEN", Color { r: 0.1803921568627451, g: 0.5450980392156862, b: 0.3411764705882353, a: 1.0 })?;
    m.add("SEASHELL", Color { r: 1.0, g: 0.9607843137254902, b: 0.9333333333333333, a: 1.0 })?;
    m.add("SIENNA", Color { r: 0.6274509803921569, g: 0.3215686274509804, b: 0.17647058823529413, a: 1.0 })?;
    m.add("SILVER", Color { r: 0.7529411764705882, g: 0.7529411764705882, b: 0.7529411764705882, a: 1.0 })?;
    m.add("SKYBLUE", Color { r: 0.5294117647058824, g: 0.807843137254902, b: 0.9215686274509803, a: 1.0 })?;
    m.add("SLATEBLUE", Color { r: 0.41568627450980394, g: 0.35294117647058826, b: 0.803921568627451, a: 1.0 })?;
    m.add("SLATEGRAY", Color { r: 0.4392156862745098, g: 0.5019607843137255, b: 0.5647058823529412, a: 1.0 })?;
    m.add("SLATEGREY", Color { r: 0.4392156862745098, g: 0.5019607843137255, b: 0.5647058823529412, a: 1.0 })?;
    m.add("SNOW", Color { r: 1.0, g: 0.9803921568627451, b: 0.9803921568627451, a: 1.0 })?;
    m.add("SPRINGGREEN", Color { r: 0.0, g: 1.0, b: 0.4980392156862745, a: 1.0 })?;
    m.add("STEELBLUE", Color { r: 0.27450980392156865, g: 0.5098039215686274, b: 0.7058823529411765, a: 1.0 })?;
    m.add("TAN", Color { r: 0.8235294117647058, g: 0.7058823529411765, b: 0.5490196078431373, a: 1.0 })?;
    m.add("TEAL", Color { r: 0.0, g: 0.5019607843137255, b: 0.5019607843137255, a: 1.0 })?;
    m.add("THISTLE", Color { r: 0.8470588235294118, g: 0.7490196078431373, b: 0.8470588235294118, a: 1.0 })?;
    m.add("TOMATO", Color { r: 1.0, g: 0.38823529411764707, b: 0.2784313725490196, a: 1.0 })?;
    m.add("TURQUOISE", Color { r: 0.25098039215686274, g: 0.8784313725490196, b: 0.8156862745098039, a: 1.0 })?;
    m.add("VIOLET", Color { r: 0.9333333333333333, g: 0.5098039215686274, b: 0.9333333333333333, a: 1.0 })?;
    m.add("WHEAT", Color { r: 0.9607843137254902, g: 0.8705882352941177, b: 0.7019607843137254, a: 1.0 })?;
    m.add("WHITE", Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })?;
    m.add("WHITESMOKE", Color { r: 0.9607843137254902, g: 0.9607843137254902, b: 0.9607843137254902, a: 1.0 })?;
    m.add("YELLOW", Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 })?;
    m.add("YELLOWGREEN", Color { r: 0.6039215686274509, g: 0.803921568627451, b: 0.19607843137254902, a: 1.0 })?;
    m.add("BLUE", Color { r: 0.12156862745098039, g: 0.4666666666666667, b: 0.7058823529411765, a: 1.0 })?;
    m.add("ORANGE", Color { r: 1.0, g: 0.4980392156862745, b: 0.054901960784313725, a: 1.0 })?;
    m.add("GREEN", Color { r: 0.17254901960784313, g: 0.6274509803921569, b: 0.17254901960784313, a: 1.0 })?;
    m.add("RED", Color { r: 0.8392156862745098, g: 0.15294117647058825, b: 0.1568627450980392, a: 1.0 })?;
    m.add("PURPLE", Color { r: 0.5803921568627451, g: 0.403921568627451, b: 0.7411764705882353, a: 1.0 })?;
    m.add("BROWN", Color { r: 0.5490196078431373, g: 0.33725490196078434, b: 0.29411764705882354, a: 1.0 })?;
    m.add("PINK", Color { r: 0.8901960784313725, g: 0.4666666666666667, b: 0.7607843137254902, a: 1.0 })?;
    m.add("GRAY", Color { r: 0.4980392156862745, g: 0.4980392156862745, b: 0.4980392156862745, a: 1.0 })?;
    m.add("OLIVE", Color { r: 0.7372549019607844, g: 0.7411764705882353, b: 0.13333333333333333, a: 1.0 })?;
    m.add("CYAN", Color { r: 0.09019607843137255, g: 0.7450980392156863, b: 0.8117647058823529, a: 1.0 })?;
    m.add("GREY", Color { r: 0.4980392156862745, g: 0.4980392156862745, b: 0.4980392156862745, a: 1.0 })?;
     Ok(())
}




//list of all structs in macroquad: 
/*

    // done adding ? 
    macroquad::prelude::IVec2
    macroquad::prelude::IVec3
    macroquad::prelude::IVec4

    macroquad::prelude::DVec2
    macroquad::prelude::DVec3
    macroquad::prelude::DVec4
    macroquad::prelude::DMat2
    macroquad::prelude::DMat3
    macroquad::prelude::DMat4
    macroquad::prelude::Config
    macroquad::prelude::UVec2
    macroquad::prelude::UVec3
    macroquad::prelude::UVec4
    macroquad::prelude::Vec2
    macroquad::prelude::Vec3
    macroquad::prelude::Vec4
    macroquad::prelude::Color
    macroquad::prelude::Image
    macroquad::prelude::Circle
    macroquad::prelude::Quat
    macroquad::prelude::Rect
    macroquad::prelude::I16Vec2
    macroquad::prelude::I16Vec3
    macroquad::prelude::I16Vec4
    macroquad::prelude::I64Vec2
    macroquad::prelude::I64Vec3
    macroquad::prelude::I64Vec4
    macroquad::prelude::Vec4
    macroquad::prelude::U16Vec2
    macroquad::prelude::U16Vec3
    macroquad::prelude::U16Vec4
    macroquad::prelude::U64Vec2
    macroquad::prelude::U64Vec3
    macroquad::prelude::U64Vec4

    //important to add
    macroquad::prelude::Camera2D  
    macroquad::prelude::Texture2D



    //not important to add
    macroquad::prelude::Camera3D
    macroquad::prelude::Material
    macroquad::prelude::Touch
    macroquad::prelude::Vertex
    macroquad::prelude::Vec3A

    //wtf
    macroquad::prelude::Affine2
    macroquad::prelude::Affine3A
    macroquad::prelude::BVec2
    macroquad::prelude::BVec3
    macroquad::prelude::BVec3A
    macroquad::prelude::BVec4
    macroquad::prelude::DAffine2
    macroquad::prelude::DAffine3

    macroquad::prelude::DQuat
    macroquad::prelude::DrawCylinderParams
    macroquad::prelude::DrawRectangleParams
    macroquad::prelude::DrawSphereParams
    macroquad::prelude::DrawTextureParams

    macroquad::prelude::Font
    macroquad::prelude::GlPipeline
    macroquad::prelude::InternalGlContext
    macroquad::prelude::IVec2
    macroquad::prelude::IVec3
    macroquad::prelude::IVec4
    macroquad::prelude::Mat2
    macroquad::prelude::Mat3
    macroquad::prelude::Mat3A
    macroquad::prelude::Mat4
    macroquad::prelude::MaterialParams
    macroquad::prelude::Mesh
    macroquad::prelude::PipelineParams
    macroquad::prelude::QuadGl
    macroquad::prelude::RectOffset
    macroquad::prelude::RenderPass
    macroquad::prelude::RenderTarget
    macroquad::prelude::TextDimensions
    macroquad::prelude::TextParams
    macroquad::prelude::UniformDesc

 
*/

