use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    direction: Vec3,
    speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, move_player);
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0., 0., 0.)),
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        Player {
            direction: Vec3::ZERO,
            speed: 200.0,
        },
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = query.single_mut();

    player.direction = Vec3::ZERO;

    if keys.pressed(KeyCode::ArrowUp) {
        player.direction.y += 1.;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        player.direction.y -= 1.;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        player.direction.x -= 1.;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        player.direction.x += 1.;
    }

    player.direction = player.direction.normalize_or_zero();
    transform.translation += time.delta_secs() * player.direction * player.speed;
}
