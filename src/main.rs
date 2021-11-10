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

struct Foreground {
    glyph_index: usize,
    fg_color: Color,
}

struct Background {
    bg_color: Color,
    // bg_material: Handle<ColorMaterial>,
}

struct Console {
    width: usize,
    height: usize,
}

fn on_window_resize(windows: ResMut<Windows>, mut console: ResMut<Console>, mut events: EventReader<WindowResized>) {
    for _e in events.iter() {
        let window = windows.get_primary().unwrap();
        console.width = window.physical_width() as usize / TILE_SIZE;
        console.height = window.physical_height() as usize / TILE_SIZE;
    }
}

fn render_console(
    console: Res<Console>,
    mut query: QuerySet<(  
        Query<(&ConsolePosition, &Foreground, &mut Transform, &mut TextureAtlasSprite)>,
        Query<(&ConsolePosition, &Background, &mut Transform, &mut TextureAtlasSprite)>,
    )>
) {
    for (position, fg, mut transform, mut sprite) in query.q0_mut().iter_mut() {
        let (x, y) = get_position_translation(position, &console);
        transform.translation = Vec3::new(x, y, 1.0);
        sprite.index = fg.glyph_index as u32;
        sprite.color = fg.fg_color;
    }

    for (position, bg, mut transform, mut sprite) in query.q1_mut().iter_mut() {
        let (x, y) = get_position_translation(position, &console);
        transform.translation = Vec3::new(x, y, 0.0);
        sprite.color = bg.bg_color;
    }
}

// fn make_player(mut commands: Commands){
    
// }

fn get_position_translation(position: &ConsolePosition, console: &Console) -> (f32, f32) {
    let x: f32 = TILE_SIZE as f32 * (0.5 + position.x as f32 - (console.width as f32) / 2.0);
    let y: f32 = TILE_SIZE as f32 * (0.5 + position.y as f32 - (console.height as f32) / 2.0);
    (x, y)
    // Transform::from_xyz(x, y, 0.0)
    // Transform::from_scale(Vec3::splat(6.0))
}

// fn get_glyph_index(glyph: &char) -> u32 {
//     match glyph {
//         '@' => 64,
//         '.' => 46,
//         _ => 47,
//         ''
//     }
// }

// fn animate_sprite_system(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
// ) {
//     for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
//         timer.tick(time.delta());
//         if timer.finished() {
//             let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//             sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
//             sprite.color = Color::GREEN;
//         }
//     }
// }

fn setup(
    mut commands: Commands,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // console: Res<Console>,
) {
    let texture_handle = asset_server.load("cheepicus12.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 12.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // commands.insert_resource(Background {
    //     bg_material: materials.add(Color::GREEN.into()),
    // });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        // transform: get_position_transform(ConsolePosition{x: 10, y: 10}, Console{width: console.width, height: console.height}),
        sprite: TextureAtlasSprite::new(32),
        ..Default::default()
    })
        .insert(ConsolePosition{x: 10, y: 10})
        
        .insert(Foreground{
            glyph_index: 64, 
            fg_color: Color::BLUE, 
        });
    
    
    for i in 0..40 {
        for j in 0..40 {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    // transform: get_position_transform(ConsolePosition{x: 10, y: 10}, Console{width: console.width, height: console.height}),
                    sprite: TextureAtlasSprite::new(0),
                    ..Default::default()
                })
                
                .insert(ConsolePosition{x: i, y: j})
                .insert(Foreground{
                    glyph_index: 46, 
                    fg_color: Color::RED,
                });
        }
    }
    for i in 0..40 {
        for j in 0..40 {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    // transform: get_position_transform(ConsolePosition{x: 10, y: 10}, Console{width: console.width, height: console.height}),
                    sprite: TextureAtlasSprite::new(219),
                    ..Default::default()
                })
                
                .insert(ConsolePosition{x: i, y: j})
                .insert(Background{
                    bg_color: Color::GREEN,
                });
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Console{width: 40, height: 10})
        // .add_startup_system(make_player.system())
        .add_startup_system(setup.system())
        .add_system(on_window_resize.system())
        .add_system(render_console.system())
        // .add_system(animate_sprite_system.system())
        .run();
}
