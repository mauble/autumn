use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use std::collections::HashMap;

use crate::utils::create_directional_animations;

#[derive(Component)]
pub struct Player {
    speed: f32,
    direction: Vec3,
    anim_type: &'static str,
    anim_direction: &'static str,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, move_player)
            .add_plugins(SpritesheetAnimationPlugin::default());
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    let texture_handle = assets.load("textures/characters/player.png");
    let spritesheet = Spritesheet::new(4, 12);

    create_directional_animations(
        &mut library,
        "idle",
        HashMap::from([
            ("down", (0, 1)),
            ("up", (8, 9)),
            ("right", (4, 5)),
            ("left", (4, 5)),
        ]),
        AnimationDuration::PerFrame(1000),
        AnimationRepeat::Loop,
    );

    create_directional_animations(
        &mut library,
        "run",
        HashMap::from([
            ("down", (12, 15)),
            ("up", (20, 23)),
            ("right", (16, 19)),
            ("left", (16, 19)),
        ]),
        AnimationDuration::PerFrame(220),
        AnimationRepeat::Loop,
    );

    commands.spawn((
        Player {
            speed: 225.,
            direction: Vec3::ZERO,
            anim_type: "idle",
            anim_direction: "down",
        },
        Sprite::from_atlas_image(
            texture_handle,
            TextureAtlas {
                layout: atlas_layouts.add(spritesheet.atlas_layout(32, 32)),
                ..default()
            },
        ),
        Transform {
            translation: Vec3::new(0., 0., 1.),
            scale: Vec3::splat(5.0),
            ..default()
        },
        SpritesheetAnimation::from_id(library.animation_with_name("idle_down").unwrap()),
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    library: Res<AnimationLibrary>,
    mut query: Query<(&mut Player, &mut Transform, &mut SpritesheetAnimation)>,
) {
    if let Ok((mut player, mut transform, mut animation)) = query.get_single_mut() {
        player.direction = Vec3::ZERO;

        let directions = [
            (KeyCode::ArrowLeft, -Vec3::X),
            (KeyCode::ArrowRight, Vec3::X),
            (KeyCode::ArrowUp, Vec3::Y),
            (KeyCode::ArrowDown, -Vec3::Y),
        ];

        for (key, direction) in directions.iter() {
            if keys.pressed(*key) {
                player.direction += *direction;
            }
        }

        if player.direction == Vec3::ZERO {
            player.anim_type = "idle";
        } else {
            player.anim_type = "run";
            player.anim_direction = match player.direction.x.abs() > player.direction.y.abs() {
                true => {
                    if player.direction.x > 0.0 {
                        "right"
                    } else {
                        "left"
                    }
                }
                false => {
                    if player.direction.y > 0.0 {
                        "up"
                    } else {
                        "down"
                    }
                }
            };

            player.direction = player.direction.normalize_or_zero();
            transform.translation += time.delta_secs() * player.direction * player.speed;
        }

        if let Some(animation_id) =
            library.animation_with_name(&format!("{}_{}", player.anim_type, player.anim_direction))
        {
            if animation.animation_id != animation_id {
                animation.switch(animation_id);
                if player.anim_direction == "left" {
                    transform.scale.x = transform.scale.x.abs() * -1.;
                } else {
                    transform.scale.x = transform.scale.x.abs();
                }
            }
        }
    }
}
