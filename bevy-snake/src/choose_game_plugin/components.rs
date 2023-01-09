use bevy::prelude::Component;

#[derive(Component)]
pub struct ChooseGameComponent;

#[derive(Component)]
pub struct ChooseGameButtonComponent {
    pub name: &'static str,
    pub board: &'static str,
}
