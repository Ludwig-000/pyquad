//use macroquad::prelude as mq;
//use std::sync::{Arc, Mutex};
//use std::ops::{Deref, DerefMut};

//pub struct Rectangle<'a> {
//    pub x: f32,
//    pub y: f32,
//    pub width: f32,
//    pub height: f32,
//    pub color: Option<mq::Color>,
//    pub texture: Option<&'a mq::Texture2D>,
//    pub animation: Option<Animation>,
//}

//impl<'a> Rectangle<'a> {
//    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Option<mq::Color>,) -> Self {
//        Self {
//            x,
//            y,
//            width,
//            height,
//            color: color,        
//            texture: None,     
//            animation: None,
          
//        }
//    }
//    pub fn with_texture(mut self, texture: &'a mq::Texture2D) -> Self {
//        self.texture = Some(texture);
//        self
//    }

//    pub fn with_animation(mut self, animation: Animation) -> Self {
//        self.animation = Some(animation);
//        self

//    }

//}


//// Define a trait for drawable shapes
//pub trait Drawable {
//    fn draw(&self);
//}


//pub trait Resizeable {
//    fn resize(&mut self, width: f32, height: f32);
//}

////resized rectangles and reshapes their textures if exist
//impl<'a> Resizeable for Rectangle<'a> {
//    //resized rectangles and reshapes their textures if exist
//    fn resize(&mut self, width: f32, height: f32) {
        
//      self.width= width;
//      self.height=height;
        
//    }
//}

//impl<'a> Drawable for Rectangle<'a> {
//    fn draw(&self) {
      
//        match &self.texture {
//            Some(texture) => {
                
//                mq::draw_texture_ex(texture, self.x, self.y, mq::WHITE, mq::DrawTextureParams {
//                    dest_size: Some(mq::vec2(self.width, self.height)), // ✅ Corrected size
//                    ..Default::default()
//                });
//            },
//            None => {
                
//                mq::draw_rectangle(self.x, self.y, self.width, self.height, self.color.unwrap_or(mq::PURPLE));
//            }
//        }
//    }
//}



//impl Player {
//    fn new(rect: Rectangle) -> Self {
//        Self {
//            inner: Arc::new(Mutex::new(rect)),
//        }
//    }
//}

//// Implement Deref to allow direct access to the underlying Rectangle
//impl Deref for Player {
//    type Target = Rectangle;

//    fn deref(&self) -> &Self::Target {
//        // Lock the Mutex here and return a reference to Rectangle
//        &self.inner.lock().unwrap()
//    }
//}

//// Implement DerefMut to allow direct mutable access to the underlying Rectangle
//impl DerefMut for Player {
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        // Lock the Mutex here and return a mutable reference to Rectangle
//        &mut self.inner.lock().unwrap()
//    }
//}