use macroquad::prelude as mq;
use std::sync::Arc;

///
/// This file is outdated.
/// 
/// 

pub trait Drawable {
    fn draw(&self);
}




pub trait Collidable{
    fn collides_with(&self, other: &Rectangle)-> bool;
}





pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Option<mq::Color>,
    pub texture: Option<Arc<mq::Texture2D>>,
    pub animation: Option<Animation>,
    pub post_processing: Option<PostProcessingFunction>,
}

pub enum PostProcessingFunction {
    FunctionA,
    FunctionB,
    
}
// Implement calling for the enum
impl PostProcessingFunction {
    fn call(&self, rect1: &Rectangle, rect2: &Rectangle) {
        match self {
            PostProcessingFunction::FunctionA => function_a(rect1, rect2),
            PostProcessingFunction::FunctionB => function_b(rect1, rect2),
        }
    }
}
// Predefined functions
fn function_a(rect1: &Rectangle, rect2: &Rectangle) {
    println!("Function A: Rect1.x = {}, Rect2.x = {}", rect1.x, rect2.x);
}

fn function_b(rect1: &Rectangle, rect2: &Rectangle) {
    println!("Function B: Special collision Rect1.x = {}, Rect2.x = {}", rect1.x, rect2.x);
}



impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Option<mq::Color>,) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color: color,        
            texture: None,     
            animation: None,
            post_processing: None,
        }
    }
    pub fn with_texture(mut self, texture: Arc<mq::Texture2D>) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self

    }
    //pub fn with_post_processing(mut self, post_processing: fn(rect1: &Rectangle, rect2: &Rectangle)) -> Self {
    //    self.post_processing = Some(post_processing);
    //    self

    //}

}



impl Drawable for Rectangle {
    fn draw(&self) {
        match &self.animation {
            Some(animation) => {
             


                mq::draw_texture_ex(&*animation.frames[animation.current_frame], self.x, self.y, mq::WHITE, mq::DrawTextureParams {
                    dest_size: Some(mq::vec2(self.width, self.height)), // ✅ Corrected size
                    ..Default::default()
                });


            },
            None => {
                
                match &self.texture {
                    Some(texture) => {
                        
                        mq::draw_texture_ex(&*texture, self.x, self.y, mq::WHITE, mq::DrawTextureParams {
                            dest_size: Some(mq::vec2(self.width, self.height)), // ✅ Corrected size
                            ..Default::default()
                        });
                    },
                    None => {
                
                mq::draw_rectangle(self.x, self.y, self.width, self.height, self.color.unwrap_or(mq::PURPLE));
            }
        }
            }
        }
        
    }
    
}

impl Collidable for Rectangle {

    fn collides_with(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

}

pub struct Triangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Option<mq::Color>,
    pub texture: Option<Arc<mq::Texture2D>>,
   
}

pub struct Circle{
   pub x: f32, 
   pub y: f32, 
   pub r: f32, 
   pub color: Option<mq::Color>,
   pub texture: Option<Arc<mq::Texture2D>>,
   pub animation: Option<Animation>,
  
}


impl Circle {
    pub fn new(x: f32, y: f32, r: f32,  color: Option<mq::Color>,) -> Self {
        Self {
            x,
            y,
            r,
            color: color,        
            texture: None,     
            animation: None,
            
        }
    }
    pub fn with_texture(mut self, texture: Arc<mq::Texture2D>) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self

    }

}

impl Drawable for Circle {
    fn draw(&self) {
        match &self.animation {
            Some(animation) => {
             

                // reshape into sphere
                //mq::draw_texture_ex(&*animation.frames[animation.current_frame], self.x, self.y, mq::WHITE, mq::DrawTextureParams {
                //    dest_size: Some(mq::vec2(self.width, self.height)), // ✅ Corrected size
                //    ..Default::default()
                //});


            },
            None => {
                
                match &self.texture {
                    Some(texture) => {

                        // reshape into sphere
                        //mq::draw_texture_ex(&*texture, self.x, self.y, mq::WHITE, mq::DrawTextureParams {
                        //    dest_size: Some(mq::vec2(self.r, self.r)), // ✅ Corrected size
                        //    ..Default::default()
                        //});
                    },
                    None => {
                
                mq::draw_circle(self.x, self.y, self.r, self.color.unwrap_or(mq::PURPLE));
            }
        }
            }
        }
        
    }
    
}



impl Drawable for Triangle {
    fn draw(&self) {
        let color = self.color.unwrap_or(mq::WHITE);
        mq::draw_rectangle(self.x, self.y, self.width, self.height, color);
    }
}

pub struct Animation  {
    pub frames: Vec<Arc<mq::Texture2D>>,
    pub pacing: f64,
    pub current_frame: usize,
    pub last_frame_time: f64,
    
}



impl  Animation  {
    pub fn new(frames: Vec<Arc<mq::Texture2D>>, pacing: f64 ) -> Self {
        Self {

          frames: frames,
          pacing: pacing,
          current_frame: 0,
          last_frame_time: mq::get_time(),
        }
    
    }
}    



























