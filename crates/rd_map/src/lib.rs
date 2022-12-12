use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("大刀".to_string()));
    commands.spawn().insert(Person).insert(Name("朱老忠".to_string()));
    commands.spawn().insert(Person).insert(Name("赵虎".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("{}，报到！", name.0);
    }
}

fn hello_world() {
    println!("hello world!");
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::MapPlugin;
    #[test]
    fn test_map_plugin() {
    }
}
