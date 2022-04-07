use bevy::prelude::*;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_bird);
        app.add_system(handle_input);
        app.add_system(handle_movement);
    }
}

#[derive(Component, Default)]
struct Bird {
    pub velocity: Vec2
}

/// Spawns in the bird sprite
fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("yellowbird-midflap.png"),
            ..Default::default()
        })
        .insert(Bird::default());
}

/// Handles the movement
fn handle_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Bird, &mut Transform)>) {
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Up) {
        if let Some((mut bird, mut transform)) = query.iter_mut().next() {
            bird.velocity.y = 18.;
        }
    }
}

/// Handles the movement of the bird
fn handle_movement(mut query: Query<(&mut Bird, &mut Transform)>) {
    if let Some((mut bird, mut transform)) = query.iter_mut().next() {
        if bird.velocity.y > -4.0 {
            bird.velocity.y -= 2.0;
        }

        transform.translation.y += bird.velocity.y;
    }
}

/// Handles the collision of the bird
fn handle_collision(mut commands: Commands, mut query: Query<(&mut Bird, &mut Transform)>, mut bird_entities: Query<Entity, With<Bird>>) {
    if let Some((mut bird, mut transform)) = query.iter_mut().next() {
        if transform.translation.y > 384. || transform.translation.y < -384. {
            for ent in bird_entities.iter() {
                commands.entity(ent).despawn();
            }
        }
    }
}