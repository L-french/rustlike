use bevy::prelude::*;
use rand::Rng;
mod terminal;
use terminal::*;

struct Player {}

struct Wall {}

fn player_can_move(
    move_pos: &TerminalPosition, 
    wall_query: &Query<&TerminalPosition, With<Wall>>,
) -> bool {
    for wall_pos in wall_query.iter() {
        if &move_pos == &wall_pos {return false}
    }
    true
}

fn movement(   
    input: Res<Input<KeyCode>>, 
    mut player_query: Query<&mut TerminalPosition, (With<Player>, Without<Wall>)>,
    wall_query: Query<&TerminalPosition, With<Wall>>,
) {
    for mut player_pos in player_query.iter_mut() {
        if input.just_pressed(KeyCode::H) {
            let move_pos = TerminalPosition {x: player_pos.x - 1, y: player_pos.y};
            if player_can_move(&move_pos, &wall_query) {
                player_pos.x = move_pos.x;
            }
        }
        if input.just_pressed(KeyCode::L) {
            let move_pos = TerminalPosition {x: player_pos.x + 1, y: player_pos.y};
            if player_can_move(&move_pos, &wall_query) {
                player_pos.x = move_pos.x;
            }
        }
        if input.just_pressed(KeyCode::J) {
            let move_pos = TerminalPosition {x: player_pos.x, y: player_pos.y - 1};
            if player_can_move(&move_pos, &wall_query) {
                player_pos.y = move_pos.y;
            }
        }
        if input.just_pressed(KeyCode::K) {
            let move_pos = TerminalPosition {x: player_pos.x, y: player_pos.y + 1};
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

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert(TerminalPosition{x: 10, y: 10})
        .insert(Player{})
        .insert(Renderable {
            glyph: 64, 
            fg_color: Color::TEAL,
            bg_color: Color::NONE,
            priority: 5
        });
    
    for i in 0..=80 {
        for j in 0..=50 {
            commands
                .spawn()
                .insert(TerminalPosition{x: i, y: j})
                .insert(Renderable {
                    glyph: 46, 
                    fg_color: Color::GRAY,
                    bg_color: Color::DARK_GRAY,
                    priority: 0
                });
        }
    }
    for i in 1..=80 {
        for j in 1..=50 {
            if rand::thread_rng().gen_bool(0.1) {
                commands.spawn()
                    .insert(TerminalPosition{x: i, y: j})
                    .insert(Wall{})
                    .insert(Renderable {
                        glyph: 35, 
                        fg_color: Color::BLACK,
                        bg_color: Color::GRAY,
                        priority: 1
                    });
            }
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup.system())
        .add_system(movement.system())
        .run();
}
