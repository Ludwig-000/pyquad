// // experimental

// use glam::BVec2 as gl;

// use pyo3::prelude::*;
// use pyo3_stub_gen::derive::*;


// /// A Boolean Vector with 2 elements: x,y.
// #[gen_stub_pyclass]
// #[pyclass(frozen)]
// #[derive(Clone, Copy, PartialEq,Debug,Eq, Hash)]
// pub struct Fozen_BVec2 {
//     #[pyo3(get)]
//     pub x: bool,
//     #[pyo3(get)]
//     pub y: bool,
// }

// impl Fozen_BVec2 {
//     // Const constructor for compile-time constants
//     #[inline(always)]
//     pub const fn const_new(x: bool, y: bool) -> Self {
//         Fozen_BVec2 { x, y }
//     }


//     // Const constructor for splat values
//     #[inline(always)]
//     pub const fn splat(value: bool) -> Self {
//         Fozen_BVec2 { x: value, y: value }
//     }

//     /// Creates a new vector mask from a bool array.
//     #[inline]
//     #[must_use]
//     pub const fn from_array(a: [bool; 2]) -> Self {
//         Self { x: a[0],y :  a[1] }
//     }
// }

// const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

// #[gen_stub_pymethods]
// #[pymethods]
// impl Fozen_BVec2 {
    
//     #[new]
//     pub fn new(x: bool, y: bool) -> Self {
//         Self { x, y }
//     }
    
//     #[setter]
//     pub fn set_x(&self, x: bool)-> Fozen_BVec2{
//         Fozen_BVec2{ x, y: self.y }
//     }
//     /// All false.
//     #[classattr]
//     fn FALSE() -> Fozen_BVec2 {
//         Self::splat(false)
//     }

//     /// All true.
//     #[classattr]
//     fn TRUE() -> Fozen_BVec2 {
//         Self::splat(true)
//     }
    
//     #[inline]
//     pub fn bitmask(&self) -> u32 {
//         (self.x as u32) | (self.y as u32) << 1
//     }

//     /// Returns true if any of the elements are true, false otherwise.
//     #[inline]
//     pub fn any(&self) -> bool {
//         self.x || self.y
//     }

//     /// Returns true if all the elements are true, false otherwise.
//     #[inline]
//     pub fn all(&self) -> bool {
//         self.x && self.y
//     }

//     /// Returns the value of index 1 or index 2.
//     #[inline]
//     pub fn test(&self, index: usize) -> bool {
//         match index {
//             0 => self.x,
//             1 => self.y,
//             _ => panic!("index out of bounds"),
//         }
//     }

//     #[inline]
//     pub fn into_bool_array(&self) -> [bool; 2] {
//         [self.x, self.y]
//     }

//     #[inline]
//     pub fn into_u32_array(&self) -> [u32; 2] {
//         [MASK[self.x as usize], MASK[self.y as usize]]
//     }

//     #[inline]
//     pub fn bitand(&self, rhs: Self) -> Self {
//         Self {
//             x: self.x & rhs.x,
//             y: self.y & rhs.y,
//         }
//     }

//     #[inline]
//     pub fn bitor(&self, rhs: Self) -> Self {
//         Self {
//             x: self.x | rhs.x,
//             y: self.y | rhs.y,
//         }
//     }

//     #[inline]
//     fn bitxor(&self, rhs: Self) -> Self {
//         Self {
//             x: self.x ^ rhs.x,
//             y: self.y ^ rhs.y,
//         }
//     }

//     #[inline]
//     fn not(&self) -> Self {
//         Self {
//             x: !self.x,
//             y: !self.y,
//         }
//     }

//     fn __str__(&self) -> String {
//         format!("{}, {}", self.x, self.y)
//     }
// }






// impl From<Fozen_BVec2> for gl {
//     fn from(v: Fozen_BVec2) -> Self {
//         gl::new(v.x, v.y)
//     }
// }


// impl From<gl> for Fozen_BVec2 {
//     fn from(v: gl) -> Self {
//         Fozen_BVec2 {
//         x: v.x,
//         y: v.y,
//         }
//     }
// }