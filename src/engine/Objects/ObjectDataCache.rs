use macroquad::prelude as mq;

#[derive(Clone,Copy,PartialEq)]
pub struct ThreeDObjCache{
    // an object's data can only be cached, if it cannot be influenced by anything else.
    // F.E.: gravity.
    pub can_be_cached: bool,

    pub location: mq::Vec3,
    pub rotation: mq::Vec3,
    pub scale: mq::Vec3,
    pub color: mq::Color,

}
impl ThreeDObjCache{
    pub fn new(cacheable: bool, location: mq::Vec3, rotation: mq::Vec3, scale: mq::Vec3, color: mq::Color)-> ThreeDObjCache{
        ThreeDObjCache{
            can_be_cached: cacheable,
            location,
            rotation,
            scale,
            color,
        }
    }
    pub fn no_cache()-> ThreeDObjCache{
        ThreeDObjCache{
            can_be_cached: false,
            location: mq::Vec3::NAN,
            rotation: mq::Vec3::NAN,
            scale: mq::Vec3::NAN,
            color: mq::BLACK,
        }
    }
}

#[derive(Clone,Copy,PartialEq)]
pub struct TwoDObjCache{
    // an object's data can only be cached, if it cannot be influenced by anything else.
    // F.E.: gravity.
    pub can_be_cached: bool,

    pub location: mq::Vec2,
    pub rotation: mq::Vec2,
    pub scale: mq::Vec2,
    pub color: mq::Color,
}
impl TwoDObjCache {
    pub fn new(cacheable: bool, location: mq::Vec2, rotation: mq::Vec2, scale: mq::Vec2, color: mq::Color)-> TwoDObjCache{
        TwoDObjCache{
            can_be_cached: cacheable,
            location,
            rotation,
            scale,
            color,
        }
    }
    pub fn no_cache()-> TwoDObjCache{
        TwoDObjCache { can_be_cached: false, location: mq::Vec2::NAN, rotation: mq::Vec2::NAN, scale: mq::Vec2::NAN, color: mq::BLACK }
    }
}