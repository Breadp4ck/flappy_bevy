use bevy::prelude::*;

// FIXME This is a crap, SOLID violation.
// The problem is in the [BirdPlugin] [crate::game::plugins::obstacle_collision_check].
// When restarting the game, collision events are not handled with [GameState] change.
// So, there are situations when player starts with additional scores. So I split scores
// into 2 parts to caclculate the begining and next scores. It's happens in [crate::game::plugins::bird::reset]
// and in the [crate::game::plugins::obstacle_collision_check]
#[derive(Resource, Default)]
pub struct Scores {
    /// Scores the player get the moment he enter doorway
    pub maybe: usize,
    /// Scores the player get the moment he exit doorway
    pub decided: usize,
}

#[derive(Default, Component)]
pub struct Bird;

#[derive(Default, Component)]
pub struct Pipe;

#[derive(Default, Component)]
pub struct Border;

#[derive(Default, Component)]
pub struct Obstacle;

#[derive(Default, Component)]
pub struct Doorway;
