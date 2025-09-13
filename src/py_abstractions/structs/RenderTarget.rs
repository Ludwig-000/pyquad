//use macroquad::prelude as mq;
//use crate::py_abstractions::structs::Textures_and_Images::Texture2D;



//#[derive(Clone, Debug)]
//pub struct RenderTarget {
//    pub texture: Texture2D,
//    pub render_pass: RenderPass,
//}

///// A shortcut to create a render target with no depth buffer and `sample_count: 4`
//pub fn render_target_msaa(width: u32, height: u32) -> RenderTarget {
//    render_target_ex(
//        width,
//        height,
//        RenderTargetParams {
//            sample_count: 4,
//            ..Default::default()
//        },
//    )
//}

//pub fn render_target_ex(width: u32, height: u32, params: RenderTargetParams) -> RenderTarget {
//    let context = get_context();

//    let color_texture = get_quad_context().new_render_texture(miniquad::TextureParams {
//        width,
//        height,
//        sample_count: params.sample_count,
//        ..Default::default()
//    });
//    let depth_texture = if params.depth {
//        Some(
//            get_quad_context().new_render_texture(miniquad::TextureParams {
//                width,
//                height,
//                format: miniquad::TextureFormat::Depth,
//                sample_count: params.sample_count,
//                ..Default::default()
//            }),
//        )
//    } else {
//        None
//    };
//    let render_pass;
//    let texture;
//    if params.sample_count != 0 {
//        let color_resolve_texture =
//            get_quad_context().new_render_texture(miniquad::TextureParams {
//                width,
//                height,
//                ..Default::default()
//            });
//        render_pass = get_quad_context().new_render_pass_mrt(
//            &[color_texture],
//            Some(&[color_resolve_texture]),
//            depth_texture,
//        );
//        texture = color_resolve_texture;
//    } else {
//        render_pass = get_quad_context().new_render_pass_mrt(&[color_texture], None, depth_texture);
//        texture = color_texture;
//    }

//    let texture = Texture2D {
//        texture: context.textures.store_texture(texture),
//    };

//    let render_pass = RenderPass {
//        color_texture: texture.clone(),
//        depth_texture: None,
//        render_pass: Arc::new(render_pass),
//    };
//    RenderTarget {
//        texture,
//        render_pass,
//    }
//}
///// A shortcut to create a render target with sample_count: 1 and no depth buffer
//pub fn render_target(width: u32, height: u32) -> RenderTarget {
//    render_target_ex(width, height, RenderTargetParams::default())
//}
//pub fn rendert(){
//	mq::RenderTarget
//}