mod camera_plugin;
use bevy::prelude::*;

use camera_plugin::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .run();
}
