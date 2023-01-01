use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{
        Commands, Entity, EventReader, EventWriter, KeyCode, Plugin, Query, Res, ResMut, SystemSet,
        Transform, With, World,
    },
    time::{Time, Timer, TimerMode},
    window::{Window, Windows},
};
use snake::{Direction, Game};

use crate::{components::*, events::GameTick, resources::*, AppState, LEVELS, draw_utils::DrawConfigurationResource};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<GameTick>()
            // Camera
            // Enter in game
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(add_all_resources)
                    // init draw
                    .with_system(init_draw),
            )
            // Run game
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    // draw walls
                    .with_system(draw_walls)
                    // Run tick
                    .with_system(tick)
                    // Play
                    .with_system(play)
                    // Update resources
                    .with_system(update_snake_body)
                    .with_system(update_snake_head)
                    .with_system(update_food)
                    .with_system(handle_keyboard_input),
            );
    }
}

fn add_all_resources(world: &mut World) {
    let game_chosen: &GameNameChosen = world.resource();
    let game_name_chosen = game_chosen.0.unwrap();

    let level = LEVELS
        .iter()
        .find(|(name, _)| name == &game_name_chosen)
        .unwrap()
        .1;

    let game: Game = level.parse().unwrap();
    let dim = game.dim();
    let cell_size = calculate_cell_size(dim, world.resource::<Windows>().primary());

    let snapshot = game.last_snapshot();
    let duration = snapshot.period_duration;

    let walls = game.walls();

    // Keep track cell size
    world.insert_resource(DrawConfigurationResource { cell_size, dim });
    world
        // Images used for drawing stuff
        .init_resource::<PbrBundles>();
    world
        // Main resource: pointer to the game
        .insert_resource(GameResource(game));
    world
        // Shadow resources
        .insert_resource(WallsResource(Some(walls)));
    world.insert_resource(ScoreResource(snapshot.score));
    world.insert_resource(SnakeResource(snapshot.snake));
    world.insert_resource(FoodPositionResource(snapshot.food_position));
    world.insert_resource(CurrentDirection(Direction::Up));
    world
        // Timer
        .insert_resource(GameTimerResource(Timer::new(
            duration,
            TimerMode::Repeating,
        )));
}

fn init_draw(
    mut commands: Commands,
    snake: Res<SnakeResource>,
    food_position: Res<FoodPositionResource>,
    bundles: Res<PbrBundles>,
    drawing_configuration: Res<DrawConfigurationResource>,
) {
    let mut snake_iter = snake.0.iter();

    let head = snake_iter.next().unwrap();

    drawing_configuration.spawn(&mut commands, &bundles, BundleType::SnakeHead, head);

    for new_snake_point in snake_iter {
        drawing_configuration.spawn(
            &mut commands,
            &bundles,
            BundleType::SnakeBody,
            new_snake_point,
        );
    }

    drawing_configuration.spawn(&mut commands, &bundles, BundleType::Food, &food_position.0);
}

fn draw_walls(
    mut commands: Commands,
    walls: Res<WallsResource>,
    walls_query: Query<(Entity, &WallComponent)>,
    bundles: Res<PbrBundles>,
    drawing_configuration: Res<DrawConfigurationResource>,
) {
    if !walls.is_changed() {
        return;
    }

    // Remove all old walls
    for (entity, _) in walls_query.iter() {
        commands.entity(entity).despawn();
    }

    if let Some(walls) = &walls.0 {
        for wall_position in walls {
            drawing_configuration.spawn(&mut commands, &bundles, BundleType::Wall, wall_position);
        }
    };
}

fn tick(
    time: Res<Time>,
    mut game_timers: ResMut<GameTimerResource>,
    mut tick_event_writer: EventWriter<GameTick>,
) {
    let game_timers = &mut game_timers.0;
    if !game_timers.tick(time.delta()).finished() {
        return;
    }

    tick_event_writer.send(GameTick);
}

fn play(
    mut tick_event: EventReader<GameTick>,
    mut game: ResMut<GameResource>,
    mut score: ResMut<ScoreResource>,
    mut snake: ResMut<SnakeResource>,
    mut food_position: ResMut<FoodPositionResource>,
    current_direction: Res<CurrentDirection>,
    mut game_timers: ResMut<GameTimerResource>,
) {
    if tick_event.iter().count() == 0 {
        return;
    }

    game.0.tick(current_direction.0);

    let snapshot = game.0.last_snapshot();

    // Update resources
    score.0 = snapshot.score;
    snake.0 = snapshot.snake;
    if snapshot.food_position != food_position.0 {
        food_position.0 = snapshot.food_position;
    }
    if game_timers.0.duration() != snapshot.period_duration {
        game_timers.0.set_duration(snapshot.period_duration);
    }
}

fn update_food(
    food_position: Res<FoodPositionResource>,
    mut food_query: Query<&mut Transform, With<FoodComponent>>,
    drawing_configuration: Res<DrawConfigurationResource>,
) {
    if !food_position.is_changed() {
        return;
    }

    // We are sure than head always exists
    let transform = food_query.single_mut();
    let position = food_position.0;

    drawing_configuration.translate(BundleType::Food, transform, &position);
}

fn update_snake_head(
    snake: Res<SnakeResource>,
    mut snake_query: Query<&mut Transform, With<SnakeHeadComponent>>,
    drawing_configuration: Res<DrawConfigurationResource>,
) {
    if !snake.is_changed() {
        return;
    }

    // We are sure than head always exists
    let transform = snake_query.single_mut();
    let position = snake.0.get(0).unwrap();

    drawing_configuration.translate(BundleType::SnakeHead, transform, position);
}

fn update_snake_body(
    mut commands: Commands,
    snake: Res<SnakeResource>,
    mut snake_query: Query<(Entity, &mut Transform), With<SnakeBodyComponent>>,
    bundles: Res<PbrBundles>,
    drawing_configuration: Res<DrawConfigurationResource>,
) {
    if !snake.is_changed() {
        return;
    }

    let iter = snake_query.iter_mut();
    // Skip 1 because the first one is the head
    let mut snake_iter = snake.0.iter().skip(1);

    for (entity, transform) in iter {
        match snake_iter.next() {
            None => {
                commands.entity(entity).despawn();
            }
            Some(snake_point) => {
                drawing_configuration.translate(BundleType::SnakeBody, transform, snake_point);
            }
        };
    }

    for new_snake_point in snake_iter {
        drawing_configuration.spawn(
            &mut commands,
            &bundles,
            BundleType::SnakeBody,
            new_snake_point,
        );
    }
}

fn handle_keyboard_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut current_direction: ResMut<CurrentDirection>,
) {
    let direction = keyboard_input_events
        .iter()
        .filter_map(|ki| ki.key_code)
        .filter_map(|kc| match kc {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        })
        .last();

    if let Some(direction) = direction {
        current_direction.0 = direction;
    }
}

fn calculate_cell_size(dim: (usize, usize), window: &Window) -> f32 {
    let height = window.height();
    let width = window.width();
    (height / dim.1 as f32).min(width / dim.0 as f32)
}
