use ggez::{
    GameResult,
    GameError,
    graphics::{DrawParam,Transform},
    glam::{Mat2, vec2},
    mint::Point2
};

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
