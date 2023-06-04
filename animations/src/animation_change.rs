use bevy_ecs::query::Changed;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ecs::system::ResMut;
use bevy_ecs::world::Mut;
use std::collections::HashMap;
use time::time_manager::game_change::GameChange;
use time::time_manager::game_change::GameChangeHistory;
use time::time_manager::game_change::InterpolationType;
use time::time_manager::level_time::LevelTime;
use time::time_manager::TimeManager;
use time::time_manager::TimeTrackedId;

use crate::animation::PlayingAnimation;

#[derive(Debug)]
pub struct PlayingAnimationChange {
    pub(crate) id: TimeTrackedId,
    pub(crate) end_time: LevelTime,
    pub(crate) reverse: bool,
}

impl GameChange for PlayingAnimationChange {}

pub(super) fn animations_track(
    mut history: ResMut<GameChangeHistory<PlayingAnimationChange>>,
    query: Query<&PlayingAnimation, Changed<PlayingAnimation>>,
) {
    for animation in &query {
        history.add_command(PlayingAnimationChange {
            id: animation.id,
            end_time: animation.end_time,
            reverse: animation.reverse,
        });
    }
}

pub(crate) fn animations_rewind(
    time_manager: Res<TimeManager>,
    mut history: ResMut<GameChangeHistory<PlayingAnimationChange>>,
    mut query: Query<&mut PlayingAnimation>,
) {
    let mut entities: HashMap<_, Mut<PlayingAnimation>> = query
        .iter_mut()
        .map(|animation| (animation.id, animation))
        .collect();

    let (commands, _interpolation) =
        history.take_commands_to_apply(&time_manager, InterpolationType::None);

    for command_collection in commands {
        for command in command_collection.commands {
            if let Some(v) = entities.get_mut(&command.id) {
                v.end_time = command.end_time;
                v.reverse = command.reverse;
            }
        }
    }
}
