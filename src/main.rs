use bevy::prelude::*;

#[derive(Component)]
struct Background {
    speed: f32,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Ship;

const SCALE: f32 = 3.;
const WIDTH: f32 = 256. * SCALE;
const HEIGHT: f32 = 192. * SCALE;
const BG_HEIGHT: f32 = 608.;
const SCROLL_SPEED: f32 = -1. * SCALE;
const SHIP_SPEED: f32 = 100. * SCALE;
const TIME_STEP: f32 = 1.0 / 60.0;
const RIGHT_BOUND: f32 = WIDTH / 2. - (17. * SCALE / 2.);
const LEFT_BOUND: f32 = -(WIDTH / 2. - (17. * SCALE / 2.));

fn main() {
    App::new() // prevents blurry sprites
        .add_startup_system(setup)
        .add_systems((scroll_background, animate_sprite, move_ship))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Shoot Them Up 3".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .run();
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                indices.last
            };
        }
    }
}

fn move_ship(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Ship>>) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position = paddle_transform.translation.x + direction * SHIP_SPEED * TIME_STEP;

    paddle_transform.translation.x = new_paddle_position.clamp(LEFT_BOUND, RIGHT_BOUND);
}

fn scroll_background(mut query: Query<(&mut Transform, &Background)>) {
    for (mut transform, background) in query.iter_mut() {
        transform.translation.y += background.speed;

        if transform.translation.y <= -(BG_HEIGHT * SCALE) {
            // If the background is fully out of view, reset its position.
            transform.translation.y = BG_HEIGHT * SCALE;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let background_texture = asset_server.load("desert-backgorund-looped.png");

    // Spawn two background sprites
    for i in 0..2 {
        commands
            .spawn(SpriteBundle {
                texture: background_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(0., BG_HEIGHT * SCALE * i as f32, 0.),
                    scale: Vec3::splat(SCALE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Background {
                speed: SCROLL_SPEED,
            });
    }

    let texture_handle = asset_server.load("ship.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 24.), 5, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 2, last: 7 };

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(2),
            transform: Transform {
                translation: Vec3::new(0., -80. * SCALE, 1.),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Ship,
    ));
}
