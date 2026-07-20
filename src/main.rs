use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
mod app_state;
mod elevator;
mod interactable;
mod moveable;
mod player;
mod ui;

use app_state::AppState;
use elevator::{spawn_elevator, spawn_floor_with_hole_for_elevator};
use interactable::{InteractablePlugin, Button};
use moveable::MoveablePlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            InteractablePlugin,
            MoveablePlugin,
            PlayerPlugin,
            UiPlugin
        ))
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(OnEnter(AppState::Quit), quit)
        .add_systems(Update, grab_cursor.run_if(in_state(AppState::InGame)))
        .run();
}

fn quit(mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
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

    let elevator = spawn_elevator(
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

    // "Button" 1
    commands.spawn((
        RigidBody::Static,
        Button{target: elevator, range: 5.0},
        Collider::cuboid(1.0, 1.5, 1.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.5, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
        Transform::from_xyz(-5.0, 0.1, 0.0),
    ));

    // "Button" 2
    commands.spawn((
        RigidBody::Static,
        Button{target: elevator, range: 5.0},
        Collider::cuboid(1.0, 1.5, 1.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.5, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
        Transform::from_xyz(-5.0, floor_height, 0.0),
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
