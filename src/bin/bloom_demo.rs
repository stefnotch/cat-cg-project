use std::f32::consts::PI;
use std::sync::Arc;

use bevy_ecs::system::{Commands, Res};
use cat_to_the_past::render::context::Context;
use nalgebra::{Point3, UnitQuaternion, UnitVector3, Vector};
use rapier3d::na::Vector3;

use cat_to_the_past::core::application::{AppConfig, ApplicationBuilder};
use cat_to_the_past::player::{PlayerControllerSettings, PlayerSpawnSettings};
use cat_to_the_past::scene::light::{Light, PointLight};
use cat_to_the_past::scene::material::Material;
use cat_to_the_past::scene::mesh::Mesh;
use cat_to_the_past::scene::model::{Model, Primitive};
use cat_to_the_past::scene::transform::TransformBuilder;

fn spawn_bloom_demo(mut commands: Commands, context: Res<Context>) {
    let memory_allocator = Arc::new(
        vulkano::memory::allocator::StandardMemoryAllocator::new_default(context.device()),
    );

    let cube = Mesh::cube(1.0, 1.0, 1.0, &memory_allocator);

    let material = Material {
        base_color: [1.0, 1.0, 1.0].into(),
        base_color_texture: None,
        roughness_factor: 0.9,
        metallic_factor: 0.1,
        emissivity: Default::default(),
    };

    let model = Model {
        primitives: vec![Primitive {
            mesh: cube.clone(),
            material: Arc::from(material),
        }],
    };

    commands.spawn((
        model.clone(),
        TransformBuilder::new()
            .position([0.0, -1.0, 0.0].into())
            .scale([12.5, 0.5, 12.5].into())
            .build(),
    ));

    commands.spawn((
        model.clone(),
        TransformBuilder::new()
            .position([0.0, 1.5, 0.0].into())
            .scale([0.5, 0.5, 0.5].into())
            .build(),
    ));

    commands.spawn((
        model.clone(),
        TransformBuilder::new()
            .position([2.0, 0.0, 1.0].into())
            .scale([0.5, 0.5, 0.5].into())
            .build(),
    ));

    commands.spawn((
        model.clone(),
        TransformBuilder::new()
            .position([-1.0, -1.0, 2.0].into())
            .rotation(UnitQuaternion::from_euler_angles(PI / 3.0, 0.0, PI / 3.0))
            .build(),
    ));

    let mut spawn_light = |position: Point3<f32>, color: Vector3<f32>, intensity: f32| {
        commands.spawn((
            Model {
                primitives: vec![Primitive {
                    mesh: cube.clone(),
                    material: Arc::from(Material {
                        base_color: Vector3::new(1.0, 1.0, 1.0),
                        base_color_texture: None,
                        roughness_factor: 0.9,
                        metallic_factor: 0.1,
                        emissivity: color * intensity,
                    }),
                }],
            },
            Light::Point(PointLight {
                color,
                range: 1000.0,
                intensity: 6.0,
            }),
            TransformBuilder::new()
                .position(position)
                .scale([0.25, 0.25, 0.25].into())
                .build(),
        ));
    };

    let lights: [(Point3<f32>, Vector3<f32>, f32); 4] = [
        ([0.0, 0.5, 1.5].into(), [1.0, 1.0, 1.0].into(), 2.0),
        ([-4.0, 0.5, -3.0].into(), [1.0, 0.0, 0.0].into(), 4.0),
        ([3.0, 0.5, 1.0].into(), [0.0, 0.0, 1.0].into(), 6.0),
        ([-0.8, 2.4, -1.0].into(), [0.0, 1.0, 0.0].into(), 2.5),
    ];

    for (position, color, intensity) in lights {
        spawn_light(position, color, intensity);
    }
}

fn main() {
    let config = AppConfig::default();

    let player_controller_settings = PlayerControllerSettings::new(5.0, 1.0, 9.81);

    let player_spawn_settings = PlayerSpawnSettings {
        initial_transform: Default::default(),
        controller_settings: player_controller_settings,
        free_cam_activated: true,
    };

    let application = ApplicationBuilder::new(config)
        .with_startup_system(spawn_bloom_demo)
        .with_player_controller(player_spawn_settings)
        .build();

    application.run();
}
