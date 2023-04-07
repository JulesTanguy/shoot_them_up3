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
const WIDTH: f32 = 768.;
const HEIGHT: f32 = 576.;
const BG_HEIGHT: f32 = 608.;
const SCROLL_SPEED: f32 = -3.;

fn main() {
    App::new() // prevents blurry sprites
        .add_startup_system(setup)
        .add_systems((scroll_background, animate_sprite))
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

fn scroll_background(mut query: Query<(&mut Transform, &Background)>) {
    for (mut transform, background) in query.iter_mut() {
        transform.translation.y += background.speed;

        if transform.translation.y <= -BG_HEIGHT * 3. {
            // If the background is fully out of view, reset its position.
            transform.translation.y -= 2.0 * BG_HEIGHT * 3.;
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
    for i in 0..10 {
        commands
            .spawn(SpriteBundle {
                texture: background_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(0., BG_HEIGHT * 3. * i as f32 - BG_HEIGHT * 3., 0.),
                    scale: Vec3::new(3., 3., 3.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Background { speed: SCROLL_SPEED });
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
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(3., 3., 1.),
                ..Default::default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
