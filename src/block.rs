use ggez::{
    glam,
    graphics,
    Context, GameResult
};

use crate::helpers::*;
use crate::constants::*;

pub struct Block {
    pos: BoardPos
}

pub struct BlockObject{
    blocks: Vec<Block>
}

impl BlockObject{
    pub fn new() -> BlockObject{
        BlockObject{
            blocks: Vec::new()
        }
    }

    pub fn from_blocklist(blocks: Vec<Block>) -> BlockObject{
        BlockObject{
            blocks
        }
    }

    // pub fn merge(a:BlockObject, b:BlockObject) -> BlockObject{}
}

impl Block{
    pub fn new(pos: BoardPos) -> Block{
        Block{
            pos
        }
    }

    pub fn draw(ctx: &mut Context, tilesize: f32, nhood: [[bool; 3]; 3]) -> GameResult<graphics::Image>{
        let color_format = ctx.gfx.surface_format();
        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            tilesize.ceil() as u32,
            tilesize.ceil() as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), graphics::Color::from_rgba(0,0,0,0));

        let base_mesh = graphics::Mesh::new_rounded_rectangle(
            ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, tilesize, tilesize),
            tilesize*BLOCK_ROUNDNESS,
            BLOCK_COLOR
        )?;

        let clear_base_mesh = graphics::Mesh::new_rounded_rectangle(
            ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, tilesize, tilesize),
            tilesize*BLOCK_ROUNDNESS,
            graphics::Color::new(0.0, 0.0, 0.0, 0.0)
        )?;

        let corner_mesh = graphics::Mesh::new_rectangle(
            ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, tilesize*BLOCK_ROUNDNESS, tilesize*BLOCK_ROUNDNESS),
            BLOCK_COLOR
        )?;

        let corner_offset = tilesize*(1.0-BLOCK_ROUNDNESS);

        if nhood[1][1]{ // if the center tile exists
            // draw the base
            image_canvas.draw(&base_mesh, graphics::DrawParam::default());
            // corners
            if nhood[0][0] || nhood[1][0] || nhood[0][1]{ // top left
                image_canvas.draw(&corner_mesh, glam::vec2(0.0, 0.0));
            }
            if nhood[0][2] || nhood[0][1] || nhood[1][2]{ // top right
                image_canvas.draw(&corner_mesh, glam::vec2(corner_offset, 0.0));
            }
            if nhood[2][2] || nhood[1][2] || nhood[2][1]{ // bottom right
                image_canvas.draw(&corner_mesh, glam::vec2(corner_offset, corner_offset));
            }
            if nhood[2][0] || nhood[1][0] || nhood[2][1]{ // bottom left
                image_canvas.draw(&corner_mesh, glam::vec2(0.0, corner_offset));
            }
        }else{ // center tile does not exist
            // corners
            if nhood[1][0] && nhood[0][1]{ // top left
                image_canvas.draw(&corner_mesh, glam::vec2(0.0, 0.0));
            }
            if nhood[0][1] && nhood[1][2]{ // top right
                image_canvas.draw(&corner_mesh, glam::vec2(corner_offset, 0.0));
            }
            if nhood[1][2] && nhood[2][1]{ // bottom right
                image_canvas.draw(&corner_mesh, glam::vec2(corner_offset, corner_offset));
            }
            if nhood[1][0] && nhood[2][1]{ // bottom left
                image_canvas.draw(&corner_mesh, glam::vec2(0.0, corner_offset));
            }
            // subtract the base
            image_canvas.set_blend_mode(graphics::BlendMode::MULTIPLY);
            image_canvas.draw(&clear_base_mesh, graphics::DrawParam::default());
        }

        image_canvas.finish(ctx)?;

        Ok(image)
    }
}
