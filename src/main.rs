use bevy::prelude::*;

pub enum VehicleType {
    Roadster,
}

impl std::fmt::Display for VehicleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VehicleType::Roadster => "Roadster",
            }
        )
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Vehicle(VehicleType);

pub struct EonsPlugin;

impl Plugin for EonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player)
            .add_system(print_player_data);
    }
}

fn add_player(mut commands: Commands) {
    commands.spawn((Player, Vehicle(VehicleType::Roadster)));
}

#[derive(Resource)]
struct PrintTimer(Timer);

fn print_player_data(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<&Vehicle, With<Player>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for vehicle in query.iter() {
            println!("Using Vehicle: {}", vehicle.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EonsPlugin)
        .run();
}
