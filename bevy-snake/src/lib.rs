use std::hash::Hash;

use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, Plugin};
use choose_game_plugin::ChooseGamePlugin;

use game_over_plugin::GameOverPlugin;
use play_plugin::SnakePlugin;
use resources::Assets;
use snake::{SNAKE_1, SNAKE_2};

mod choose_game_plugin;
mod draw_utils;
mod events;
mod game_over_plugin;
mod play_plugin;
mod resources;

#[derive(Debug, Clone)]
enum AppState {
    ChooseGame,
    Play,
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

#[cfg(test)]
mod tests {
    use bevy::{
        app::AppExit,
        ecs::query::{ReadOnlyWorldQuery, WorldQuery},
        prelude::{
            default, App, Button, Entity, Events, GlobalTransform, ImagePlugin, Input, MouseButton,
            QueryState, UiCameraConfig,
        },
        text::Text,
        time::TimePlugin,
        ui::Interaction,
        window::{Window, WindowDescriptor, WindowId, Windows},
    };
    use tracing_log::LogTracer;

    use crate::{
        choose_game_plugin::components::*,
        events::GameTick,
        game_over_plugin::components::QuitComponent,
        play_plugin::components::ScoreComponent,
        resources::{ScoreResource, SnakeResource},
        MainPlugin,
    };

    #[test]
    fn test_run() {
        let mut app = create_app();

        run_till(
            &mut app,
            |app| {
                let items = get_entities_with::<
                    (Entity, &ChooseGameButtonComponent),
                    &ChooseGameButtonComponent,
                >(app);
                items.is_empty()
            },
            false,
        );

        let buttons = get_entities_with::<(Entity, &ChooseGameButtonComponent), &Button>(&mut app);
        let (snake1_entity, _) = buttons.into_iter().find(|c| c.1.name == "snake1").unwrap();
        click_on(&mut app, snake1_entity);

        run_till(
            &mut app,
            |app| {
                let has_score = app.world.get_resource::<ScoreResource>().is_some();
                let has_snake = app.world.get_resource::<SnakeResource>().is_some();

                let scores = get_entities_with::<&Text, &ScoreComponent>(app);

                !(has_score && has_snake && !scores.is_empty())
            },
            false,
        );

        let scores = get_entities_with::<&Text, &ScoreComponent>(&mut app);
        let score = &scores[0].sections[0];
        assert_eq!(score.value, "score: 0");

        run_till(
            &mut app,
            |app| {
                let quit = get_entities_with::<&QuitComponent, &QuitComponent>(app);
                quit.is_empty()
            },
            true,
        );

        let buttons = get_entities_with::<(Entity, &QuitComponent), &Button>(&mut app);
        let (quit, _) = buttons.into_iter().next().unwrap();
        click_on(&mut app, quit);

        run_till(
            &mut app,
            |app| {
                let quit = app.world.resource::<Events<AppExit>>();
                quit.is_empty()
            },
            false,
        );
    }

    pub fn click_on(app: &mut App, button_entity: Entity) {
        let buttons = get_entities_with::<(Entity, &GlobalTransform), &Button>(app);
        let g = buttons
            .into_iter()
            .find(|c| c.0 == button_entity)
            .unwrap()
            .1;

        let click_position = g.translation();
        let click_position = click_position.as_dvec3().truncate();

        let mut buttons = get_entities::<&mut Interaction>(app);
        let mut interaction = buttons.get_mut(&mut app.world, button_entity).unwrap();

        *interaction = Interaction::Clicked;

        let mut input = app.world.resource_mut::<Input<MouseButton>>();
        input.press(MouseButton::Left);

        {
            let mut windows = app.world.resource_mut::<Windows>();
            let window = windows.primary_mut();
            window.update_cursor_physical_position_from_backend(Some(click_position));
            window.set_cursor_position(click_position.as_vec2());
            window.update_focused_status_from_backend(true);

            let mut input = app.world.resource_mut::<Input<MouseButton>>();
            input.press(MouseButton::Left);
        }

        run_till(app, |_| false, false);
    }

    pub fn create_app() -> App {
        use bevy::{
            asset::AssetPlugin, core::CorePlugin, core_pipeline::CorePipelinePlugin,
            hierarchy::HierarchyPlugin, input::InputPlugin, pbr::PbrPlugin, render::RenderPlugin,
            scene::ScenePlugin, sprite::SpritePlugin, text::TextPlugin, transform::TransformPlugin,
            ui::UiPlugin, utils::tracing::subscriber::set_global_default, window::WindowPlugin,
        };
        use tracing_subscriber::{prelude::*, registry::Registry, EnvFilter};

        LogTracer::init().unwrap();
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("OFF,brando=INFO"))
            .unwrap();
        let subscriber = Registry::default().with(filter_layer);
        let fmt_layer = tracing_subscriber::fmt::Layer::default();
        let subscriber = subscriber.with(fmt_layer);
        set_global_default(subscriber).unwrap();

        let mut app = App::new();

        app.world.clear_entities();
        app.world.clear_trackers();

        app.add_plugin(CorePlugin::default())
            .add_plugin(TimePlugin::default())
            .add_plugin(TransformPlugin::default())
            .add_plugin(HierarchyPlugin::default())
            .add_plugin(InputPlugin::default())
            .add_plugin(WindowPlugin {
                add_primary_window: true,
                ..default() // exit_on_close: false,
            })
            .add_plugin(AssetPlugin::default())
            .add_plugin(ScenePlugin::default())
            .add_plugin(RenderPlugin::default())
            .add_plugin(CorePipelinePlugin::default())
            .add_plugin(TextPlugin::default())
            .add_plugin(UiPlugin::default())
            .add_plugin(ImagePlugin::default())
            .add_plugin(PbrPlugin::default())
            .add_plugin(SpritePlugin::default());

        let mut windows = app.world.resource_mut::<Windows>();
        windows.add(Window::new(
            WindowId::primary(),
            &WindowDescriptor::default(),
            200,
            200,
            1.,
            None,
            None,
        ));

        app.add_plugin(MainPlugin);

        run_till(&mut app, |_| false, false);

        let mut q = get_entities::<(Entity, &mut UiCameraConfig)>(&mut app);
        for (_, mut c) in q.iter_mut(&mut app.world) {
            c.show_ui = true;
        }

        app
    }

    pub fn get_entities_with<Comp: WorldQuery, WithComp: ReadOnlyWorldQuery>(
        app: &mut App,
    ) -> Vec<<<Comp as WorldQuery>::ReadOnly as WorldQuery>::Item<'_>> {
        let world = &mut app.world;
        let mut query = world.query_filtered::<Comp, WithComp>();
        let items = query.iter(world).collect::<Vec<_>>();
        items
    }

    pub fn get_entities<Comp: WorldQuery>(app: &mut App) -> QueryState<Comp> {
        let world = &mut app.world;
        world.query::<Comp>()
    }

    pub fn run_till<F>(app: &mut App, mut f: F, send_game_tick: bool)
    where
        F: FnMut(&mut App) -> bool,
    {
        if !f(app) {
            return;
        }

        let mut i = 1;
        loop {
            if send_game_tick {
                let world = &mut app.world;
                let mut game_tick = world.get_resource_mut::<Events<GameTick>>().unwrap();

                game_tick.send(GameTick);
            }

            app.update();

            if !f(app) {
                break;
            }

            i += 1;
            assert!(i < 100);
        }
    }
}
