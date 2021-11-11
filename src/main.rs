use bevy::{prelude::*, window::WindowResized};

const TILE_SIZE: usize = 12;

struct MapPosition {
    x: isize,
    y: isize,
}

struct ConsolePosition {
    x: usize,
    y: usize,
}


// TODO: create new() method to ease console clearing, etc.
#[derive(Clone)]
struct Renderable {
    glyph: u32,
    fg_color: Color,
    // TODO: implement as Option<Color>, rather than using Color::NONE
    bg_color: Color,
    priority: usize,
}

struct Foreground {}
struct Background {}

struct Console {
    width: usize,
    height: usize,
    buffer: Vec<Renderable>,
}

fn on_window_resize(
    mut commands: Commands, 
    windows: ResMut<Windows>, 
    asset_server: Res<AssetServer>,
    mut console: ResMut<Console>, 
    mut events: EventReader<WindowResized>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<Entity, With<TextureAtlasSprite>>,
) {
    for _e in events.iter() {
        let window = windows.get_primary().unwrap();
        let new_width = window.physical_width() as usize / TILE_SIZE;
        let new_height = window.physical_height() as usize / TILE_SIZE;
        console.width = new_width;
        console.height = new_height;
        console.buffer.clear();
        console.buffer.resize(new_width * new_height, Renderable { glyph: 0, fg_color: Color::NONE, bg_color: Color::NONE, priority: 0 });

        let texture_handle = asset_server.load("cheepicus12.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 12.0), 16, 16);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // delete foreground sprites (TODO: optimization: only delete/add needed sprites)
        for entity in query.iter_mut() {
            commands.entity(entity).despawn();
        }

        // spawn new foreground sprites
        for i in 0..console.width {
            for j in 0..console.height {
                let position = ConsolePosition {x: i, y:j};
                let pos_transform = get_position_transform(&position, &console, 1.0);
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
        for i in 0..console.width {
            for j in 0..console.height {
                let position = ConsolePosition {x: i, y:j};
                let pos_transform = get_position_transform(&position, &console, 0.0);
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

fn render_console(
    mut console: ResMut<Console>,
    renderable_query: Query<(&ConsolePosition, &Renderable)>,
    mut sprite_query: QuerySet<(  
        Query<(&ConsolePosition, &mut TextureAtlasSprite), With<Foreground>>,
        Query<(&ConsolePosition, &mut TextureAtlasSprite), With<Background>>,
    )>
) {
    // clear buffer
    for i in console.buffer.iter_mut() {

    }


    for (position, renderable) in renderable_query.iter() {
        let buffer_index = get_buffer_index(position, &console);
        // ignore if we would go outside boundaries of console buffer
        // which can happen if the buffer hasn't been initialized yet
        if buffer_index < console.buffer.len() {
            if renderable.bg_color != Color::NONE {
                console.buffer[buffer_index].bg_color = renderable.bg_color;
            }
            if renderable.priority >= console.buffer[buffer_index].priority {
                console.buffer[buffer_index].glyph = renderable.glyph;
                console.buffer[buffer_index].fg_color = renderable.fg_color;
                console.buffer[buffer_index].priority = renderable.priority;

            }
        }
    }

    for (position, mut sprite) in sprite_query.q0_mut().iter_mut() {
        let buffer_index = get_buffer_index(position, &console);
        if buffer_index < console.buffer.len() {
            sprite.index = console.buffer[buffer_index].glyph;
            sprite.color = console.buffer[buffer_index].fg_color;
        }
    }
    for (position, mut sprite) in sprite_query.q1_mut().iter_mut() {
        let buffer_index = get_buffer_index(position, &console);
        if buffer_index < console.buffer.len() {
            sprite.color = console.buffer[buffer_index].bg_color;
        }
    }
    // TODO: use ColorMaterial for terminal background, using update method in examples/3d/spawner?
}

// TODO: implement as Option<usize> to catch positions which fall outside the console boundaries
fn get_buffer_index(pos: &ConsolePosition, console: &Console) -> usize {
    pos.x + (pos.y * console.width)
}

fn get_position_transform(position: &ConsolePosition, console: &Console, z: f32) -> Transform {
    let x: f32 = TILE_SIZE as f32 * (0.5 + position.x as f32 - (console.width as f32) / 2.0);
    let y: f32 = TILE_SIZE as f32 * (0.5 + position.y as f32 - (console.height as f32) / 2.0);
    // (x, y)
    Transform::from_xyz(x, y, z)
    // Transform::from_scale(Vec3::splat(6.0))
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // commands.insert_resource(Background {
    //     bg_material: materials.add(Color::GREEN.into()),
    // });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert(ConsolePosition{x: 10, y: 10})
        .insert(Renderable {
            glyph: 64, 
            fg_color: Color::BLUE,
            bg_color: Color::NONE,
            priority: 5
        });
    
    for i in 0..40 {
        for j in 0..40 {
            commands
                .spawn()
                .insert(ConsolePosition{x: i, y: j})
                .insert(Renderable {
                    glyph: 46, 
                    fg_color: Color::RED,
                    bg_color: Color::GREEN,
                    priority: 0
                });
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Console{width: 40, height: 10, buffer: vec![Renderable { glyph: 0, fg_color: Color::NONE, bg_color: Color::NONE, priority: 0 }]})
        // .add_startup_system(make_player.system())
        .add_startup_system(setup.system())
        .add_startup_system(on_window_resize.system())
        .add_system(on_window_resize.system())
        .add_system(render_console.system())
        // .add_system(animate_sprite_system.system())
        .run();
}
