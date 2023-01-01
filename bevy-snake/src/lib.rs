use bevy::{
    prelude::{
        Camera2dBundle,
        ClearColor, Color, Commands, Plugin,
    },
};
use choose_game::ChooseGamePlugin;


use resources::*;
use snake::{SNAKE_1, SNAKE_2};
use snake_plugin::SnakePlugin;

mod choose_game;
mod components;
mod events;
mod resources;
mod snake_plugin;
mod draw_utils;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    ChooseGame,
    InGame,
    Dead,
}

static LEVELS: [(&str, &str); 2] = [("snake1", SNAKE_1), ("snake2", SNAKE_2)];

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(add_camera)
            // Background
            .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .insert_resource(GameNameChosen(None))
            .add_plugin(ChooseGamePlugin)
            .add_plugin(SnakePlugin)
            .add_state(AppState::ChooseGame);
    }
}

fn add_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}
