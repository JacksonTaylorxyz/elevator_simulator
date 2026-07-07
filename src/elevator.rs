use avian3d::prelude::*;
use bevy:: prelude::*;

#[derive(Component)]
struct Elevator {
    min_travel_height: f32,
    max_travel_height: f32,
    speed: f32,
}

pub struct ElevatorPlugin;
impl Plugin for ElevatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, elevator_move);

    }
}

pub fn spawn_floor_with_hole_for_elevator(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    floor_y: f32,
    floor_thickness: f32,
    total_floor_width: f32,
    elevator_hole_width: f32,
    slab_material: Handle<StandardMaterial>,
) {
    /* |======================floor size========================| =
     * |-------------------||---------------||-------------------| ^
     * |-------------------||---------------||-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||--back slab----||-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||<elevator size>||-------------------| |
     * |-------------------|                 |-------------------| |
     * |-------------------|                 |-------------------| |
     * |-----left slab-----|                 |--right slab slab--| floor size
     * |-------------------|                 |-------------------| |
     * |-------------------|                 |-------------------| |
     * |-------------------|                 |-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||--front slab---||-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||---------------||-------------------| |
     * |-------------------||---------------||-------------------| v
     * |-------------------||---------------||-------------------| =
     */

    let half_of_hole_size = elevator_hole_width / 2.0;

    let edge_to_hole_distance = (total_floor_width / 2.0) - half_of_hole_size;

    commands.spawn((
        RigidBody::Static,
        MeshMaterial3d(slab_material.clone()),
        Restitution::new(0.7),
        children![
            // Left slab
            (
                Collider::cuboid(edge_to_hole_distance, floor_thickness, total_floor_width),
                Transform::from_xyz(
                    -(half_of_hole_size + (edge_to_hole_distance / 2.0)),
                    floor_y,
                    0.0
                ),
                Mesh3d(meshes.add(Cuboid::new(
                    edge_to_hole_distance,
                    floor_thickness,
                    total_floor_width,
                ))),
                MeshMaterial3d(slab_material.clone()),
            ),
            // Right Slab
            (
                RigidBody::Static,
                Collider::cuboid(edge_to_hole_distance, floor_thickness, total_floor_width),
                Transform::from_xyz(
                    half_of_hole_size + (edge_to_hole_distance / 2.0),
                    floor_y,
                    0.0
                ),
                Mesh3d(meshes.add(Cuboid::new(
                    edge_to_hole_distance,
                    floor_thickness,
                    total_floor_width,
                ))),
                MeshMaterial3d(slab_material.clone()),
            ),
            // Front Slab
            (
                RigidBody::Static,
                Collider::cuboid(elevator_hole_width, floor_thickness, edge_to_hole_distance,),
                Transform::from_xyz(
                    0.0,
                    floor_y,
                    half_of_hole_size + (edge_to_hole_distance / 2.0)
                ),
                Mesh3d(meshes.add(Cuboid::new(
                    elevator_hole_width,
                    floor_thickness,
                    edge_to_hole_distance,
                ))),
                MeshMaterial3d(slab_material.clone()),
            ),
            // Back Slab
            (
                RigidBody::Static,
                Collider::cuboid(elevator_hole_width, floor_thickness, edge_to_hole_distance,),
                Transform::from_xyz(
                    0.0,
                    floor_y,
                    -(half_of_hole_size + (edge_to_hole_distance / 2.0))
                ),
                Mesh3d(meshes.add(Cuboid::new(
                    elevator_hole_width,
                    floor_thickness,
                    edge_to_hole_distance
                ))),
                MeshMaterial3d(slab_material.clone()),
            )
        ],
    ));
}

pub fn spawn_elevator(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &MeshMaterial3d<StandardMaterial>,
    elevator_floor_width: f32,
    elevator_floor_thickness: f32,
    elevator_wall_height: f32,
    elevator_wall_thickness: f32,
    min_elevator_height: f32,
    max_elevator_height: f32,
    elevator_speed: f32,
) {
    // Elevator
    commands.spawn((
        Elevator {
            min_travel_height: min_elevator_height,
            max_travel_height: max_elevator_height,
            speed: elevator_speed
        },
        LinearVelocity(Vec3::new(0.0, elevator_speed, 0.0)),
        RigidBody::Kinematic,
        children![
            // Floor
            (
                // Visuals
                Mesh3d(meshes.add(Cuboid::new(
                    elevator_floor_width,
                    elevator_floor_thickness,
                    elevator_floor_width,
                ))),
                (*material).clone(),
                Transform::from_xyz(0.0, elevator_floor_thickness, 0.0),
                Collider::cuboid(elevator_floor_width, elevator_floor_thickness, elevator_floor_width),
            ),
            // Left Wall
            (
                // Visuals
                Mesh3d(meshes.add(Cuboid::new(
                    elevator_wall_thickness,
                    elevator_wall_height,
                    elevator_floor_width,
                ))),
                (*material).clone(),
                Transform::from_xyz(-(elevator_floor_width / 2.0), max_elevator_height / 2.0 + elevator_floor_thickness, 0.0),
                Collider::cuboid(elevator_floor_thickness, elevator_wall_height, elevator_floor_width),
            ),
            // Right Wall
            (
                // Visuals
                Mesh3d(meshes.add(Cuboid::new(
                    elevator_wall_thickness,
                    elevator_wall_height,
                    elevator_floor_width,
                ))),
                (*material).clone(),
                Transform::from_xyz(elevator_floor_width / 2.0, max_elevator_height / 2.0 + elevator_floor_thickness, 0.0),
                Collider::cuboid(elevator_floor_thickness, elevator_wall_height, elevator_floor_width),
            ),
            // Back Wall
            (
                // Visuals
                Mesh3d(meshes.add(Cuboid::new(
                    elevator_floor_width,
                    elevator_wall_height,
                    elevator_wall_thickness,
                ))),
                (*material).clone(),
                Transform::from_xyz(0.0, max_elevator_height / 2.0 + elevator_floor_thickness, elevator_floor_width / 2.0),
                Collider::cuboid(elevator_floor_width, elevator_wall_height, elevator_floor_thickness),
            ),
            // ceiling
            (
                // Visuals
                Mesh3d(meshes.add(Cuboid::new(
                    elevator_floor_width,
                    elevator_floor_thickness,
                    elevator_floor_width,
                ))),
                (*material).clone(),
                Transform::from_xyz(0.0, elevator_floor_thickness + max_elevator_height, 0.0),
                Collider::cuboid(elevator_floor_width, elevator_floor_thickness, elevator_floor_width),
            ),
        ]
    ));
}

fn elevator_move(mut query: Query<(&Elevator, &mut LinearVelocity, &Transform)>, _time: Res<Time>) {
    for (elevator, mut vel, transform) in &mut query {
        if transform.translation.y >= elevator.max_travel_height {
            vel.y = -elevator.speed;
        } else if transform.translation.y <= elevator.min_travel_height {
            vel.y = elevator.speed;
        }
    }
}

