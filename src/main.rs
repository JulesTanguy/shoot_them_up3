use bevy::prelude::*;

#[derive(Component)]
struct AnimationState {
    start_index: usize,
    end_index: usize,
    current_index: usize,
    direction: isize,
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct LinearEasing {
    time: f32,
    duration: f32,
}

const WIDTH: f32 = 768.;
const HEIGHT: f32 = 576.;

fn main() {
    App::new() // prevents blurry sprites
        .add_startup_system(setup)
        .add_system(animate_sprite_on_keypress)
        .add_system(move_background)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Shoot Them Up 3".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        position: WindowPosition::Centered(MonitorSelection::Current),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .run();
}

fn animate_sprite_on_keypress(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationState)>,
) {
    let mut should_update = false;
    let mut direction = 0;

    // Replace KEY_SPACE with your desired key
    if keyboard_input.just_pressed(KeyCode::Left) {
        should_update = true;
        direction = -1;
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        should_update = true;
        direction = 1;
    }

    if !should_update {
        return;
    }
    for (mut sprite, mut animation_state) in query.iter_mut() {
        animation_state.current_index = (animation_state.current_index + 1)
            % (animation_state.end_index - animation_state.start_index + 1);
        sprite.index = (animation_state.start_index + animation_state.current_index) as usize;
        animation_state.direction = direction;
    }
}

// fn move_background(mut background_query: Query<(&Background, &mut Transform)>, time: Res<Time>) {
//     let background_speed = 150.0;
//     let loop_at = -HEIGHT;
//     for (_, mut transform) in background_query.iter_mut() {
//         transform.translation.y -= time.delta_seconds() * background_speed;
//         if transform.translation.y <= loop_at {
//             transform.translation.y = 0.0;
//         }
//     }
// }

fn move_background(
    mut background_query: Query<(&Background, &mut Transform, &mut LinearEasing)>,
    time: Res<Time>
) {
    let base_speed = 300.0;
    let loop_at = -HEIGHT;

    for (_, mut transform, mut easing) in background_query.iter_mut() {
        easing.time = easing.time.min(easing.duration);
        let t = easing.time / easing.duration;
        let eased_speed = base_speed * t;

        transform.translation.y -= time.delta_seconds() * eased_speed;
        if transform.translation.y <= loop_at {
            transform.translation.y = 0.0;
            easing.time = 0.0; // Reset easing timer
        } else {
            easing.time += time.delta_seconds();
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let background_texture = asset_server.load("desert-backgorund-looped.png");
    commands
        .spawn(SpriteBundle {
            texture: background_texture,
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, -1.0)),
            ..Default::default()
        })
        .insert(Background).insert(LinearEasing {
            time: 0.0,  // Initialize the easing timer to 0 seconds
            duration: 1.0,  // Set the easing duration, e.g., 1 second would mean smooth movement over a period of 1 second
        });

    let texture_handle = asset_server.load("ship.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 24.0), 5, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(2),
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 30.0)),
            ..default()
        })
        .insert(AnimationState {
            start_index: 2,
            end_index: 9,
            current_index: 0,
            direction: 0,
        });
}