use avian3d::prelude::*;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

#[derive(Component)]
struct Player {
    yaw: f32,
}

#[derive(Component)]
struct ThirdPersonCamera {
    pitch: f32,
    distance: f32,
}

#[derive(Component)]
struct Elevator(f32);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                grab_cursor,
                player_look,
                player_move,
                camera_follow,
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_y = 5.0;
    let floor_thickness = 0.01;
    let floor_width = 100.0;
    let elevator_floor_size = 10.0;
    let elevator_speed = 1.0;
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

    // Elevator platform
    commands.spawn((
        // Visuals
        Mesh3d(meshes.add(Cuboid::new(
            elevator_floor_size,
            floor_thickness,
            elevator_floor_size,
        ))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 1.0))), // Purple
        Transform::from_xyz(0.0, 0.1, 0.0),

        // Collision logic
        Elevator(floor_height),
        RigidBody::Kinematic,
        Collider::cuboid(elevator_floor_size, floor_thickness, elevator_floor_size),
        LinearVelocity(Vec3::new(0.0, elevator_speed, 0.0)),
    ));

    // Player
    commands.spawn((
        Player { yaw: 0.0 },
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        LockedAxes::ROTATION_LOCKED,
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));

    // Player camera
    commands.spawn((
        ThirdPersonCamera {
            pitch: -0.3,
            distance: 6.0,
        },
        Camera3d::default(),
        Transform::default(),
        DirectionalLight::default(),
    ));
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

fn player_look(
    mut motion: MessageReader<MouseMotion>,
    mut player: Single<(&mut Transform, &mut Player), With<Player>>,
    mut cam: Single<&mut ThirdPersonCamera>,
    cursor: Single<&CursorOptions, With<PrimaryWindow>>,
) {
    if cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let sensitivity = 0.002;

    for ev in motion.read() {
        // Rotate the player body left/right so movement stays aligned
        player.1.yaw -= ev.delta.x * sensitivity;
        player.0.rotation = Quat::from_rotation_y(player.1.yaw);

        // Tilt the camera up/down
        cam.pitch = (cam.pitch - ev.delta.y * sensitivity).clamp(-0.8, 0.4);
    }
}

fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&Transform, &mut LinearVelocity), With<Player>>,
) {
    let (transform, mut vel) = player.single_mut().unwrap();
    let normal_speed = 5.0;
    let shift_speed = 7.5;

    let speed = if keys.pressed(KeyCode::ShiftLeft) {
        shift_speed
    } else {
        normal_speed
    };

    let mut dir = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        dir += *transform.forward();
    }
    if keys.pressed(KeyCode::KeyS) {
        dir += *transform.back();
    }
    if keys.pressed(KeyCode::KeyA) {
        dir += *transform.left();
    }
    if keys.pressed(KeyCode::KeyD) {
        dir += *transform.right();
    }

    if dir != Vec3::ZERO {
        let flat = Vec3::new(dir.x, 0.0, dir.z).normalize() * speed;
        vel.x = flat.x;
        vel.z = flat.z;
    } else {
        vel.x = 0.0;
        vel.z = 0.0;
    }
    // leave vel.y alone so gravity works
}

fn camera_follow(
    player: Single<&Transform, With<Player>>,
    mut cam: Query<(&ThirdPersonCamera, &mut Transform), Without<Player>>,
) {
    let (cam_settings, mut cam_t) = cam.single_mut().unwrap();

    // Orbit offset in spherical coordinates around the player
    let yaw = player.rotation.to_euler(EulerRot::YXZ).0;
    let offset = Quat::from_rotation_y(yaw)
        * Quat::from_rotation_x(cam_settings.pitch)
        * Vec3::new(0.0, 0.0, cam_settings.distance);

    let target = player.translation + Vec3::Y * 1.0; // look at chest height
    cam_t.translation = target + offset;
    cam_t.look_at(target, Vec3::Y);
}

fn elevator_move(mut query: Query<(&Elevator, &mut LinearVelocity, &Transform)>, _time: Res<Time>) {
    for (elevator, mut vel, transform) in &mut query {
        if transform.translation.y >= elevator.0 || transform.translation.y <= 0.0 {
            vel.y = -vel.y;
        }
    }
}
