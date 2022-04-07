use bevy::core::FixedTimestep;
use bevy::prelude::*;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets);
        app.add_system(move_pipes);

        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_pipes)
        );
    }
}

/// Pipe component
#[derive(Component)]
struct Pipe;

#[derive(Default, Clone)]
struct PipeResource(Handle<Image>);

/// Loads the needed assets
fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PipeResource(asset_server.load("pipe-green.png")));
}

/// Spawns the pipes at the edge of the screen
fn spawn_pipes(mut commands: Commands, mut pipe_resource: ResMut<PipeResource>) {
    let mut upper_pipe = SpriteBundle {
        texture: *pipe_resource.0,
        transform: Transform::from_xyz(144., 256., 0.),
        ..Default::default()
    };

    upper_pipe.sprite.flip_y = true;

    let mut lower_pipe = SpriteBundle {
        texture: *pipe_resource.0,
        transform: Transform::from_xyz(144., -150., 0.),
        ..Default::default()
    };

    commands.spawn_bundle(upper_pipe).insert(Pipe);
    commands.spawn_bundle(lower_pipe).insert(Pipe);
}

/// Moves all the pipes forward
fn move_pipes(mut query: Query<(&Pipe, &mut Transform)>) {

    for ((_pipe, mut transform)) in query.iter_mut().next() {
        transform.translation.x += 50.;
    }

}