use avian3d::prelude::*;
use bevy::{
    prelude::*,
    input::mouse::MouseMotion,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

pub struct PlayerPlugin;

#[derive(Component)]
struct Player {
    yaw: f32,
}

#[derive(Component)]
struct ThirdPersonCamera {
    pitch: f32,
    distance: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                (
                player_look,
                player_move,
                camera_follow,
                )
            )
            .add_systems(Startup,
                (
                spawn_player,
                )
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
