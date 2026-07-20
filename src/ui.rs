use bevy::{
    prelude::*,
    text::Underline,
};
use super::app_state::AppState;

const GAME_TITLE: &str = "Elevator Simulator";

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
struct ColorChanging {
    pub base: Color,
    pub hovered: Color,
}

fn color_changing(base: Color, hovered: Color) -> impl Bundle {
    (
        ColorChanging { base, hovered },
        BackgroundColor(base),
    )
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(Update, (button_system, button_color_system).run_if(in_state(AppState::MainMenu)));
    }
}

#[derive(Component)]
struct MainMenuButton(AppState);

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((Camera2d, MainMenuRoot));

    let button_box_width = Val::Px(300.0);
    let button_box_height = Val::Px(65.0);
    let text_justification = Justify::Center;

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new(GAME_TITLE),
                    Underline,
                    TextShadow::default(),
                    TextLayout::new(text_justification, LineBreak::WordBoundary),
                    Node {
                        width: button_box_width,
                        height: button_box_height,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ));
            parent
                .spawn((
                    Button,
                    MainMenuButton(AppState::InGame),
                    Node {
                        width: button_box_width,
                        height: button_box_height,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color_changing(Color::srgb(0.2, 0.2, 0.2), Color::srgb(0.35, 0.35, 0.35)),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Start"));
                });
            parent
                .spawn((
                    Button,
                    MainMenuButton(AppState::Quit),
                    Node {
                        width: button_box_width,
                        height: button_box_height,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color_changing(Color::srgb(0.2, 0.2, 0.2), Color::srgb(0.35, 0.35, 0.35)),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Exit"));
                });
        });
}

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn button_color_system(
    mut query: Query<
        (&Interaction, &ColorChanging, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, colors, mut background) in &mut query {
        background.0 = match interaction {
            Interaction::Hovered | Interaction::Pressed => colors.hovered,
            Interaction::None => colors.base,
        };
    }
}

fn button_system(
    interaction_query: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, main_menu_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(main_menu_button.0);
        }
    }
}
