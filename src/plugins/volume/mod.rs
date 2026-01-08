use bevy::{input::keyboard::Key, prelude::*};
use smol_str::SmolStr;

pub struct VolumePlugin;

impl Plugin for VolumePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Volume>()
            .init_resource::<VolumeDisplayTimer>()
            .add_systems(Startup, setup_volume_bar)
            .add_systems(
                Update,
                (
                    volume_input,
                    update_volume_bar.run_if(resource_changed::<Volume>),
                    hide_volume_bar,
                ),
            );
    }
}

#[derive(Resource, Deref, Debug, Clone, Copy)]
pub struct Volume(u8);

impl Default for Volume {
    fn default() -> Self {
        Self(10)
    }
}

impl Volume {
    pub fn up(&mut self) {
        if self.0 < 20 {
            self.0 += 1;
        }
    }

    pub fn down(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }
}

const VOLUME_UP: Key = Key::Character(SmolStr::new_inline("b"));
const VOLUME_DOWN: Key = Key::Character(SmolStr::new_inline("a"));

pub fn volume_input(key_input: Res<ButtonInput<Key>>, mut volume: ResMut<Volume>) {
    if key_input.just_pressed(VOLUME_UP) {
        volume.up();
    }
    if key_input.just_pressed(VOLUME_DOWN) {
        volume.down();
    }
}

const VOLUME_INCREMENT_WIDTH: f32 = 20.0;
const VOLUME_INCREMENT_MARGIN: f32 = 1.0;
const VOLUME_INCREMENT_HEIGHT: f32 = 100.0;
const VOLUME_MAX: u8 = 20;

pub fn update_volume_bar(
    mut commands: Commands,
    volume: Res<Volume>,
    mut timer: ResMut<VolumeDisplayTimer>,
    mut visibility: Query<&mut Visibility, With<VolumeCanvas>>,
    container: Query<Entity, With<VolumeBar>>,
    volume_increment_query: Query<Entity, With<VolumeIncrement>>,
) {
    // Skip first frame initialization
    if !timer.initialized {
        timer.initialized = true;
        return;
    }

    timer.timer.reset();
    timer.is_visible = true;

    // Remove all existing rectangles
    for entity in &volume_increment_query {
        commands.entity(entity).despawn();
    }

    let Ok(mut visibility) = visibility.single_mut() else {
        return;
    };
    *visibility = Visibility::Visible;

    let Ok(container) = container.single() else {
        return;
    };
    commands.entity(container).with_children(|parent| {
        for _ in 0..**volume {
            parent.spawn((
                Node {
                    width: Val::Px(VOLUME_INCREMENT_WIDTH),
                    height: Val::Px(VOLUME_INCREMENT_HEIGHT),
                    margin: UiRect::all(Val::Px(VOLUME_INCREMENT_MARGIN)),
                    display: Display::Block,
                    ..default()
                },
                BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                VolumeIncrement,
            ));
        }
    });
}

pub fn hide_volume_bar(
    time: Res<Time>,
    mut timer: ResMut<VolumeDisplayTimer>,
    mut query: Query<&mut Visibility, With<VolumeCanvas>>,
) {
    if timer.is_visible {
        timer.timer.tick(time.delta());

        if timer.timer.is_finished() {
            timer.is_visible = false;
            for mut visibility in &mut query {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

#[derive(Component)]
pub struct VolumeCanvas;

#[derive(Component)]
pub struct VolumeBar;

#[derive(Component)]
pub struct VolumeIncrement;

#[derive(Resource)]
pub struct VolumeDisplayTimer {
    timer: Timer,
    is_visible: bool,
    initialized: bool,
}

impl Default for VolumeDisplayTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
            is_visible: false,
            initialized: false,
        }
    }
}

pub fn setup_volume_bar(mut commands: Commands) {
    // Outer container for centering
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Visibility::Hidden,
            VolumeCanvas,
        ))
        .with_children(|parent| {
            // volume bar
            parent.spawn((
                Node {
                    width: Val::Px(
                        VOLUME_MAX as f32
                            * (VOLUME_INCREMENT_WIDTH + 2.0 * VOLUME_INCREMENT_MARGIN),
                    ),
                    height: Val::Px(VOLUME_INCREMENT_HEIGHT + 2.0 * VOLUME_INCREMENT_MARGIN),
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    box_sizing: BoxSizing::ContentBox,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.0, 0.0, 1.0).into()),
                VolumeBar,
            ));
        });
}
