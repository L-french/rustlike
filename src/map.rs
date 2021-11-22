use bevy::prelude::*;
use rand::Rng;
use crate::terminal::*;

// pub struct Wall {}

pub enum MapTile {
    Wall,
    Floor,
}

// struct MapBuffer {
//     height: usize,
//     width: usize,
//     buffer: Vec<MapTile>,
// }

// TODO: add width/height to just render to a subsection of the terminal, IDs to allow for multiple views
// could rename to 'viewport' or similar
pub struct MapTransform {
    pub x: isize,
    pub y: isize
}

pub fn spawn_map() {
    
}

pub fn spawn_debug_map(mut commands: Commands) {
    for i in 0..=160 {
        for j in 0..=50 {
            if rand::thread_rng().gen_bool(0.1) {
                commands.spawn()
                    .insert(MapPosition{x: i, y: j})
                    .insert(MapTile::Wall)
                    .insert(Renderable {
                        glyph: 35, 
                        fg_color: Color::BLACK,
                        bg_color: Color::GRAY,
                        priority: 1
                    });
            } else {
                commands
                    .spawn()
                    .insert(MapPosition{x: i, y: j})
                    .insert(MapTile::Floor)
                    .insert(Renderable {
                        glyph: 46, 
                        fg_color: Color::GRAY,
                        bg_color: Color::DARK_GRAY,
                        priority: 0
                    });
            }
        }
    }
}