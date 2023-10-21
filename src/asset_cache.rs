use std::collections::HashMap;
use std::sync::Mutex;
use std::path::Path;
use std::fs;

use ggez::{
    glam,
    graphics::{Image, Canvas, DrawParam},
    GameResult,
    Context
};
use once_cell::sync::Lazy;

use crate::constants::*;

static ASSETCACHE: Lazy<Mutex<HashMap<String, Image>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});


pub fn get_image(ctx: &mut Context, name: String) -> GameResult<Image>{
    let mut cache = ASSETCACHE.lock().unwrap();
    if let Some(im) = cache.get(&name){
        return Ok(im.clone());
    }
    let path = Path::new("assets").join("images").join(name.clone()).with_extension("png");
    let bytes = fs::read(path)?;
    let im = Image::from_bytes(ctx, &bytes)?;
    cache.insert(name, im.clone());
    return Ok(im);
}

pub fn get_scaled_image(ctx: &mut Context, name: String, tilesize: f32) -> GameResult<Image>{
    let base_image = get_image(ctx, name)?;

    let color_format = ctx.gfx.surface_format();
    let result_image = Image::new_canvas_image(
        ctx, color_format,
        tilesize.ceil() as u32,
        tilesize.ceil() as u32,
        1
    );
    let mut image_canvas = Canvas::from_image(ctx, result_image.clone(), TRANSPARENT_COLOR);

    image_canvas.draw(
        &base_image,
        DrawParam::default().scale(glam::vec2(
            tilesize / base_image.width() as f32,
            tilesize / base_image.height() as f32
        ))
    );

    image_canvas.finish(ctx)?;

    Ok(result_image)
}
