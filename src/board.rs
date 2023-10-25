use std::time::Duration;
use std::collections::HashMap;
use std::collections::HashSet;

use ggez::{
    glam,
    graphics,
    input::mouse::MouseButton,
    input::keyboard::{KeyInput, KeyCode, KeyMods},
    Context, GameResult
};

use crate::level::{Holding, LevelMode};
use crate::tile::{Tile, TileType};
use crate::block::{BlockObject, BlockObjectMode, BlockObjectAnimation};
use crate::constants::*;
use crate::helpers::*;
use crate::asset_cache;

pub struct Board {
    mouse_down: bool,
    canvas: BoardCanvas,
    state: BoardState
}

struct BoardCanvas {
    pos: graphics::Rect, // where to render it on the screen
    tile_size: f32,
    offset_x: f32, // the top left corner of the screen should show what's at (offset_x, offset_y)
    offset_y: f32
}

struct BoardState {
    animation_duration: Duration,
    animation_timer: Duration,
    tiles: Vec<Tile>,
    blockobjects: Vec<BlockObject>,
    activeblockobjects: Vec<BlockObject>,
    id_counter: i32
}

impl Board{
    pub fn new(screenpos: graphics::Rect) -> Board {
        Board{
            mouse_down: false,
            canvas: BoardCanvas::new(screenpos),
            state: BoardState::new()
        }
    }

    pub fn update(&mut self, ctx: &mut Context, mode: &LevelMode) -> SimulationResult {
        match *mode{
            LevelMode::Building => Ok(()),
            LevelMode::Running => {
                self.state.animation_timer += ctx.time.delta();
                while self.state.animation_timer >= self.state.animation_duration{
                    self.state.animation_timer -= self.state.animation_duration;
                    self.state.process_step()?;
                }
                Ok(())
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, out_canvas: &mut graphics::Canvas, mode: &LevelMode) -> GameResult {
        let color_format = ctx.gfx.surface_format();
        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            self.canvas.pos.w as u32,
            self.canvas.pos.h as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), BOARD_BG_COLOR);

        // empty tiles
        let empty_tile_image = asset_cache::get_scaled_image(ctx, "empty_tile".to_string(), self.canvas.tile_size)?;

        let mut empty_tile_ia = graphics::InstanceArray::new(ctx, empty_tile_image);

        let tilex_min = (self.canvas.offset_x/self.canvas.tile_size).floor() as i32;
        let tilex_max = ((self.canvas.offset_x+self.canvas.pos.w)/self.canvas.tile_size).ceil() as i32;
        let tiley_min = (self.canvas.offset_y/self.canvas.tile_size).floor() as i32;
        let tiley_max = ((self.canvas.offset_y+self.canvas.pos.h)/self.canvas.tile_size).ceil() as i32;

        for tiley in tiley_min..tiley_max {
            for tilex in tilex_min..tilex_max {
                empty_tile_ia.push(
                    glam::vec2(
                        tilex as f32 * self.canvas.tile_size - self.canvas.offset_x,
                        tiley as f32 * self.canvas.tile_size - self.canvas.offset_y
                    ).into()
                );
            }
        }
        image_canvas.draw(&empty_tile_ia, graphics::DrawParam::default());

        // filled tiles
        // I don't think an instance array would actually help here, given that rotations are different images
        // however, I could draw the bases first and then the symbols if I need the speed

        for tile in self.state.tiles.iter(){
            if tile.get_x() >= tilex_min && tile.get_x() <= tilex_max &&
                tile.get_y() >= tiley_min && tile.get_y() <= tiley_max{

                image_canvas.draw(
                    &tile.draw(ctx, self.canvas.tile_size)?,
                    glam::vec2(
                        tile.get_x() as f32 * self.canvas.tile_size - self.canvas.offset_x,
                        tile.get_y() as f32 * self.canvas.tile_size - self.canvas.offset_y
                    )
                )
            }
        }

        // blocks
        for blockobject in self.state.blockobjects.iter_mut().chain(self.state.activeblockobjects.iter_mut()){
            let bo_image = blockobject.draw(ctx, self.canvas.tile_size)?;
            let bo_pos = blockobject.get_top_left()?;
            let mut screenpos = glam::vec2(
                bo_pos.x as f32 * self.canvas.tile_size - self.canvas.offset_x,
                bo_pos.y as f32 * self.canvas.tile_size - self.canvas.offset_y
            );

            let animation_proportion = self.state.animation_timer.as_secs_f32()/self.state.animation_duration.as_secs_f32();
            match blockobject.anim{
                BlockObjectAnimation::Translation { x, y } => {
                    screenpos.x += x * self.canvas.tile_size * (animation_proportion - 1.0);
                    screenpos.y += y * self.canvas.tile_size * (animation_proportion - 1.0);
                },
                BlockObjectAnimation::Rotation { theta, around } => {
                    panic!("do this at some point");
                }
            }

            match (blockobject.mode, mode){
                (BlockObjectMode::Input, LevelMode::Building) =>
                    image_canvas.draw(&mult_alpha(ctx, bo_image, BUILDING_BLOCKOBJECT_ALPHA)?, screenpos),
                (BlockObjectMode::Output, _) =>
                    image_canvas.draw(&bo_image, screenpos),
                (BlockObjectMode::Processing, LevelMode::Running) =>
                    image_canvas.draw(&mult_alpha(ctx, bo_image, RUNNING_BLOCKOBJECT_ALPHA)?, screenpos),
                _default => ()
            }

        }


        image_canvas.finish(ctx)?;

        out_canvas.draw(&image, glam::vec2(self.canvas.pos.x, self.canvas.pos.y));
        Ok(())
    }

