use ggez::{
    glam,
    graphics::{self, Image, Text, TextFragment},
    Context, GameResult
};

use crate::constants::*;
use crate::helpers::*;

pub struct PopupBox{
    width: f32,
    height: f32,
    margin_x: f32,
    margin_y: f32,
    text: Text
    // scroll: f32
}

impl PopupBox{
    pub fn new(width: f32, height: f32, content: impl Into<TextFragment>) -> PopupBox{
        let text: Text = Text::new(content);
        let mut this = PopupBox{
            width,
            height,
            margin_x: POPUP_MARGIN_X,
            margin_y: POPUP_MARGIN_Y,
            text
        };
        this.setup_text();
        this
    }

    fn setup_text(&mut self){
        self.text.set_bounds(glam::vec2(
                self.width - (self.margin_x * 2.0),
                self.height - (self.margin_y * 2.0)))
            .set_font(POPUP_FONT)
            .set_scale(POPUP_SCALE);
    }

    pub fn set_text(&mut self, content: impl Into<Text>){
        self.text = content.into();
    }

    pub fn get_width(&self) -> f32{
        self.width
    }

    pub fn get_height(&self) -> f32{
        self.height
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<Image> {
        let color_format = ctx.gfx.surface_format();

        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            self.width.ceil() as u32,
            self.height.ceil() as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), TRANSPARENT_COLOR);

        image_canvas.draw(
            &graphics::Mesh::new_rounded_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, self.width, self.height),
                POPUP_CORNER_RAD,
                POPUP_BG_COLOR
            )?,
            glam::vec2(0.0, 0.0)
        );

        image_canvas.draw(
            &self.text,
            glam::vec2(self.margin_x, self.margin_y)
        );

        image_canvas.finish(ctx)?;
        Ok(image)
    }

}
