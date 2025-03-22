use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use std::collections::HashMap;

use crate::utils::create_directional_animations;

#[derive(Component)]
pub struct Player {
    direction: Vec3,
    last_direction: &'static str,
    speed: f32,
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
        AnimationDuration::PerFrame(250),
        AnimationRepeat::Loop,
    );

    commands.spawn((
        Player {
            direction: Vec3::ZERO,
            last_direction: "down",
            speed: 100.0,
        },
        Sprite::from_atlas_image(
            texture_handle,
            TextureAtlas {
                layout: atlas_layouts.add(spritesheet.atlas_layout(32, 32)),
                ..default()
            },
        ),
        Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::splat(5.0),
            ..default()
        },
        SpritesheetAnimation::from_id(library.animation_with_name("idle_down").unwrap()),
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut SpritesheetAnimation)>,
    library: Res<AnimationLibrary>,
) {
    if let Ok((mut player, mut transform, mut animation)) = query.get_single_mut() {
        player.direction = Vec3::ZERO;

        let directions = HashMap::from([
            ("up", (KeyCode::ArrowUp, Vec3::Y)),
            ("down", (KeyCode::ArrowDown, -Vec3::Y)),
            ("right", (KeyCode::ArrowRight, Vec3::X)),
            ("left", (KeyCode::ArrowLeft, -Vec3::X)),
        ]);

        for (anim_suffix, (key, direction)) in directions.iter() {
            if keys.pressed(*key) {
                player.direction += direction;

                if let Some(run_animation_id) =
                    library.animation_with_name(&format!("run_{}", player.last_direction))
                {
                    if animation.animation_id != run_animation_id
                        || !keys.pressed(directions[player.last_direction].0)
                    {
                        player.last_direction = anim_suffix;
                        animation.switch(run_animation_id);
                        if direction.x != 0. {
                            transform.scale.x = transform.scale.x.abs() * direction.x;
                        }
                    }
                }
            }
        }

        if player.direction.length() > 0. {
            player.direction = player.direction.normalize_or_zero();
            transform.translation += time.delta_secs() * player.direction * player.speed;
        } else {
            if let Some(idle_animation_id) =
                library.animation_with_name(&format!("idle_{}", player.last_direction))
            {
                if animation.animation_id != idle_animation_id {
                    animation.switch(idle_animation_id);
                }
            }
        }
    }
}
