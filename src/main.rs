use std::sync::Arc;

use bevy_ecs::system::{Commands, Res};
use context::Context;
use nalgebra::Point3;
use rapier3d::na::Vector3;
use scene::material::Material;
use scene::mesh::Mesh;
use scene::model::Model;

use crate::application::{AppConfig, ApplicationBuilder};
use crate::physics_context::{BoxCollider, RapierRigidBody};
use crate::player::PlayerSettings;
use crate::scene::transform::TransformBuilder;

mod application;
mod camera;
mod context;
mod input;
mod physics_context;
mod player;
mod render;
mod scene;
mod time;

fn spawn_world(mut commands: Commands, context: Res<Context>) {
    let memory_allocator = Arc::new(
        vulkano::memory::allocator::StandardMemoryAllocator::new_default(context.device()),
    );

    let cube = Mesh::cube(0.5, 0.5, 0.5, &memory_allocator);

    commands.spawn((
        Model {
            mesh: cube,
            material: Arc::new(Material {}),
        },
        TransformBuilder::new()
            .position(Point3::from(Vector3::new(0.0, 10.0, 0.0)))
            .build(),
        BoxCollider {
            size: Vector3::new(0.5, 0.5, 0.5),
        },
        RapierRigidBody { handle: None },
    ));

    let platform = Mesh::cube(20.0, 0.1, 20.0, &memory_allocator);

    commands.spawn((
        Model {
            mesh: platform,
            material: Arc::new(Material {}),
        },
        TransformBuilder::new()
            .position(Point3::from(Vector3::new(0.0, -0.5, 0.0)))
            .build(),
        BoxCollider {
            size: Vector3::new(20.0, 0.1, 20.0),
        },
    ));
}

fn main() {
    // TODO: remove this
    std::env::set_var("RUST_BACKTRACE", "1");

    // TODO: read from file
    let config = AppConfig {
        resolution: (1280, 720),
        fullscreen: false,
        brightness: 1.0,
        refresh_rate: 60,
    };

    let player_settings = PlayerSettings::new(5.0, 0.5, 2.81);

    let application = ApplicationBuilder::new(config)
        .with_startup_system(spawn_world)
        .with_player_controller(player_settings)
        .build();

    application.run();
}
