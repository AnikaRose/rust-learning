use bevy:: prelude::*;

fn main(){
    App:: new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_player)
    .run();
}

fn spawn_player(mut commands : Commands){
    commands.spawn_empty();
    println!("Player spawned!");
}
