
use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{
        Commands, Entity, EventReader, EventWriter, KeyCode, Plugin, Query, Res, ResMut, State,
        SystemSet, Transform, With, World, TextBundle,
    },
    time::{Time, Timer, TimerMode},
    window::{Window, Windows}, text::{TextStyle, Text},
};
use snake::{Direction, Game};

use crate::{
    components::*,
    draw_utils::DrawConfigurationResource,
    events::{GameChosen, GameTick, GameOver},
    resources::*,
    AppState, LEVELS,
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<GameTick>()
            .add_event::<GameOver>()
            .add_system(wait_for_game_chosen_event)
            // Enter in game
            .add_system_set(
                SystemSet::on_enter(AppState::InGame(""))
                    .with_system(add_all_resources)
                    // init draw
                    .with_system(init_draw),
            )
            // Run game
            .add_system_set(
                SystemSet::on_update(AppState::InGame(""))
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
                    .with_system(update_score)
                    .with_system(handle_keyboard_input),
            );
    }
}

fn wait_for_game_chosen_event(
    mut game_chosen_reader: EventReader<GameChosen>,
    mut app_state: ResMut<State<AppState>>,
) {
    if let Some(name) = game_chosen_reader.iter().last().map(|e| e.0) {
        app_state.set(AppState::InGame(name)).unwrap();
    };
}

fn add_all_resources(world: &mut World) {
    let state: &State<AppState> = world.resource();
    let state = state.current();
    let game_name = match state {
        AppState::InGame(name) => name,
        _ => unreachable!("`add_all_resources` should be called only when the state is InGame"),
    };

    let level = LEVELS
        .iter()
        .find(|(name, _)| name == game_name)
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
    assets: Res<Assets>,
    drawing_configuration: Res<DrawConfigurationResource>,
    score: Res<ScoreResource>,
) {
    let mut snake_iter = snake.0.iter();

    let head = snake_iter.next().unwrap();

    // Background
    drawing_configuration.spawn_background(&mut commands, &assets);

    // Head
    drawing_configuration.spawn(&mut commands, &assets, BundleType::SnakeHead, head);

    // Body
    for new_snake_point in snake_iter {
        drawing_configuration.spawn(
            &mut commands,
            &assets,
            BundleType::SnakeBody,
            new_snake_point,
        );
    }

    // Food
    drawing_configuration.spawn(&mut commands, &assets, BundleType::Food, &food_position.0);

    // Score
    commands
        .spawn(TextBundle::from_section(
            format!("score: {}", score.0),
            TextStyle {
                font: assets.font.clone(),
                font_size: 30.0,
                color: assets.text_button_color,
            },
        ))
        .insert(ScoreComponent);
}

fn draw_walls(
    mut commands: Commands,
    walls: Res<WallsResource>,
    walls_query: Query<(Entity, &WallComponent)>,
    assets: Res<Assets>,
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
            drawing_configuration.spawn(&mut commands, &assets, BundleType::Wall, wall_position);
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
    mut game_over_writer: EventWriter<GameOver>,
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

    // game over
    if let Some(reason) = snapshot.get_dead_reason() {
        game_over_writer.send(GameOver(reason));
        game_timers.0.pause();
        return;
    }

    // Update resources
    snake.0 = snapshot.snake;
    if score.0 != snapshot.score {
        score.0 = snapshot.score;
    }
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

fn update_score(
    score: Res<ScoreResource>,
    mut score_query: Query<&mut Text, With<ScoreComponent>>,
) {
    if !score.is_changed() {
        return;
    }

    let mut text = score_query.single_mut();
    text.sections[0].value = format!("score: {}", score.0);
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
    assets: Res<Assets>,
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
            &assets,
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
