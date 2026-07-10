use bevy:: prelude::*;
use super::player::Player;

#[derive(Component)]
pub struct Button {
    pub target: Entity,
    pub range: f32,
}

#[derive(Message)]
pub struct Interact {
    pub target: Entity,
}

pub struct InteractablePlugin;
impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_interact)
            .add_message::<Interact>() ;

    }
}

fn button_interact(
    keys: Res<ButtonInput<KeyCode>>,
    player: Single<&Transform, With<Player>>,
    buttons: Query<(&Button, &Transform)>,
    mut writer: MessageWriter<Interact>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    for (button, button_transform) in &buttons {
        let dist = player.translation.distance(button_transform.translation);
        if dist <= button.range {
            info!("player hit button in range");
            writer.write(Interact { target: button.target });
        }
    }
}

