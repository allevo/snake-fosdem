use bevy::prelude::{Resource, Commands, Transform, Mut, Vec3};
use snake::Point;

use crate::{resources::{PbrBundles, BundleType}, components::*};

const WALL_Z: f32 = 0.;
const SNAKE_Z: f32 = 1.;
const FOOD_Z: f32 = 1.;

#[derive(Resource)]
pub struct DrawConfigurationResource {
    pub cell_size: f32,
    pub dim: (usize, usize),
}

impl DrawConfigurationResource {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        bundles: &PbrBundles,
        bundle_type: BundleType,
        position: &Point,
    ) {
        let mut bundle = match bundle_type {
            BundleType::Wall => bundles.wall.clone(),
            BundleType::SnakeBody => bundles.snake_body.clone(),
            BundleType::SnakeHead => bundles.snake_head.clone(),
            BundleType::Food => bundles.food.clone(),
        };
        bundle.transform.translation = self.get_translation(position, Self::get_z(bundle_type));

        let mut entity_commands = commands.spawn(bundle);

        match bundle_type {
            BundleType::Wall => entity_commands.insert(WallComponent),
            BundleType::SnakeBody => entity_commands.insert(SnakeBodyComponent),
            BundleType::SnakeHead => entity_commands.insert(SnakeHeadComponent),
            BundleType::Food => entity_commands.insert(FoodComponent),
        };
    }

    pub fn translate(
        &self,
        bundle_type: BundleType,
        mut transform: Mut<Transform>,
        position: &Point,
    ) {
        transform.translation = self.get_translation(position, Self::get_z(bundle_type));
    }

    pub fn get_z(bundle_type: BundleType) -> f32 {
        match bundle_type {
            BundleType::Wall => WALL_Z,
            BundleType::SnakeBody => SNAKE_Z,
            BundleType::SnakeHead => SNAKE_Z,
            BundleType::Food => FOOD_Z,
        }
    }

    pub fn get_translation(&self, point: &Point, z: f32) -> Vec3 {
        let x = point.x as f32 * self.cell_size + self.cell_size / 2.
            - (self.cell_size * self.dim.0 as f32) / 2.;
        let y = point.y as f32 * self.cell_size + self.cell_size / 2.
            - (self.cell_size * self.dim.1 as f32) / 2.;

        Vec3::new(x, y, z)
    }
}