    pub fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) -> GameResult{
        if self.canvas.pos.contains(glam::vec2(x, y)) && button == MouseButton::Left{
            self.mouse_down = true;
        }
        Ok(())
    }

    pub fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32
    ) -> GameResult{
        if button == MouseButton::Left{
            self.mouse_down = false;
        }
        Ok(())
    }

    pub fn mouse_click_event(
        &mut self,
            ctx: &mut Context,
            button: MouseButton,
            x: f32,
            y: f32,
            held: &mut Holding
    ) -> GameResult{
        if self.canvas.pos.contains(glam::vec2(x, y)) && button == MouseButton::Left{
            let tilepos = self.canvas.screen_pos_to_tile(x, y);
            match held{
                Holding::Tile { tile } => {
                    self.state.place_tile(tile.get_type(), tilepos, tile.get_dir());
                },
                Holding::BlockObject { blockobject } => self.state.place_blockobject(blockobject.clone(), tilepos)?,
                Holding::None => ()
            }
            // NOTE: when I implement blockobject, make sure shift-placing it doesn't break anything
            if !ctx.keyboard.is_mod_active(KeyMods::SHIFT){
                *held = Holding::None;
            }
        }
        Ok(())
    }

    pub fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        dx: f32,
        dy: f32
    ) -> GameResult{
        if self.mouse_down{
            self.canvas.offset_x -= dx;
            self.canvas.offset_y -= dy;
        }
        Ok(())
    }

    pub fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        let mouse_pos = ctx.mouse.position();
        let tile_pos = self.canvas.screen_pos_to_tile(mouse_pos.x, mouse_pos.y);
        if input.keycode == Some(KeyCode::R) {
            if input.mods.contains(KeyMods::SHIFT){
                self.state.rotate_tile_ccw(tile_pos);
            }else{
                self.state.rotate_tile_cw(tile_pos);
            }
        }else if input.keycode == Some(KeyCode::D) {
            self.state.remove_tile(tile_pos);
        }
        Ok(())
    }

    pub fn process_start(&mut self) -> GameResult{
        self.state.process_start()
    }

    pub fn process_end(&mut self) -> GameResult{
        self.state.process_end()
    }
}

impl BoardCanvas{
    fn new(screenpos: graphics::Rect) -> BoardCanvas {
        BoardCanvas{
            pos: screenpos,
            tile_size: TILESIZE,
            offset_x: 0.0,
            offset_y: 0.0
        }
    }

    fn screen_pos_to_tile(&self, x: f32, y: f32) -> BoardPos{
        let true_x = x + self.offset_x - self.pos.x;
        let true_y = y + self.offset_y - self.pos.y;
        BoardPos{
            x: (true_x/self.tile_size).floor() as i32,
            y: (true_y/self.tile_size).floor() as i32
        }
    }
}

