use std::collections::HashMap;
use std::sync::Mutex;
use std::path::Path;
use std::fs;

use ggez::{
    graphics::Image,
    GameResult,
    Context
};
use once_cell::sync::Lazy;

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
