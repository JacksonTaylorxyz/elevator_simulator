use avian3d::prelude::*;
use bevy:: prelude::*;
use std::f32::consts::PI;


use super::moveable::MoveBetweenPoints;

#[derive(Component)]
struct Elevator;

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
    elevator_floor_width: f32,
    elevator_floor_thickness: f32,
    elevator_wall_height: f32,
    elevator_wall_thickness: f32,
    min_elevator_height: f32,
    max_elevator_height: f32,
    elevator_speed: f32,
    asset_server: Res<AssetServer>,
)-> Entity {
    // Since the middle of the walls are at 0 on the Y, we need to shift them up half their height
    // plus the amount of the floor.
    let wall_start_height = max_elevator_height / 2.0 + elevator_floor_thickness;

    // Height of where the ceiling should be
    let ceiling_start_height = max_elevator_height + elevator_floor_thickness;

    let elevator_asset = asset_server.load("elevator.glb#Scene0");

    // Elevator
    commands.spawn((
        Elevator,
        MoveBetweenPoints::new(
            Vec3::new(0.0, min_elevator_height, 0.0),
            Vec3::new(0.0, max_elevator_height, 0.0),
            elevator_speed
        ),
        LinearVelocity(Vec3::new(0.0, 0.0, 0.0)),
        RigidBody::Kinematic,
        SceneRoot(elevator_asset),
        Transform::from_xyz(0.0, 0.01, 0.0)
            .with_scale(Vec3::splat(5.0))
            .with_rotation(Quat::from_rotation_y(PI)),
        children![
            // Floor
            (
                Transform::from_xyz(0.0, elevator_floor_thickness, 0.0),
                Collider::cuboid(elevator_floor_width, elevator_floor_thickness, elevator_floor_width),
            ),
            // Left Wall
            (
                Transform::from_xyz(-(elevator_floor_width / 2.0), wall_start_height, 0.0),
                Collider::cuboid(elevator_wall_thickness, elevator_wall_height, elevator_floor_width),
            ),
            // Right Wall
            (
                Transform::from_xyz(elevator_floor_width / 2.0, wall_start_height, 0.0),
                Collider::cuboid(elevator_wall_thickness, elevator_wall_height, elevator_floor_width),
            ),
            // Back Wall
            (
                Transform::from_xyz(0.0, wall_start_height, elevator_floor_width / 2.0),
                Collider::cuboid(elevator_floor_width, elevator_wall_height, elevator_wall_thickness),
            ),
            // ceiling
            (
                Transform::from_xyz(0.0, ceiling_start_height, 0.0),
                Collider::cuboid(elevator_floor_width, elevator_floor_thickness, elevator_floor_width),
            ),
        ]
    )).id()
}
