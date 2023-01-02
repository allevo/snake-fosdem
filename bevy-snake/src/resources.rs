use bevy::{
    prelude::{AssetServer, Color, FromWorld, Handle, Resource},
    sprite::{Sprite, SpriteBundle},
    text::Font,
    time::Timer,
};
use snake::{Direction, Game, Point};

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
pub struct Assets {
    pub wall: SpriteBundle,
    pub snake_body: SpriteBundle,
    pub snake_head: SpriteBundle,
    pub food: SpriteBundle,
    pub background_tile: SpriteBundle,
    pub font: Handle<Font>,
    pub normal_button_color: Color,
    pub hovered_button_color: Color,
    pub pressed_button_color: Color,
    pub text_button_color: Color,
    pub text_color: Color,
    pub overlay_background_color: Color,
}

impl FromWorld for Assets {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut asset_server = world.resource_mut::<AssetServer>();

        let wall = load_sprite(&mut asset_server, "wall.png");
        let snake_head = load_sprite(&mut asset_server, "snake_head.png");
        let snake_body = load_sprite(&mut asset_server, "snake_body.png");
        let food = load_sprite(&mut asset_server, "food.png");
        let background_tile = load_sprite(&mut asset_server, "background.png");

        let font = asset_server.load("RobotoMedium-Owv4.ttf");

        Self {
            wall,
            snake_body,
            snake_head,
            food,
            background_tile,
            font,
            normal_button_color: Color::rgb(0.15, 0.15, 0.15),
            hovered_button_color: Color::rgb(0.25, 0.25, 0.25),
            pressed_button_color: Color::rgb(0.35, 0.75, 0.35),
            text_button_color: Color::WHITE,
            text_color: Color::WHITE,
            overlay_background_color: Color::rgba(0.6, 0.6, 0.6, 0.6),
        }
    }
}

fn load_sprite(asset_server: &mut AssetServer, s: &'static str) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            // custom_size: Some(Vec2::new(cell_size, cell_size)),
            ..Sprite::default()
        },
        texture: asset_server.load(s),
        ..SpriteBundle::default()
    }
}
