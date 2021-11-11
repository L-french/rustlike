use bevy::prelude::*;
mod terminal;
use terminal::*;

struct Player {}

fn movement(input: Res<Input<KeyCode>>, mut query: Query<&mut TerminalPosition, With<Player>>) {
    for mut pos in query.iter_mut() {
        if input.just_pressed(KeyCode::H) {
            pos.x = pos.x - 1;
        }
        if input.just_pressed(KeyCode::L) {
            pos.x = pos.x + 1;
        }
        if input.just_pressed(KeyCode::J) {
            pos.y = pos.y - 1;
        }
        if input.just_pressed(KeyCode::K) {
            pos.y = pos.y + 1;
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
            fg_color: Color::BLUE,
            bg_color: Color::NONE,
            priority: 5
        });
    
    for i in 0..40 {
        for j in 0..40 {
            commands
                .spawn()
                .insert(TerminalPosition{x: i, y: j})
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
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup.system())
        .add_system(movement.system())
        .run();
}
