use bevy::prelude::Component;

#[derive(Component)]
pub struct ChooseGameComponent;

#[derive(Component)]
pub struct ChooseGameButtonComponent(pub &'static str);

#[derive(Component)]
pub struct WallComponent;

#[derive(Component)]
pub struct SnakeBodyComponent;

#[derive(Component)]
pub struct SnakeHeadComponent;

#[derive(Component)]
pub struct FoodComponent;
