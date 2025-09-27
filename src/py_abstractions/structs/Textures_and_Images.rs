use pyo3::prelude::*;
use pyo3::wrap_pyfunction; 
use macroquad::prelude as mq;
use std::sync::Arc;
use crate::py_abstractions::py_structs::*;
use std::option;
use std::sync::mpsc;
use crate::COMMAND_QUEUE;
use crate::Command;
use pyo3::exceptions::PyValueError;use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer,derive::*};

/// Image, data stored in CPU memory
#[gen_stub_pyclass]
#[pyclass(name = "Image")]
#[derive(Clone)]
pub struct Image {
    #[pyo3(get, set)]
    pub bytes: Vec<u8>, 
    #[pyo3(get, set)]
    pub width: u16, 
    #[pyo3(get, set)]
    pub height: u16,
}

#[gen_stub_pymethods]
#[pymethods]
impl Image {

    #[new]
    pub fn new(bytes: Vec<u8>, width: u16, height: u16, ) -> PyResult<Self> {
        if bytes.len() != (width as usize) * (height as usize) * 4 {
            return Err(PyErr::new::<PyValueError, _>(
                "Invalid image data size: expected width * height * 4 bytes",
            ));
        }

        Ok(Self { bytes, width, height })
    }

    #[staticmethod]
    pub fn empty() -> Image {
        Image {
            width: 0,
            height: 0,
            bytes: vec![],
        }
    }

    /// Creates an Image filled with the provided [Color].
    #[staticmethod]
    pub fn gen_image_color(width: u16, height: u16, color: Color) -> Image {
        let mut bytes = vec![0; width as usize * height as usize * 4];
        for i in 0..width as usize * height as usize {
            bytes[i * 4 + 0] = (color.r * 255.) as u8;
            bytes[i * 4 + 1] = (color.g * 255.) as u8;
            bytes[i * 4 + 2] = (color.b * 255.) as u8;
            bytes[i * 4 + 3] = (color.a * 255.) as u8;
        }
        Image {
            width,
            height,
            bytes,
        }
    }

    /// Updates this image from a slice of [Color]s.
    pub fn update(&mut self, colors: Vec<Color>) {
        assert!(self.width as usize * self.height as usize == colors.len());

        for i in 0..colors.len() {
            self.bytes[i * 4] = (colors[i].r * 255.) as u8;
            self.bytes[i * 4 + 1] = (colors[i].g * 255.) as u8;
            self.bytes[i * 4 + 2] = (colors[i].b * 255.) as u8;
            self.bytes[i * 4 + 3] = (colors[i].a * 255.) as u8;
        }
    }

    /// Returns this image's data as a slice of 4-byte arrays.
    pub fn get_image_data(&self) -> Vec<[u8; 4]> {
        assert!(self.width as usize * self.height as usize * 4 == self.bytes.len());

        // SAFETY: We're converting a slice of u8 to a slice of [u8; 4].
        // This is safe because we have asserted that the total number of bytes
        // is a multiple of 4, and the memory layout of u8 is the same as [u8; 4]
        // for alignment purposes.
        let image_slice = unsafe {
            std::slice::from_raw_parts(
                self.bytes.as_ptr() as *const [u8; 4],
                self.width as usize * self.height as usize,
            )
        };

        image_slice.to_vec()
    }

    /// Modifies a pixel's color in this image.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        // Assert that the x and y coordinates are within the image boundaries.
        assert!(x < self.width as u32);
        assert!(y < self.height as u32);

        // Calculate the starting byte index for the given pixel.
        // Each pixel takes up 4 bytes (r, g, b, a).
        let index = (y * self.width as u32 + x) as usize * 4;

        // Convert the float color values (assuming a 0.0 to 1.0 range) to u8 (0 to 255).
        // The cast truncates the fractional part, so we multiply by 255.0 first.
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let a = (color.a * 255.0) as u8;

        // Set the individual byte values in the underlying 'bytes' vector.
        self.bytes[index] = r;
        self.bytes[index + 1] = g;
        self.bytes[index + 2] = b;
        self.bytes[index + 3] = a;
    }

    /// Returns a pixel [Color] from this image.
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        assert!(x < self.width as u32);
        assert!(y < self.height as u32);

        let index = (y * self.width as u32+ x) as usize * 4;
        
        let r = self.bytes[index];
        let g = self.bytes[index + 1];
        let b = self.bytes[index + 2];
        let a = self.bytes[index + 3];
        
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }


    #[staticmethod]
    pub fn from_file( path: String) -> PyResult<Image> {
        let (sender, receiver) = mpsc::sync_channel(1);
        
        let command = Command::LoadImage {
            path,
            sender,
        };
        COMMAND_QUEUE.push(command);

        let result = receiver
            .recv()
            .map_err(|e| PyValueError::new_err(format!("{}", e)))?;


        match result {
            Ok(mq_image) => Ok(Image { bytes: mq_image.bytes, width: mq_image.width, height: mq_image.height}),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }

    
    
}


/// Texture, data stored in GPU memory
#[gen_stub_pyclass]
#[pyclass(name = "Texture2D")]
#[derive(Clone, Debug, PartialEq)]
pub struct Texture2D {
   pub texture: mq::Texture2D,
}

#[gen_stub_pymethods]
#[pymethods]
impl Texture2D {
   
    #[staticmethod]
    pub fn from_image(image: PyRef<Image>) -> Texture2D {
        let inner_im = mq::Image {
            bytes: image.bytes.clone(),
            width: image.width,
            height: image.height,
        };
        let imagePointer = Arc::new(inner_im);
        
        let (sender, receiver) = mpsc::sync_channel(1);
        
        let command = Command::ImgToTexture {
            image: imagePointer.clone(),
            sender,
        };

        COMMAND_QUEUE.push(command);

        let mq_texture = receiver.recv().unwrap();
        let ourTexture=  Texture2D{
            texture: mq_texture,
        };
        ourTexture
        
    }

}
    