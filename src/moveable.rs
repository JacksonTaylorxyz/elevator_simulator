use avian3d::prelude::*;
use bevy:: prelude::*;
use super::interactable::Interact;

#[derive(Copy, Clone)]
pub enum MoveBetweenPointsState {
    Idle,
    MovingForward,
    MovingBackward
}

#[derive(Copy, Clone)]
pub enum MoveBetweenPointsPosition {
    InMiddle, // somewhere in the middle, not necessarily *the* middle
    AtFrom,
    AtTo
}

#[derive(Component)]
pub struct MoveBetweenPoints {
    pub from: Vec3,
    pub to: Vec3,
    pub speed: f32,
    pub state: MoveBetweenPointsState,
    pub prev_state: Option<MoveBetweenPointsState>,
    pub position: MoveBetweenPointsPosition
}

pub struct MoveablePlugin;
impl Plugin for MoveablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_moveable)
            .add_systems(Update, handle_interact);

    }
}

impl MoveBetweenPoints {
    pub fn new(from: Vec3, to: Vec3, speed: f32) -> Self {
        MoveBetweenPoints {
            from,
            to,
            speed,
            state: MoveBetweenPointsState::Idle,
            prev_state: None,
            position: MoveBetweenPointsPosition::InMiddle
        }
    }
    pub fn move_forwards(&mut self) {
        self.save_prev_state();
        self.state = MoveBetweenPointsState::MovingForward
    }

    pub fn move_backwards(&mut self) {
        self.save_prev_state();
        self.state = MoveBetweenPointsState::MovingBackward
    }

    pub fn resume(&mut self) {
        match self.state {
            MoveBetweenPointsState::Idle => {
                match self.prev_state {
                    None => self.move_forwards(),
                    Some(MoveBetweenPointsState::Idle) => {
                        match self.position {
                            MoveBetweenPointsPosition::InMiddle => {
                                self.move_forwards();
                            },
                            MoveBetweenPointsPosition::AtFrom => {
                                self.move_forwards();
                            },
                            MoveBetweenPointsPosition::AtTo => {
                                self.move_backwards();
                            },
                        }
                    },
                    Some(MoveBetweenPointsState::MovingForward) => {
                        match self.position {
                            MoveBetweenPointsPosition::InMiddle => {
                                self.move_forwards();
                            },
                            MoveBetweenPointsPosition::AtFrom => {
                                self.move_forwards();
                            },
                            MoveBetweenPointsPosition::AtTo => {
                                self.move_backwards();
                            },
                        }
                    },
                    Some(MoveBetweenPointsState::MovingBackward) => {
                        match self.position {
                            MoveBetweenPointsPosition::InMiddle => {
                                self.move_backwards();
                            },
                            MoveBetweenPointsPosition::AtFrom => {
                                self.move_forwards();
                            },
                            MoveBetweenPointsPosition::AtTo => {
                                self.move_backwards();
                            },
                        }
                    },
                }
            },
            _ => {}
        }
    }

    pub fn stop(&mut self) {
        self.save_prev_state();
        self.state = MoveBetweenPointsState::Idle
    }

    fn save_prev_state(&mut self) {
        self.prev_state = Some(self.state);
    }

    fn at_to(&mut self) {
        self.position = MoveBetweenPointsPosition::AtTo
    }

    fn at_from(&mut self) {
        self.position = MoveBetweenPointsPosition::AtFrom
    }

    fn in_middle(&mut self) {
        self.position = MoveBetweenPointsPosition::InMiddle
    }
}

fn handle_interact(
    mut reader: MessageReader<Interact>,
    mut moveables: Query<&mut MoveBetweenPoints>,
) {
    for event in reader.read() {
        let Ok(mut mbp) = moveables.get_mut(event.target) else {
            continue;
        };

        match mbp.state {
            MoveBetweenPointsState::Idle => {
                mbp.resume();
            }
            _ => {}
        };
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
                    mbp.at_to();
                } else {
                    mbp.in_middle();
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
                    mbp.at_from();
                } else {
                    mbp.in_middle();
                    vel.0 = to_target.normalize() * mbp.speed;
                }
            }
        }
    }
}