impl BoardState{
    fn new() -> BoardState {
        BoardState{
            animation_duration: Duration::from_secs_f32(ANIMATION_DURATION),
            animation_timer: Duration::ZERO,
            tiles: Vec::new(),
            blockobjects: Vec::new(),
            activeblockobjects: Vec::new(),
            id_counter: 0
        }
    }

    // find the index of the tile at a position
    // returns None if there is no tile
    fn find_tile(&mut self, pos: BoardPos) -> Option<usize>{
        let mut found_index : Option<usize> = None;
        for (i, tile) in self.tiles.iter().enumerate(){
            if tile.get_x() == pos.x && tile.get_y() == pos.y{
                found_index = Some(i);
            }
        }

        found_index
    }

    fn place_tile(&mut self, tiletype: TileType, pos: BoardPos, dir: Direction) -> usize{
        let newtile = Tile::new_directional(tiletype, pos, dir);
        let to_remove: Option<usize> = self.find_tile(pos);

        if let Some(i) = to_remove{
            self.tiles[i] = newtile;
            i
        }else{
            self.tiles.push(newtile);
            self.tiles.len()-1
        }
    }

    fn rotate_tile_cw(&mut self, pos: BoardPos){
        if let Some(i) = self.find_tile(pos){
            self.tiles[i].rotate_cw();
        }
    }

    fn rotate_tile_ccw(&mut self, pos: BoardPos){
        if let Some(i) = self.find_tile(pos){
            self.tiles[i].rotate_ccw();
        }
    }

    // returns false if the tile is not removed (typically because there is no tile at x,y)
    fn remove_tile(&mut self, pos: BoardPos){
        if let Some(i) = self.find_tile(pos){
            self.tiles.remove(i);
        }
    }

    fn place_blockobject(&mut self, mut blockobject: BlockObject, pos: BoardPos) -> GameResult{
        let tl = blockobject.get_top_left()?;
        blockobject.translate(pos.x - tl.x, pos.y - tl.y);

        // remove everything with matching ids
        let mut i = 0;
        while i < self.blockobjects.len(){
            if blockobject.id != -1 && self.blockobjects[i].id == blockobject.id{
                self.blockobjects.remove(i);
            }else if self.blockobjects[i].has_overlap(&mut blockobject){
                self.blockobjects.remove(i);
            }else{
                i += 1;
            }
        }
        // remove everything with overlap


        self.blockobjects.push(blockobject);
        Ok(())
    }

    fn process_start(&mut self) -> GameResult{
        // init the id_counter as the max of current block objects + 1
        self.id_counter = 0;
        for bo in self.blockobjects.iter(){
            if bo.id > self.id_counter{
                self.id_counter = bo.id;
            }
        }
        self.id_counter += 1;

        // create the initial block objects
        for bo in self.blockobjects.iter(){
            if bo.mode == BlockObjectMode::Input{
                let mut bocopy = bo.clone();

                bocopy.mode = BlockObjectMode::Processing;
                bocopy.id = self.id_counter;
                self.id_counter += 1;

                self.activeblockobjects.push(bocopy);
            }
        }

        Ok(())
    }

    fn process_end(&mut self) -> GameResult{
        // reset idcounter
        self.id_counter = 0;

        // remove active blockobjects
        self.activeblockobjects.clear();

        Ok(())
    }

