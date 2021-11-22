use bevy::{prelude::*, reflect::Map, window::WindowResized};

use crate::map::{MapTransform};

const TILE_SIZE: usize = 12;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app:&mut AppBuilder) {
        app.insert_resource(Terminal::new())
            .add_system(on_window_resize.system())
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(render_terminal.system())
            );
            // TODO: add stages to rendering, putting window resizes, game logic before render_terminal
    }
}

#[derive(PartialEq)]
pub struct MapPosition {
    pub x: isize,
    pub y: isize,
}

impl MapPosition {
    fn as_terminal(&self, transform: &Res<MapTransform>, terminal: &Terminal) -> Option<TerminalPosition> {
        let translated_x = self.x + transform.x;
        let translated_y = self.y + transform.y;

        // return position only if inside terminal boundaries
        if translated_x < terminal.width as isize && translated_y < terminal.height as isize {
            return Some(TerminalPosition {
                x: translated_x as usize,
                y: translated_y as usize,
            })
        }
        None
    }
}

#[derive(PartialEq)]
pub struct TerminalPosition {
    pub x: usize,
    pub y: usize,
}

// TODO: create new() method to ease terminal clearing, etc.
// TODO: privatize fields for this and TerminalPosition, replacing with methods to create new components?
#[derive(Clone)]
pub struct Renderable {
    pub glyph: u32,
    pub fg_color: Color,
    // TODO: implement as Option<Color>, rather than using Color::NONE
    pub bg_color: Color,
    pub priority: usize,
}

pub struct Foreground {}

pub struct Background {}

struct Terminal {
    width: usize,
    height: usize,
    buffer: Vec<Renderable>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal{
            width: 40, 
            height: 10, 
            buffer: Vec::new()}
    }
}

fn on_window_resize(
    mut commands: Commands, 
    windows: ResMut<Windows>, 
    asset_server: Res<AssetServer>,
    mut terminal: ResMut<Terminal>, 
    mut events: EventReader<WindowResized>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<Entity, With<TextureAtlasSprite>>,
) {
    for _e in events.iter() {
        let window = windows.get_primary().unwrap();
        let new_width = window.physical_width() as usize / TILE_SIZE;
        let new_height = window.physical_height() as usize / TILE_SIZE;

        terminal.width = new_width;
        terminal.height = new_height;
        terminal.buffer.clear();
        terminal.buffer.resize(new_width * new_height, Renderable { glyph: 0, fg_color: Color::NONE, bg_color: Color::NONE, priority: 0 });

        // TODO: maybe should init the atlas in setup(), then reference here
        // or at least load the asset just once, passing handle to here
        let texture_handle = asset_server.load("cheepicus12.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 12.0), 16, 16);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // delete foreground sprites (TODO: optimization: only delete/add needed sprites)
        for entity in query.iter_mut() {
            commands.entity(entity).despawn();
        }

        // spawn new foreground sprites
        for i in 0..terminal.width {
            for j in 0..terminal.height {
                let position = TerminalPosition {x: i, y:j};
                let pos_transform = get_position_transform(&position, &terminal, 1.0);
                commands.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: pos_transform,
                    sprite: TextureAtlasSprite::new(3),
                    ..Default::default()
                })
                .insert(position)
                .insert(Foreground{});
            }
        }

        // spawn new background sprites
        for i in 0..terminal.width {
            for j in 0..terminal.height {
                let position = TerminalPosition {x: i, y:j};
                let pos_transform = get_position_transform(&position, &terminal, 0.0);
                commands.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: pos_transform,
                    sprite: TextureAtlasSprite::new(219),
                    ..Default::default()
                })
                .insert(position)
                .insert(Background{});
            }
        }
    }
}

fn render_terminal(
    mut terminal: ResMut<Terminal>,
    map_transfrom: Res<MapTransform>,
    renderable_query: Query<(&MapPosition, &Renderable)>, // TODO: also query for terminalposition entities for ui, etc.
    mut sprite_query: QuerySet<(  
        Query<(&TerminalPosition, &mut TextureAtlasSprite), With<Foreground>>,
        Query<(&TerminalPosition, &mut TextureAtlasSprite), With<Background>>,
    )>
) {
    // clear buffer
    for i in terminal.buffer.iter_mut() {
        i.fg_color = Color::NONE;
        i.bg_color = Color::NONE;
        i.priority = 0;
    }


    for (position, renderable) in renderable_query.iter() {
        let terminal_pos = position.as_terminal(&map_transfrom, &terminal);
        if let Some(pos) = terminal_pos {
            let buffer_index = get_buffer_index(&pos, &terminal);
            // skip if we would go outside boundaries of terminal buffer
            // which can happen if the buffer hasn't been initialized yet
            if buffer_index < terminal.buffer.len() {
                if renderable.bg_color != Color::NONE {
                    terminal.buffer[buffer_index].bg_color = renderable.bg_color;
                }
                if renderable.priority >= terminal.buffer[buffer_index].priority {
                    terminal.buffer[buffer_index].glyph = renderable.glyph;
                    terminal.buffer[buffer_index].fg_color = renderable.fg_color;
                    terminal.buffer[buffer_index].priority = renderable.priority;

                }
            }
        }
        
    }

    for (position, mut sprite) in sprite_query.q0_mut().iter_mut() {
        let buffer_index = get_buffer_index(position, &terminal);
        if buffer_index < terminal.buffer.len() {
            sprite.index = terminal.buffer[buffer_index].glyph;
            sprite.color = terminal.buffer[buffer_index].fg_color;
        }
    }
    for (position, mut sprite) in sprite_query.q1_mut().iter_mut() {
        let buffer_index = get_buffer_index(position, &terminal);
        if buffer_index < terminal.buffer.len() {
            sprite.color = terminal.buffer[buffer_index].bg_color;
        }
    }
    // TODO: use ColorMaterial for terminal background, using update method in examples/3d/spawner?
}

// TODO: implement as Option<usize> to catch positions which fall outside the terminal boundaries 
// if we leave terminalposition public, there aren't guarantees that this will be inside bounds
fn get_buffer_index(pos: &TerminalPosition, terminal: &Terminal) -> usize {
    pos.x + (pos.y * terminal.width)
}

fn get_position_transform(position: &TerminalPosition, terminal: &Terminal, z: f32) -> Transform {
    let x: f32 = TILE_SIZE as f32 * (0.5 + position.x as f32 - (terminal.width as f32) / 2.0);
    let y: f32 = TILE_SIZE as f32 * (0.5 + position.y as f32 - (terminal.height as f32) / 2.0);
    // (x, y)
    Transform::from_xyz(x, y, z)
    // Transform::from_scale(Vec3::splat(6.0))
}