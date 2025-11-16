use macroquad::prelude as mq;

pub struct Circle{
    pub center: mq::Vec2,
    pub radius: f32,
    pub color: mq::Color,
    pub texture: Option<mq::Texture2D>,
}