    fn process_step(&mut self) -> SimulationResult{
        println!("processing");

        let n = self.activeblockobjects.len();
        let mut max_priority = vec![0; n];
        let mut relevant_tiles: Vec<Vec<Tile>> = vec![vec![]; n];

        // can I make this more efficient?
        for tile in self.tiles.iter(){
            for (i, blockobject) in self.activeblockobjects.iter_mut().enumerate(){
                if blockobject.overlap_tile(tile.get_pos()){
                    if tile.get_priority() > max_priority[i]{
                        max_priority[i] = tile.get_priority();
                        relevant_tiles[i].clear();
                    }
                    if tile.get_priority() == max_priority[i]{
                        relevant_tiles[i].push(tile.clone());
                    }
                }
            }
        }

        let mut move_dir = vec![None; n];

        // resolve directions
        for i in 0..self.activeblockobjects.len(){
            //TODO: splitting
            for tile in relevant_tiles[i].iter(){
                if None == move_dir[i]{
                    move_dir[i] = Some(tile.get_dir());
                } else if Some(tile.get_dir()) != move_dir[i]{
                    return Err(SimulationError{
                        message: "Attempted to move block in multiple directions".to_string(),
                        relevant_locations: relevant_tiles[i].iter().map(|tile| tile.get_pos()).collect()
                    })
                }
            }
        }

        // merge check
        // I could do a DSU, but the speed of this is so small anyways
        let mut merge_groups: Vec<HashSet<usize>> = Vec::new();
        for i in 0..self.activeblockobjects.len(){
            for j in 0..self.activeblockobjects.len(){
                let itl = self.activeblockobjects[i].get_top_left().map_err(|e| SimulationError::from_string(e.to_string()))?;
                let ibr = self.activeblockobjects[i].get_bottom_right().map_err(|e| SimulationError::from_string(e.to_string()))?;
                let jtl = self.activeblockobjects[j].get_top_left().map_err(|e| SimulationError::from_string(e.to_string()))?;
                let jbr = self.activeblockobjects[j].get_bottom_right().map_err(|e| SimulationError::from_string(e.to_string()))?;

                let can_merge = match (move_dir[i], move_dir[j]){
                    (Some(Direction::Up), Some(Direction::Down)) => (ibr.x >= jtl.x && jbr.x >= itl.x) && (jbr.y == itl.y - 1),
                    (Some(Direction::Left), Some(Direction::Right)) => (ibr.y >= jtl.y && jbr.y >= itl.y) && (jbr.x == itl.x - 1),
                    _default => false
                };

                if can_merge{
                    let mut found_group = false;
                    for group in merge_groups.iter_mut(){
                        if group.contains(&i){
                            found_group = true;
                            group.insert(j);
                        } else if group.contains(&j){
                            found_group = true;
                            group.insert(i);
                        }
                    }

                    if !found_group{
                        let mut tempset = HashSet::new();
                        tempset.insert(i);
                        tempset.insert(j);
                        merge_groups.push(tempset);
                    }
                }
            }
        }

        // resolve merges
        let mut to_remove: Vec<usize> = vec![];
        for group in merge_groups.into_iter(){
            let mut merged_group = BlockObject::new();
            merged_group.id = self.id_counter;
            self.id_counter += 1;
            for i in group.into_iter(){
                merged_group.merge(&mut self.activeblockobjects[i]);
                to_remove.push(i);
            }
            self.activeblockobjects.push(merged_group);
            move_dir.push(None);
        }
        to_remove.sort_unstable_by_key(|i| -(*i as i64));
        for i in to_remove{
            self.activeblockobjects.remove(i);
            move_dir.remove(i);
        }

        // TODO: do I need ids to be constant across ticks, because if not I can use index for collision


        // TODO: figure out splitting before starting collision detection

        let mut collision_map: HashMap<BoardPos, i32> = HashMap::new();
        let mut collisions: Vec<BoardPos> = Vec::new();

        // MOVE THOSE FELLAS
        for i in 0..self.activeblockobjects.len(){
            let bo = &mut self.activeblockobjects[i];
            // before move check
            for block in bo.blocks.iter_mut(){
                if let Some(current) = collision_map.insert(block.pos, bo.id){
                    if current != bo.id{
                        collisions.push(block.pos.clone());
                    }
                }
            }

            // move
            // println!("attempting to move id:{}", bo.id);
            let dx: i32;
            let dy: i32;
            match move_dir[i]{
                Some(Direction::Right) => {dx = 1; dy = 0},
                Some(Direction::Down)  => {dx = 0; dy = 1},
                Some(Direction::Left)  => {dx = -1; dy = 0},
                Some(Direction::Up)    => {dx = 0; dy = -1},
                None                   => {dx = 0; dy = 0},
            }
            bo.translate(dx,dy);
            bo.anim = BlockObjectAnimation::Translation {x: dx as f32, y: dy as f32};
            bo.generate_bounds().map_err(|e| SimulationError::from_string(e.to_string()))?;

            // after move check
            for block in bo.blocks.iter_mut(){
                if let Some(current) = collision_map.insert(block.pos, bo.id){
                    if current != bo.id{
                        collisions.push(block.pos.clone());
                    }
                }
            }
        }

        if collisions.len() != 0{
            return Err(SimulationError{
                message: "Collision occured".to_string(),
                relevant_locations: collisions
            })
        }

        Ok(())
    }
}
