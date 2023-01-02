use std::hash::Hash;

use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, Plugin, App};
use choose_game::ChooseGamePlugin;

use game_over::GameOverPlugin;
use resources::Assets;
use snake::{SNAKE_1, SNAKE_2};
use snake_plugin::SnakePlugin;

mod choose_game;
mod components;
mod draw_utils;
mod events;
mod game_over;
mod resources;
mod snake_plugin;

#[derive(Debug, Clone)]
enum AppState {
    ChooseGame,
    InGame(&'static str),
    GameOver(&'static str),
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
impl Eq for AppState {}
impl Hash for AppState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

static LEVELS: [(&str, &str); 2] = [("snake1", SNAKE_1), ("snake2", SNAKE_2)];

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(add_camera)
            // Background
            .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            // Images used for drawing stuff
            .init_resource::<Assets>()
            // Other plugins...
            .add_plugin(ChooseGamePlugin)
            .add_plugin(SnakePlugin)
            .add_plugin(GameOverPlugin)
            .add_state(AppState::ChooseGame);
    }
}

fn add_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}
