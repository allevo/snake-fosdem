use bevy::{
    prelude::{AssetServer, FromWorld, Resource, Vec2},
    sprite::{Sprite, SpriteBundle},
    time::Timer,
};
use snake::{Direction, Game, Point};

use crate::{draw_utils::DrawConfigurationResource};


#[derive(Resource)]
pub struct GameNameChosen(pub Option<&'static str>);

#[derive(Resource)]
pub struct GameResource(pub Game);

#[derive(Resource)]
pub struct WallsResource(pub Option<Vec<Point>>);

#[derive(Resource)]
pub struct ScoreResource(pub usize);

#[derive(Resource)]
pub struct SnakeResource(pub Vec<Point>);

#[derive(Resource)]
pub struct FoodPositionResource(pub Point);

#[derive(Resource)]
pub struct GameTimerResource(pub Timer);

#[derive(Resource)]
pub struct CurrentDirection(pub Direction);

#[derive(Clone, Copy)]
pub enum BundleType {
    Wall,
    SnakeBody,
    SnakeHead,
    Food,
}

#[derive(Resource)]
pub struct PbrBundles {
    pub wall: SpriteBundle,
    pub snake_body: SpriteBundle,
    pub snake_head: SpriteBundle,
    pub food: SpriteBundle,
}

impl FromWorld for PbrBundles {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let cell_size = world.resource::<DrawConfigurationResource>().cell_size;

        let mut asset_server = world.resource_mut::<AssetServer>();

        let wall = load_sprite(&mut asset_server, "wall.png", cell_size);
        let snake_head = load_sprite(&mut asset_server, "snake_head.png", cell_size);
        let snake_body = load_sprite(&mut asset_server, "snake_body.png", cell_size);
        let food = load_sprite(&mut asset_server, "food.png", cell_size);

        Self {
            wall,
            snake_body,
            snake_head,
            food,
        }
    }
}

fn load_sprite(asset_server: &mut AssetServer, s: &'static str, cell_size: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(cell_size, cell_size)),
            ..Sprite::default()
        },
        texture: asset_server.load(s),
        ..SpriteBundle::default()
    }
}
