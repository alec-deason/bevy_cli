#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_unit_bundle)]

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut commands: Commands) {
    commands.spawn(());
    //~^ ERROR: inserted a `Bundle` containing a unit `()` type
}
