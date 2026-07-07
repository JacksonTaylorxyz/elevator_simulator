use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
mod player;
use player::PlayerPlugin;

#[derive(Component)]
struct Elevator {
    min_travel_height: f32,
    max_travel_height: f32,
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), PlayerPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                grab_cursor,
                elevator_move,
            ),
        )
        .run();
}

fn spawn_floor_with_hole_for_elevator(
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

fn spawn_elevator(
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_y = 5.0;
    let floor_thickness = 0.01;
    let floor_width = 50.0;
    let elevator_floor_size = 5.0;
    let elevator_speed = 3.0;
    let floor_height = 5.0;

    // Ground Floor
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(floor_width, floor_thickness, floor_width),
        Mesh3d(meshes.add(Cuboid::new(floor_width, floor_thickness, floor_width))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Restitution::new(0.7),
    ));

    let blue_slab_material = materials.add(Color::srgb(0.0, 0.0, 1.0));
    let purple_material = MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 1.0)));

    // First floor
    spawn_floor_with_hole_for_elevator(
        &mut commands,
        &mut meshes,
        floor_y * 1.0, // First floor
        floor_thickness,
        floor_width,
        elevator_floor_size,
        blue_slab_material,
    );

    spawn_elevator(
        &mut commands,
        &mut meshes,
        &purple_material,
        elevator_floor_size,
        floor_thickness,
        floor_height,
        floor_thickness,
        0.0,
        floor_height,
        elevator_speed,
    );
}

fn grab_cursor(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut window_cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        window_cursor.grab_mode = CursorGrabMode::Locked;
        window_cursor.visible = false;
    }
    if keys.just_pressed(KeyCode::Escape) {
        window_cursor.grab_mode = CursorGrabMode::None;
        window_cursor.visible = true;
    }
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
