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

struct Renderable {
    glyph: char,
    fg: Color,
    bg: Color,
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
    mut query: Query<(&ConsolePosition, &mut Transform)>,
) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = get_position_translation(&position, &console);
    }
}

// fn make_player(mut commands: Commands){
    
// }

fn get_position_translation(position: &ConsolePosition, console: &Console) -> Vec3 {
    let x: f32 = TILE_SIZE as f32 * (0.5 + position.x as f32 - (console.width as f32) / 2.0);
    let y: f32 = TILE_SIZE as f32 * (0.5 + position.y as f32 - (console.height as f32) / 2.0);
    Vec3::new(x, y, 0.0)
    // Transform::from_xyz(x, y, 0.0)
    // Transform::from_scale(Vec3::splat(6.0))
}

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
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    console: Res<Console>,
) {
    let texture_handle = asset_server.load("cheepicus12.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 12.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        // transform: get_position_transform(ConsolePosition{x: 10, y: 10}, Console{width: console.width, height: console.height}),
        sprite: TextureAtlasSprite::new(3),
        ..Default::default()
    })
        .insert(ConsolePosition{x: 10, y: 10})
        
        .insert(Renderable{
            glyph: '@', 
            fg: Color::Rgba{ red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }, 
            bg: Color::Rgba{ red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
        });
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
