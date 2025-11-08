//use macroquad::prelude as mq;
//use std::sync::Arc;


//pub trait Drawable {
//    fn draw(&self);
//}

//pub struct Rectangle {
//    pub x: f32,
//    pub y: f32,
//    pub width: f32,
//    pub height: f32,
//    pub texture: Option<Arc<mq::Texture2D>>,
//}

//impl Rectangle {
//    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
//        Self {
//            x,
//            y,
//            width,
//            height,
//            texture: None,     
//        }
//    }
    

//}


//impl Drawable for Rectangle {
//    fn draw(&self) {
//        //draw code     
//    }  
//}


//impl Player {
//    fn new(rect: Rectangle) -> Self {
//        Self {
//            inner: Arc::new(Mutex::new(rect)),
//        }
//    }
//}


//impl Deref for Player {
//    type Target = Rectangle;
//    fn deref(&self) -> &Self::Target {
//        &self.inner.lock().unwrap()
//    }
//}

//impl DerefMut for Player {
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        &mut self.inner.lock().unwrap()
//    }
//}




