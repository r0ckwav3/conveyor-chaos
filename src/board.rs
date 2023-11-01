use std::time::Duration;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::consts::PI;

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
    game_ticks: i32,
    tiles: Vec<Tile>,
    blockobjects: Vec<BlockObject>,
    activeblockobjects: Vec<BlockObject>,
}

impl Board{
    pub fn new(screenpos: graphics::Rect) -> Board {
        Board{
            mouse_down: false,
            canvas: BoardCanvas::new(screenpos),
            state: BoardState::new()
        }
    }

    pub fn update(&mut self, ctx: &mut Context, mode: &LevelMode) -> SimulationResult<bool> {
        match *mode{
            LevelMode::Building => Ok(false),
            LevelMode::Running => {
                self.state.animation_timer += ctx.time.delta();
                while self.state.animation_timer >= self.state.animation_duration{
                    self.state.animation_timer -= self.state.animation_duration;
                    if self.state.process_step()?{
                        return Ok(true)
                    }
                }
                Ok(false)
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
            let param: graphics::DrawParam = match blockobject.anim{
                BlockObjectAnimation::Translation { x, y } => {
                    screenpos.x += x * self.canvas.tile_size * (animation_proportion - 1.0);
                    screenpos.y += y * self.canvas.tile_size * (animation_proportion - 1.0);
                    screenpos.into()
                },
                BlockObjectAnimation::Rotation { theta , around } => {
                    // The default behaviour of ggez is to rotate around the *original* top left
                    // we want to find the shift that turns that into rotation around `around`
                    // consider the vector pointing from the pivot to around
                    // find out where it points after the rotation
                    // point it in the right direction
                    let rot = theta*(1.0 - animation_proportion);
                    let bo_tl = blockobject.get_top_left()?;
                    let around_vec = glam::vec2(
                        self.canvas.tile_size * ((around.x - bo_tl.x) as f32 + 0.5),
                        self.canvas.tile_size * ((around.y - bo_tl.y) as f32 + 0.5)
                    );
                    let rot_mat = glam::Mat2::from_angle(rot);
                    let rotated_vec = rot_mat * around_vec;
                    screenpos.x += around_vec.x - rotated_vec.x;
                    screenpos.y += around_vec.y - rotated_vec.y;
                    let param: graphics::DrawParam= screenpos.into();
                    param.rotation(rot)
                },
                BlockObjectAnimation::Output => {
                    // TODO: think of some fun animation to do here
                    screenpos.into()
                }
            };

            match (blockobject.mode, mode){
                (BlockObjectMode::Input, LevelMode::Building) =>
                    image_canvas.draw(&mult_alpha(ctx, bo_image, BUILDING_BLOCKOBJECT_ALPHA)?, param),
                (BlockObjectMode::Output, _) =>
                    image_canvas.draw(&bo_image, param),
                (BlockObjectMode::Processing, LevelMode::Running) =>
                    image_canvas.draw(&mult_alpha(ctx, bo_image, RUNNING_BLOCKOBJECT_ALPHA)?, param),
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
        // immediately start the first step
        self.state.animation_timer = self.state.animation_duration;
        self.state.process_start()
    }

    pub fn process_end(&mut self) -> GameResult{
        self.state.process_end()
    }

    pub fn num_blockobjects(&self) -> usize{
        self.state.blockobjects.len()
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
            game_ticks: 0,
            tiles: Vec::new(),
            blockobjects: Vec::new(),
            activeblockobjects: Vec::new(),
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
        // reset the counters
        for bo in self.blockobjects.iter_mut(){
            bo.counter = bo.start_counter;
        }

        for tile in self.tiles.iter_mut(){
            tile.save_dir()
        }

        self.game_ticks = 0;

        Ok(())
    }

    fn process_end(&mut self) -> GameResult{
        // remove active blockobjects
        self.activeblockobjects.clear();

        for tile in self.tiles.iter_mut(){
            tile.load_dir()
        }

        Ok(())
    }

    // returning true means we won
    fn process_step(&mut self) -> SimulationResult<bool>{
        // did we win?
        let mut winning = true;
        for out in self.blockobjects.iter().filter(|bo| bo.mode == BlockObjectMode::Output){
            if out.counter != 0{
                winning = false;
            }
        }
        if winning{
            return Ok(true);
        }

        // place block objects every other tick
        if self.game_ticks % 2 == 0{
            for bo in self.blockobjects.iter_mut(){
                if bo.mode == BlockObjectMode::Input && bo.counter > 0{
                    let mut bocopy = bo.clone();

                    bocopy.mode = BlockObjectMode::Processing;
                    bocopy.just_moved = true; // make delay blocks work more intuitively
                    bo.counter -= 1;

                    self.activeblockobjects.push(bocopy);
                }
            }
        }


        let n = self.activeblockobjects.len();
        let mut max_priority = vec![0; n];
        let mut relevant_tiles: Vec<Vec<Tile>> = vec![vec![]; n];

        // can I make this more efficient?
        for tile in self.tiles.iter(){
            for (i, blockobject) in self.activeblockobjects.iter_mut().enumerate(){
                if blockobject.overlap_tile(tile.get_pos()){
                    if tile.get_type().get_priority() > max_priority[i]{
                        max_priority[i] = tile.get_type().get_priority();
                        relevant_tiles[i].clear();
                    }
                    if tile.get_type().get_priority() == max_priority[i]{
                        relevant_tiles[i].push(tile.clone());
                    }
                }
            }
        }

        let mut moves: Vec<MovementType> = vec![MovementType::None; n];

        // make sure we only have one rot tile
        // assume that rot tiles have their own reserved priority (1 for now)
        for i in 0..self.activeblockobjects.len(){
            if let Some(tile) = relevant_tiles[i].get(0){
                if tile.get_type().is_rot_tile(){
                    if relevant_tiles[i].len() > 1{
                        return Err(SimulationError{
                            message: "Attempted to rotate block from multiple pivots".to_string(),
                            relevant_locations: relevant_tiles[i].iter().map(|tile| tile.get_pos()).collect()
                        });
                    } else {
                        match tile.get_type(){
                            TileType::RotTileCW => {
                                moves[i] = MovementType::Rotation{cw: true, around: tile.get_pos()}
                            },
                            TileType::RotTileCCW => {
                                moves[i] = MovementType::Rotation{cw: false, around: tile.get_pos()}
                            },
                            _ => panic!("Unregistered Rotation tile type")
                        }
                    }
                }
            }
        }

        let mut move_dirs: Vec<HashSet<Direction>> = vec![HashSet::new(); n];
        // list all directions
        for i in 0..self.activeblockobjects.len(){
            for tile in relevant_tiles[i].iter(){
                if tile.get_type().is_push_tile(){
                    move_dirs[i].insert(tile.get_dir());
                }
            }
        }

        // resolve directions which will potentially split
        for i in 0..self.activeblockobjects.len(){
            if move_dirs[i].len() == 1 {
                let temp: Vec<&Direction> = move_dirs[i].iter().collect();
                moves[i] = MovementType::Translation(*temp[0]);
            } else if move_dirs[i].len() > 1{
                let mut good = false;
                if relevant_tiles[i][0].get_dir() == Direction::Left || relevant_tiles[i][0].get_dir() == Direction::Right{
                    good = true;
                    let seam = match relevant_tiles[i][0].get_dir(){
                        Direction::Left => relevant_tiles[i][0].get_pos().x,
                        Direction::Right => relevant_tiles[i][0].get_pos().x-1,
                        _default => panic!("this should be impossible")
                    };

                    // are all tiles on the correct side of the seam?
                    for tile in relevant_tiles[i].iter(){
                        match tile.get_dir(){
                            Direction::Left => {good = good && (tile.get_pos().x <= seam)}
                            Direction::Right => {good = good && (tile.get_pos().x > seam)}
                            _default => {good = false;}
                        }
                    }
                    if good{
                        // do we have the whole seam covered
                        let mut sides_covered = HashMap::new();
                        for y in self.activeblockobjects[i].get_vert_seam(seam){
                            sides_covered.insert(y, (false, false));
                        }
                        for tile in relevant_tiles[i].iter(){
                            if tile.get_pos().x == seam{
                                sides_covered.get_mut(&tile.get_pos().y).map(|p| p.0 = true);
                            } else if tile.get_pos().x == seam + 1{
                                sides_covered.get_mut(&tile.get_pos().y).map(|p| p.1 = true);
                            }
                        }
                        for (_k, v) in sides_covered{
                            if v != (true, true){
                                good = false;
                            }
                        }
                    }
                    if good{
                        let new_bo = self.activeblockobjects[i].split_vert_seam(seam);
                        self.activeblockobjects.push(new_bo);
                        moves[i] = MovementType::Translation(Direction::Left);
                        moves.push(MovementType::Translation(Direction::Right));
                    }
                } else if relevant_tiles[i][0].get_dir() == Direction::Up || relevant_tiles[i][0].get_dir() == Direction::Down{
                    good = true;
                    let seam = match relevant_tiles[i][0].get_dir(){
                        Direction::Up => relevant_tiles[i][0].get_pos().y,
                        Direction::Down => relevant_tiles[i][0].get_pos().y-1,
                        _default => panic!("this should be impossible")
                    };

                    // are all tiles on the correct side of the seam?
                    for tile in relevant_tiles[i].iter(){
                        match tile.get_dir(){
                            Direction::Up => {good = good && (tile.get_pos().y <= seam)}
                            Direction::Down => {good = good && (tile.get_pos().y > seam)}
                            _default => {good = false;}
                        }
                    }
                    if good{
                        // do we have the whole seam covered
                        let mut sides_covered = HashMap::new();
                        for x in self.activeblockobjects[i].get_hori_seam(seam){
                            sides_covered.insert(x, (false, false));
                        }
                        for tile in relevant_tiles[i].iter(){
                            if tile.get_pos().y == seam{
                                sides_covered.get_mut(&tile.get_pos().x).map(|p| p.0 = true);
                            } else if tile.get_pos().y == seam + 1{
                                sides_covered.get_mut(&tile.get_pos().x).map(|p| p.1 = true);
                            }
                        }
                        for (_k, v) in sides_covered{
                            if v != (true, true){
                                good = false;
                            }
                        }
                    }
                    if good{
                        let new_bo = self.activeblockobjects[i].split_hori_seam(seam);
                        self.activeblockobjects.push(new_bo);
                        moves[i] = MovementType::Translation(Direction::Up);
                        moves.push(MovementType::Translation(Direction::Down));
                    }
                }
                if !good{
                    return Err(SimulationError{
                        message: "Attempted to move block in multiple directions".to_string(),
                        relevant_locations: relevant_tiles[i].iter().map(|tile| tile.get_pos()).collect()
                    });
                }
            }
        }

        // reset just_moved
        for bo in self.activeblockobjects.iter_mut(){
            bo.just_moved = false;
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

                let can_merge = match (moves[i], moves[j]){
                    (MovementType::Translation(Direction::Up), MovementType::Translation(Direction::Down)) =>
                        (ibr.x >= jtl.x && jbr.x >= itl.x) && (jbr.y == itl.y - 1),
                    (MovementType::Translation(Direction::Left), MovementType::Translation(Direction::Right)) =>
                        (ibr.y >= jtl.y && jbr.y >= itl.y) && (jbr.x == itl.x - 1),
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
            for i in group.into_iter(){
                merged_group.merge(&mut self.activeblockobjects[i]);
                to_remove.push(i);
            }
            self.activeblockobjects.push(merged_group);
            moves.push(MovementType::None);
        }
        to_remove.sort_unstable_by_key(|i| -(*i as i64));
        for i in to_remove{
            self.activeblockobjects.remove(i);
            moves.remove(i);
        }

        let mut collision_map: HashMap<BoardPos, usize> = HashMap::new();
        let mut collisions: Vec<BoardPos> = Vec::new();

        // MOVE THOSE FELLAS
        for i in 0..self.activeblockobjects.len(){
            let bo = &mut self.activeblockobjects[i];
            // before move check
            for block in bo.blocks.iter_mut(){
                if let Some(current) = collision_map.insert(block.pos, i){
                    if current != i{
                        collisions.push(block.pos.clone());
                    }
                }
            }

            // move
            let dx: i32;
            let dy: i32;
            if let MovementType::Translation(move_dir) = moves[i]{
                match move_dir{
                    Direction::Right => {dx = 1; dy = 0},
                    Direction::Down  => {dx = 0; dy = 1},
                    Direction::Left  => {dx = -1; dy = 0},
                    Direction::Up    => {dx = 0; dy = -1},
                }

                bo.translate(dx,dy);
                bo.anim = BlockObjectAnimation::Translation {x: dx as f32, y: dy as f32};

                // set just_moved
                bo.just_moved = true;
            } else if let MovementType::Rotation{cw, around} = moves[i]{
                if cw {
                    bo.rotate_cw(around);
                    bo.anim = BlockObjectAnimation::Rotation {theta: -PI/2.0, around};
                }else{
                    bo.rotate_ccw(around);
                    bo.anim = BlockObjectAnimation::Rotation {theta: PI/2.0, around};
                }
            } else {
                bo.anim = BlockObjectAnimation::Translation {x: 0.0, y: 0.0};
            }

            // after move check
            for block in bo.blocks.iter_mut(){
                if let Some(current) = collision_map.insert(block.pos, i){
                    if current != i{
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

        // update alternating tiles
        // relevant_tiles contains copies of the real tiles, so I've got to grab the
        // real ones back
        let mut toflip: HashSet<BoardPos> = HashSet::new();
        for tile in relevant_tiles.iter().flatten(){
            if tile.get_type() == TileType::AltTile{
                toflip.insert(tile.get_pos());
            }
        }
        for tile in self.tiles.iter_mut(){
            if toflip.contains(&tile.get_pos()){
                tile.flip_dir();
            }
        }

        // erase things on outputs
        let mut to_remove: Vec<usize> = vec![];
        for i in 0..self.activeblockobjects.len(){
            if self.activeblockobjects[i].just_moved{
                continue;
            }
            for out in self.blockobjects.iter_mut().filter(|bo| bo.mode == BlockObjectMode::Output){
                if out.exact_overlap(&mut self.activeblockobjects[i]){
                    out.anim = BlockObjectAnimation::Output;
                    out.counter -= 1;
                    if out.counter < 0{
                        return Err(SimulationError{
                            message: format!("Too many objects in one output (expected {})", out.start_counter),
                            relevant_locations: out.block_locations()
                        })
                    }
                    to_remove.push(i);
                }
            }
        }
        to_remove.sort_unstable_by_key(|i| -(*i as i64));
        for i in to_remove{
            self.activeblockobjects.remove(i);
            moves.remove(i); // not really neccisary, but the housekeeping is nice
        }

        self.game_ticks += 1;
        Ok(false)
    }
}
