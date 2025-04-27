mod plugins;
use bevy::prelude::*;
use plugins::camera::CameraPlugin;
use plugins::map::MapPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_word_system() {
    println!("Hello world!!!")
}

fn spawn_people(mut commands: Commands) {
    commands.spawn((Person, Name("Paul Noble".to_string())));
    commands.spawn((Person, Name("Paul Noble".to_string())));
    commands.spawn((Person, Name("Baba booy".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    println!("Greetings");
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .run();
}
