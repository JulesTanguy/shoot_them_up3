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

fn move_background(mut background_query: Query<(&Background, &mut Transform)>, time: Res<Time>) {
    let background_speed = 450.;

    for (_, mut transform) in background_query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * background_speed;

        error!("{:?}", transform);

        // When the background sprite goes out of view, move it back to the other side
        if transform.translation.y <= -HEIGHT {
            transform.translation.y += 2. * HEIGHT;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_size = Vec2::new(WIDTH, HEIGHT);
    let background_texture = asset_server.load("desert-backgorund-looped.png");
    // Spawn two background sprites
    for i in 0..2 {
        commands
            .spawn(SpriteBundle {
                texture: background_texture.clone(),
                transform: Transform::from_xyz(0., i as f32 * sprite_size.y, 0.),
                ..Default::default()
            })
            .insert(Background);
    }

    let texture_handle = asset_server.load("ship.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 24.), 5, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(2),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(AnimationState {
            start_index: 2,
            end_index: 9,
            current_index: 0,
            direction: 0,
        });
}
