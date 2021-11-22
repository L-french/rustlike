use bevy::prelude::*;
mod terminal;
mod map;
use terminal::*;
use map::*;

struct Player {}


fn player_can_move(
    move_pos: &MapPosition, 
    map_query: &Query<(&MapPosition, &MapTile)>,
) -> bool {
    for (tile_pos, tile) in map_query.iter() {
        if let MapTile::Wall = tile {
            if &move_pos == &tile_pos {return false}
        }
    }
    true
}

fn movement(   
    input: Res<Input<KeyCode>>, 
    mut player_query: Query<&mut MapPosition, (With<Player>, Without<MapTile>)>,
    wall_query: Query<(&MapPosition, &MapTile)>,
) {
    for mut player_pos in player_query.iter_mut() {
        if input.just_pressed(KeyCode::H) {
            let move_pos = MapPosition {x: player_pos.x - 1, y: player_pos.y};
            if player_can_move(&move_pos, &wall_query) {
                player_pos.x = move_pos.x;
            }
        } else if input.just_pressed(KeyCode::L) {
            let move_pos = MapPosition {x: player_pos.x + 1, y: player_pos.y};
            if player_can_move(&move_pos, &wall_query) {
                player_pos.x = move_pos.x;
            }
        } else if input.just_pressed(KeyCode::J) {
            let move_pos = MapPosition {x: player_pos.x, y: player_pos.y - 1};
            if player_can_move(&move_pos, &wall_query) {
                player_pos.y = move_pos.y;
            }
        } else if input.just_pressed(KeyCode::K) {
            let move_pos = MapPosition {x: player_pos.x, y: player_pos.y + 1};
            if player_can_move(&move_pos, &wall_query) {
                player_pos.y = move_pos.y;
            }
        }
    }
}

fn setup(
    mut commands: Commands,
) {
    // commands.insert_resource(Background {
    //     bg_material: materials.add(Color::GREEN.into()),
    // });

    commands.insert_resource(MapTransform {x: 0, y: 0});

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert(MapPosition{x: 10, y: 10})
        .insert(Player{})
        .insert(Renderable {
            glyph: 64, 
            fg_color: Color::TEAL,
            bg_color: Color::NONE,
            priority: 5
        });
    spawn_debug_map(commands);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup.system())
        .add_system(movement.system())
        .run();
}
