use bevy::prelude::*;

use crate::player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game.after(player::setup_player))
            .add_systems(Update, update_game)
            .add_plugins(player::PlayerPlugin);
    }
}

fn setup_game(mut commands: Commands, player_query: Query<&Transform, With<player::Player>>) {
    if let Ok(player_pos) = player_query.get_single() {
        commands.spawn((
            Transform::from_translation(player_pos.translation),
            Camera2d,
        ));
    }
}

fn update_game(
    time: Res<Time>,
    player_query: Query<&Transform, With<player::Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<player::Player>)>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        if let Ok(mut camera_pos) = camera_query.get_single_mut() {
            let current_pos = camera_pos.translation;
            let target_pos = player_pos.translation;

            let lerp_factor = 5.0 * time.delta_secs();

            camera_pos.translation = current_pos + (target_pos - current_pos) * lerp_factor;
        }
    }
}
