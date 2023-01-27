use bevy::{prelude::*};
use plugins::{base::BasePlugin, display::DisplayPlugin};

mod plugins;
mod entities;

fn main() {
    App::new()
    // Resources
    .insert_resource(ClearColor(Color::Rgba { red: 0.2, green: 0.2, blue: 0.3, alpha: 1. }))

    // Systems
    

    // Plugins
    .add_plugin(BasePlugin)
    .add_plugin(DisplayPlugin)
    .run();
}