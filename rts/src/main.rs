use bevy::prelude::*;
use gmap;

fn main() {
    let num = 3;
    println!("Hello, world! {num} plus 5 is {}!", gmap::add(num, 5));


    App::new()
        .add_plugin(gmap::GMapPlugin)
        .run();
}
