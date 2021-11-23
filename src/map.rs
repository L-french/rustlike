use bevy::prelude::*;
use rand::Rng;
use crate::rect::Rect;
use crate::terminal::*;

// pub struct Wall {}

#[derive(PartialEq, Clone, Copy)]
pub enum MapTile {
    Wall,
    Floor,
}

// maybe publish and divide map generation/entity spawning
struct MapBuffer {
    height: usize,
    width: usize,
    buffer: Vec<MapTile>,
}

impl MapBuffer {
    fn size(&self) -> usize {
        self.height * self.width
    }
}

// TODO: add width/height to just render to a subsection of the terminal, IDs to allow for multiple views
// could rename to 'viewport' or similar
pub struct MapTransform {
    pub x: isize,
    pub y: isize
}

fn apply_rectangle(tile: MapTile, rectangle: Rect, map: &mut MapBuffer) {
    for y in rectangle.y1 .. rectangle.y2 {
        for x in rectangle.x1 .. rectangle.x2 {
            map.buffer[x as usize + y as usize * map.width] = tile;
        }
    }
}

fn generate_map() -> MapBuffer {
    let mut map = MapBuffer {
        height: 50,
        width: 80,
        buffer: Vec::new(),
    };

    map.buffer = vec![MapTile::Wall; map.height * map.width];
    let room1 = Rect::new(1, 1, 10, 15);

    apply_rectangle(MapTile::Floor, room1, &mut map);
    map
}

pub fn spawn_map(mut commands: Commands) {
    let map = generate_map();

    for i in 0..map.size() {
        let (x, y) = (
            (i % map.width) as isize, 
            (i / map.width) as isize,
        );
        match map.buffer[i] {
            MapTile::Floor => {
                commands
                    .spawn()
                    .insert(MapPosition{x: x, y: y})
                    .insert(MapTile::Floor)
                    .insert(Renderable {
                        glyph: 46, 
                        fg_color: Color::GRAY,
                        bg_color: Color::DARK_GRAY,
                        priority: 0
                    });
            }
            MapTile::Wall => {
                commands.spawn()
                    .insert(MapPosition{x: x, y: y})
                    .insert(MapTile::Wall)
                    .insert(Renderable {
                        glyph: 35, 
                        fg_color: Color::BLACK,
                        bg_color: Color::GRAY,
                        priority: 0
                    });
            },
        }
    }
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