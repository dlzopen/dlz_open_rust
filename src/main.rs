use bevy::prelude::*;
use rd_map::MapPlugin;



fn main() {
    println!("你好，轰隆隆……");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .run();
}
