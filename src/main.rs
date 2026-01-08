use bevy::{prelude::*, window::PresentMode};
use bevy_crt::CathodePlugin;
use radioshack_os::{
    input::navigation, plugins::volume::VolumePlugin, rotate, setup, update_settings,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "RadioshackOS".into(),
                    resolution: (1000, 800).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            CathodePlugin,
            VolumePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, update_settings, navigation))
        .run();
}
