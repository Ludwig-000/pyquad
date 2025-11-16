use macroquad::prelude as mq;
use std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};

pub struct Rectangle {
   pub pos: mq::Vec2,
   pub size: mq::Vec2,
   pub rot: f32,

   pub color: mq::Color,
   pub texture: Option<mq::Texture2D>,

}
