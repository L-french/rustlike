use bevy::prelude::*;

struct Position {
    x: isize,
    y: isize,
}

struct Renderable {
    glyph: char,
    fg: Color,
    bg: Color,
}

fn make_player(mut commands: Commands){
    commands.spawn()
        .insert(Position{x: 10, y: 10})
        .insert(Renderable{
            glyph: '@', 
            fg: Color::Rgba{ red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }, 
            bg: Color::Rgba{ red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
        });
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("cheepicus12.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 12.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true));
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(make_player.system())
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .run();
}
