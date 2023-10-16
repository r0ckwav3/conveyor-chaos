use ggez::{
    GameResult,
    GameError,
    Context,
    graphics::{self,DrawParam,Transform,Image},
    glam::{Mat2, vec2},
    mint::Point2
};

#[derive(Clone, Copy, PartialEq)]
pub struct BoardPos {
    pub x: i32,
    pub y: i32
}

// takes in a DrawParam and adjusts the dest so that that the original dest point is now the actual top left corner
// assumes offset is 0, causes unexpected behavior otherwise
// designed to work with right angles, but technically works otherwise
// probably destructive idk
pub fn rot_fix(dp: &mut DrawParam, w: f32, h:f32) -> GameResult<DrawParam>{
    let tf = dp.transform;

    if let Transform::Values { dest, rotation, scale: _, offset: _ } = tf{
        let rot_mat = Mat2::from_angle(rotation);
        let mut points = [
            vec2(0.0,0.0),
            vec2(w  ,0.0),
            vec2(0.0,h),
            vec2(w  ,h),
        ];

        let mut min_x = 0.0;
        let mut min_y = 0.0;
        for i in 0..4{
            points[i] = rot_mat * points[i];
            if points[i].x < min_x{
                min_x = points[i].x;
            }
            if points[i].y < min_y{
                min_y = points[i].y;
            }
        }

        Ok(dp.dest(Point2::<f32>{x: dest.x-min_x, y: dest.y-min_y}))
    }else{
        Err(GameError::CustomError("Cannot use rot_fix on matrix transform".to_string()))
    }
}

// multiply the whole image by a given alpha value/
// nondestructive
pub fn mult_alpha(ctx: &mut Context, im: Image, alpha: f32) -> GameResult<Image>{
    let color_format = ctx.gfx.surface_format();
    let image = Image::new_canvas_image(
        ctx, color_format,
        im.width(),
        im.height(),
        1
    );
    let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), graphics::Color::new(1.0, 1.0, 1.0, alpha));
    image_canvas.set_blend_mode(graphics::BlendMode::MULTIPLY);
    image_canvas.draw(&im, DrawParam::default());

    image_canvas.finish(ctx)?;
    Ok(image)

}
