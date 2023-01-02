use bevy::prelude::*;
use bevy_snake::MainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Hello FOSDEM - Snake".to_string(),
                width: 500.,
                height: 500.,
                ..WindowDescriptor::default()
            },
            ..WindowPlugin::default()
        }))
        .add_plugin(MainPlugin)
        .run();
}
