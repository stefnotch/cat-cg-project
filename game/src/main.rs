//#![windows_subsystem = "windows"]

use animations::animation::PlayingAnimation;
use app::plugin::{Plugin, PluginAppAccess};
use bevy_ecs::prelude::{Component, EventReader, Query, With};
use bevy_ecs::schedule::IntoSystemConfig;

use debug::setup_debugging;
use game::level_flags::LevelFlags;
use game::pickup_system::PickupPlugin;
use game_core::time::Time;
use game_core::time_manager::TimeManager;
use loader::config_loader::LoadableConfig;
use loader::loader::{Door, SceneLoader};
use scene::flag_trigger::FlagTrigger;
use scene::level::LevelId;
use scene::{
    mesh::CpuMesh,
    model::{CpuPrimitive, Model},
};
use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Instant;

use bevy_ecs::system::{Commands, Res, ResMut};
use nalgebra::{Point3, Translation3};

use game::core::application::{AppConfig, AppStage, Application};
use game::player::{PlayerControllerSettings, PlayerPlugin, PlayerSpawnSettings};

use physics::physics_context::{BoxCollider, RigidBody, RigidBodyType};
use physics::physics_events::{CollisionEvent, CollisionEventFlags};
use scene::transform::{Transform, TransformBuilder};

fn spawn_world(mut commands: Commands, scene_loader: Res<SceneLoader>) {
    let before = Instant::now();
    scene_loader
        .load_default_scene(
            "./assets/scene/testing/prototype/prototype.glb",
            &mut commands,
        )
        .unwrap();
    println!(
        "Loading the scene took {}sec",
        before.elapsed().as_secs_f64()
    );
}

fn setup_levels(mut level_flags: ResMut<LevelFlags>) {
    level_flags.set_count(LevelId::new(0), 1);
}

fn _print_fps(time: Res<Time>) {
    println!("{}", 1.0 / time.delta_seconds())
}

#[derive(Component)]
pub struct MovingBox;

pub fn spawn_moving_cube(mut commands: Commands) {
    let cube = CpuMesh::cube(1.0, 1.0, 1.0);

    commands.spawn((
        Transform::default(),
        Model {
            primitives: vec![CpuPrimitive {
                mesh: cube.clone(),
                material: Arc::new(Default::default()),
            }],
        },
        BoxCollider {
            bounds: cube.bounding_box.clone(),
        },
        RigidBody(RigidBodyType::KinematicPositionBased),
        MovingBox,
    ));
}

pub fn move_cubes(mut query: Query<&mut Transform, With<MovingBox>>, time: Res<TimeManager>) {
    let origin = Point3::origin();
    for mut move_body_position in query.iter_mut() {
        let shift = Translation3::new(
            -4.0,
            1.0,
            5.0 * (time.level_time_seconds() * PI / 2.0 * 0.5).sin(),
        );
        move_body_position.position = shift.transform_point(&origin);
    }
}

// TODO: Maybe refactor the CollisionEvents design, since it doesn't work well with the rest of Bevy ECS.
// TODO: Maybe refactor the flags to be more type safe? Like using a struct for each flag? And maybe letting entities "subscribe" to a flag being changed?
fn flag_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut level_flags: ResMut<LevelFlags>,
    flag_triggers: Query<&FlagTrigger>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, CollisionEventFlags::SENSOR) = collision_event {
            if let Ok(flag_trigger) = flag_triggers
                .get(*e1)
                .or_else(|_err| flag_triggers.get(*e2))
            {
                level_flags.set(flag_trigger.level_id, flag_trigger.flag_id, true);
            }
        }
    }
}

fn door_system(
    level_flags: Res<LevelFlags>,
    time: Res<TimeManager>,
    mut query: Query<&mut PlayingAnimation, With<Door>>,
) {
    if level_flags.get(LevelId::new(0), 0) {
        let mut animation = query.single_mut();
        animation.play_forwards(time.level_time());
    }
}

struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&mut self, app: &mut PluginAppAccess) {
        app.with_startup_system(spawn_world)
            .with_startup_system(setup_levels)
            .with_startup_system(spawn_moving_cube)
            .with_plugin(PickupPlugin)
            .with_system(flag_system.in_set(AppStage::Update))
            .with_system(door_system.in_set(AppStage::Update).after(flag_system))
            .with_system(move_cubes.in_set(AppStage::Update));
    }
}

fn main() {
    let _guard = setup_debugging();

    // Only the main project actually loads the config from the file
    let config: AppConfig = LoadableConfig::load("./assets/config.json").into();

    let player_spawn_settings = PlayerSpawnSettings {
        initial_transform: TransformBuilder::new()
            .position([0.0, 1.0, 3.0].into())
            .build(),
        controller_settings: PlayerControllerSettings::default()
            .with_sensitivity(config.mouse_sensitivity),
        free_cam_activated: false,
    };

    let mut application = Application::new(config);
    application
        .app
        .with_plugin(GamePlugin)
        .with_plugin(PlayerPlugin::new(player_spawn_settings));

    application.run();
}
