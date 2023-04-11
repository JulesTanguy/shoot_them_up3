use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;

const SCROLL_SPEED: f32 = -1. * SCALE;
const SHIP_SPEED: f32 = 75. * SCALE;
const TIME_STEP: f32 = 1.0 / 60.0;

// Bounds
const RIGHT_BOUND: f32 = WIDTH / 2. - (16. * SCALE / 2.);
const LEFT_BOUND: f32 = -(WIDTH / 2. - (16. * SCALE / 2.));
const SCALE: f32 = 3.;
const WIDTH: f32 = 256. * SCALE;
const HEIGHT: f32 = 256. * SCALE;
const BG_HEIGHT: f32 = 608.;

struct EnemiesTexturePathAndSize {
    size: Vec2,
    path: &'static str,
    label: EnemyLabel,
}

const ENEMIES_TEXTURES: [EnemiesTexturePathAndSize; 3] = [
    EnemiesTexturePathAndSize {
        size: Vec2 { x: 16., y: 16. },
        path: "enemy-small.png",
        label: EnemyLabel::Small,
    },
    EnemiesTexturePathAndSize {
        size: Vec2 { x: 32., y: 16. },
        path: "enemy-medium.png",
        label: EnemyLabel::Medium,
    },
    EnemiesTexturePathAndSize {
        size: Vec2 { x: 32., y: 32. },
        path: "enemy-big.png",
        label: EnemyLabel::Big,
    },
];

fn main() {
    App::new() // prevents blurry sprites
        .add_startup_system(setup)
        .add_systems((
            scroll_background,
            animate_sprite,
            move_ship,
            random_enemy_spawn,
        ))
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

#[derive(Component, Deref, DerefMut)]
struct EnemiesTimer(Timer);

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Enemy(EnemyLabel);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum EnemyLabel {
    Big,
    Medium,
    Small,
}

struct EnemySpriteMaterial {
    handle: Handle<TextureAtlas>,
    enemy_label: EnemyLabel,
}

#[derive(Resource, Deref)]
struct EnemiesSpriteMaterial([EnemySpriteMaterial; 3]);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    /*
     *  Background
     */
    let background_texture = asset_server.load("desert-backgorund-looped.png");
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

    /*
     *  Ship
     */
    let texture_handle = asset_server.load("ship.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 24.), 5, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 2, last: 7 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(2),
            transform: Transform {
                translation: Vec3::new(0., -110. * SCALE, 1.),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Ship,
    ));

    /*
     *  Enemies
     */
    let mut enemies: Vec<EnemySpriteMaterial> = Vec::with_capacity(3);
    for enemies_texture_path_and_size in ENEMIES_TEXTURES {
        let texture_handle = asset_server.load(enemies_texture_path_and_size.path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            enemies_texture_path_and_size.size,
            2,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        enemies.push(EnemySpriteMaterial {
            handle: texture_atlas_handle,
            enemy_label: enemies_texture_path_and_size.label,
        })
    }
    commands.insert_resource(EnemiesSpriteMaterial(enemies.try_into().unwrap_or_else(
        |v: Vec<EnemySpriteMaterial>| {
            panic!("Expected a Vec of length {} but it was {}", 3, v.len())
        },
    )));
    commands.spawn(EnemiesTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
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

fn random_enemy_spawn(
    time: Res<Time>,
    texture_atlas_handle: Res<EnemiesSpriteMaterial>,
    mut commands: Commands,
    mut query: Query<&mut EnemiesTimer>,
) {
    for mut timer in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0..3);
            let selected_enemy = &texture_atlas_handle[random_number];
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: selected_enemy.handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform {
                        translation: Vec3::new(
                            rng.gen_range(LEFT_BOUND..RIGHT_BOUND),
                            -110. * SCALE,
                            2.,
                        ),
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..default()
                },
                Enemy(selected_enemy.enemy_label),
            ));
        }
    }
}
