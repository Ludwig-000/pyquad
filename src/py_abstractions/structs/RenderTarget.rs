use macroquad::prelude as mq;
use pyo3::{pyclass, pyfunction,pymethods};
use pyo3_stub_gen::derive::gen_stub_pyfunction;
use crate::py_abstractions::structs::Textures_and_Images::Texture2D;

use pyo3_stub_gen::{define_stub_info_gatherer,derive::*};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Debug)]
pub struct RenderTarget {
   renderTarget:  mq::RenderTarget,
}


impl From<mq::RenderTarget> for RenderTarget{
    fn from(r: mq::RenderTarget) -> Self {
        RenderTarget{ renderTarget: r }
    }
}


impl From<RenderTarget> for mq::RenderTarget{
    fn from(r: RenderTarget) -> Self {
        r.renderTarget
    }
}








/// A shortcut to create a render target with no depth buffer and `sample_count: 4`
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (width, height, params = None))]
pub fn render_target_msaa(width: u32, height: u32, params: Option<RenderTargetParams>) -> RenderTarget {
    match params{
        Some(p) => mq::render_target_ex(width, height, p.into()).into(),
        None => mq::render_target(width, height).into(),
    }

}

#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (width, height, params = None))]
pub fn render_target(width: u32, height: u32, params: Option<RenderTargetParams>) -> RenderTarget {
    match params {
        Some(p) => mq::render_target_ex(width, height, p.into()).into(),
        None => mq::render_target(width, height).into(),
    }
}





#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, PartialEq, Debug)]
pub struct RenderTargetParams {
    /// 1 means no multi sampling.
    /// Note that sample_count > 1 is not supported on GL2, GLES2 and WebGL1
    #[pyo3(get, set)]
    pub sample_count: i32,

    /// depth: true creates a depth render target attachment and allows
    /// such a render target being used for a depth-testing cameras
    #[pyo3(get, set)]
    pub depth: bool,
}

#[gen_stub_pymethods]
#[pymethods]
impl RenderTargetParams {

    #[new]
    #[pyo3(signature = (sample_count = 1, depth = false))]
    pub fn new(sample_count: i32, depth: bool) -> Self {
        Self { sample_count, depth }
    }

}

impl From<mq::RenderTargetParams> for RenderTargetParams{
    fn from(r: mq::RenderTargetParams) -> Self {
        RenderTargetParams { sample_count: r.sample_count, depth: r.depth}
    }
}



impl From<RenderTargetParams> for mq::RenderTargetParams{
    fn from(r: RenderTargetParams) -> Self {
        mq::RenderTargetParams {sample_count: r.sample_count, depth: r.depth}
    }
}
