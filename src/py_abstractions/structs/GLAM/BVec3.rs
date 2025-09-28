
use glam::BVec3 as gl;

use pyo3::prelude::*;
use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer,derive::*};


#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy, PartialEq,Debug,Eq, Hash)]
pub struct BVec3 {
    #[pyo3(get, set)]
    pub x: bool,
    #[pyo3(get, set)]
    pub y: bool,
    #[pyo3(get, set)]
    pub z: bool,
}

impl BVec3 {
    // Const constructor for compile-time constants
    #[inline(always)]
    pub const fn const_new(x: bool, y: bool, z: bool) -> Self {
    BVec3 { x, y, z }
    }


    // Const constructor for splat values
    #[inline(always)]
    pub const fn splat(value: bool) -> Self {
    BVec3 { x: value, y: value, z: value }
    }

    /// Creates a new vector mask from a bool array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [bool; 3]) -> Self {
        Self { x: a[0],y :  a[1],z:  a[2]   }
    }
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

#[gen_stub_pymethods]
#[pymethods]
impl BVec3 {
    #[new]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
    Self { x, y, z }
    }

    /// All false.
    #[classattr]
    fn FALSE() -> BVec3 {
        Self::splat(false)
    }

    /// All true.
    #[classattr]
    fn TRUE() -> BVec3 {
        Self::splat(true)
    }

    
    #[inline]
    pub fn bitmask(&self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.x || self.y || self.z
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(&self) -> bool {
        self.x && self.y && self.z
    }

    #[inline]
    pub fn test(&self, index: usize) -> bool {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    pub fn into_bool_array(&self) -> [bool; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    pub fn into_u32_array(&self) -> [u32; 3] {
        [
            MASK[self.x as usize],
            MASK[self.y as usize],
            MASK[self.z as usize],
        ]
    }

    #[inline]
    pub fn bitand(&self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
        }
    }

    #[inline]
    pub fn bitor(&self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
        }
    }

    #[inline]
    fn bitxor(&self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
        }
    }

    #[inline]
    fn not(&self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }
    fn __str__(&self) -> String {
        format!("{}, {}, {}", self.x, self.y, self.z)
    }
}






impl From<BVec3> for gl {
    fn from(v: BVec3) -> Self {
        gl::new(v.x, v.y, v.z)
    }
}


impl From<gl> for BVec3 {
    fn from(v: gl) -> Self {
        BVec3 {
        x: v.x,
        y: v.y,
        z: v.z,
        }
    }
}