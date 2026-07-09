use avian3d::prelude::*;
use bevy:: prelude::*;

pub enum MoveBetweenPointsState {
    Idle,
    MovingForward,
    MovingBackward
}

#[derive(Component)]
pub struct MoveBetweenPoints {
    pub from: Vec3,
    pub to: Vec3,
    pub speed: f32,
    pub state: MoveBetweenPointsState
}

pub struct MoveablePlugin;
impl Plugin for MoveablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_moveable);

    }
}

impl MoveBetweenPoints {
    pub fn new(from: Vec3, to: Vec3, speed: f32) -> Self {
        MoveBetweenPoints {
            from,
            to,
            speed,
            state: MoveBetweenPointsState::Idle
        }
    }
    pub fn move_forwards(&mut self) {
        self.state = MoveBetweenPointsState::MovingForward
    }

    pub fn move_backwards(&mut self) {
        self.state = MoveBetweenPointsState::MovingBackward
    }

    pub fn stop(&mut self) {
        self.state = MoveBetweenPointsState::Idle
    }
}




fn move_moveable(
    mut query: Query<(&mut MoveBetweenPoints, &mut LinearVelocity, &mut Transform)>,
) {
    // a threshold to account for being off by a small amount. If we're within this threshold
    // we'll snap to the end point and stop.
    let arrival_threshold = 0.05;

    for (mut mbp, mut vel, mut transform) in &mut query {
        match mbp.state {
            MoveBetweenPointsState::Idle => {
                // stop movement
                vel.0 = Vec3::ZERO;
            }
            MoveBetweenPointsState::MovingForward => {
                let to_target = mbp.to - transform.translation;
                let distance = to_target.length();

                if distance < arrival_threshold {
                    transform.translation = mbp.to;
                    vel.0 = Vec3::ZERO;
                    mbp.stop();
                } else {
                    vel.0 = to_target.normalize() * mbp.speed;
                }
            }
            MoveBetweenPointsState::MovingBackward => {
                let to_target = mbp.from - transform.translation;
                let distance = to_target.length();

                if distance < arrival_threshold {
                    transform.translation = mbp.from;
                    vel.0 = Vec3::ZERO;
                    mbp.stop();
                } else {
                    vel.0 = to_target.normalize() * mbp.speed;
                }
            }
        }
    }
}
