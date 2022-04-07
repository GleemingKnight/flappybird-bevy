use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, OrthographicCameraBundle, Res, Transform};
use bevy::sprite::SpriteBundle;
use bevy::window::WindowDescriptor;

use crate::bird::BirdPlugin;
use crate::pipe::PipePlugin;

mod bird;
mod pipe;

/// Main function of the program
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Flappy Bird".to_string(),
            width: 288.0,
            height: 512.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BirdPlugin)
        .add_plugin(PipePlugin)
        .add_startup_system(setup_background)
        .add_startup_system(create_camera)
        .run()
}

/// Creates a simple orthographic camera
fn create_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

/// Sets up the background image
fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("background-night.png"),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("base.png"),
        transform: Transform::from_xyz(0., -256., 0.),
        ..Default::default()
    });
}