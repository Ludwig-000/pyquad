
use glam::BVec2 as gl;

use pyo3::prelude::*;
use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer,derive::*};


#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy, PartialEq,Debug,Eq, Hash)]
pub struct BVec2 {
    #[pyo3(get, set)]
    pub x: bool,
    #[pyo3(get, set)]
    pub y: bool,
}

impl BVec2 {
    // Const constructor for compile-time constants
    #[inline(always)]
    pub const fn const_new(x: bool, y: bool) -> Self {
    BVec2 { x, y }
    }


    // Const constructor for splat values
    #[inline(always)]
    pub const fn splat(value: bool) -> Self {
    BVec2 { x: value, y: value }
    }

    /// Creates a new vector mask from a bool array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [bool; 2]) -> Self {
        Self { x: a[0],y :  a[1] }
    }
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

#[gen_stub_pymethods]
#[pymethods]
impl BVec2 {
    
    #[new]
    pub fn new(x: bool, y: bool) -> Self {
    Self { x, y }
    }

    /// All false.
    #[classattr]
    fn FALSE() -> BVec2 {
        Self::splat(false)
    }

    /// All true.
    #[classattr]
    fn TRUE() -> BVec2 {
        Self::splat(true)
    }
    
    #[inline]
    pub fn bitmask(&self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.x || self.y
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(&self) -> bool {
        self.x && self.y
    }

    #[inline]
    pub fn test(&self, index: usize) -> bool {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    pub fn into_bool_array(&self) -> [bool; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub fn into_u32_array(&self) -> [u32; 2] {
        [MASK[self.x as usize], MASK[self.y as usize]]
    }

    #[inline]
    pub fn bitand(&self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }

    #[inline]
    pub fn bitor(&self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
        }
    }

    #[inline]
    fn bitxor(&self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }

    #[inline]
    fn not(&self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
    fn __str__(&self) -> String {
        format!("{}, {}", self.x, self.y)
    }
}






impl From<BVec2> for gl {
    fn from(v: BVec2) -> Self {
        gl::new(v.x, v.y)
    }
}


impl From<gl> for BVec2 {
    fn from(v: gl) -> Self {
        BVec2 {
        x: v.x,
        y: v.y,
        }
    }
}