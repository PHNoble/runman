mod plugins;
mod components;

use bevy::prelude::*;
use plugins::camera::CameraPlugin;
use plugins::map::MapPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .run();
}
