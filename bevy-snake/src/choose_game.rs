use bevy::{
    prelude::{
        default, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Color, Commands,
        DespawnRecursiveExt, Entity, NodeBundle, Plugin, Query, Res, ResMut, State, SystemSet,
        TextBundle, With,
    },
    text::TextStyle,
    ui::{AlignItems, BackgroundColor, Interaction, JustifyContent, Style, UiRect, Val},
};

use crate::{
    components::{ChooseGameButtonComponent, ChooseGameComponent},
    resources::GameNameChosen,
    AppState, LEVELS,
};

pub struct ChooseGamePlugin;

impl Plugin for ChooseGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // Enter choose game
            .add_system_set(
                SystemSet::on_enter(AppState::ChooseGame)
                    // init draw
                    .with_system(draw_choose_game),
            )
            // Run choose game
            .add_system_set(
                SystemSet::on_update(AppState::ChooseGame)
                    // init draw
                    .with_system(handle_choose_game),
            )
            // Exit choose game
            .add_system_set(
                SystemSet::on_exit(AppState::ChooseGame)
                    // init draw
                    .with_system(remove_choose_game),
            );
    }
}

fn remove_choose_game(
    mut commands: Commands,
    choose_game: Query<Entity, With<ChooseGameComponent>>,
) {
    let entity = choose_game.single();
    commands.entity(entity).despawn_recursive();
}

fn draw_choose_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("RobotoMedium-Owv4.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                size: bevy::ui::Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .insert(ChooseGameComponent)
        .with_children(|parent| {
            let num = LEVELS.len();
            let percent = 100. / num as f32;

            for (name, _) in LEVELS {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            size: bevy::ui::Size::new(Val::Percent(percent), Val::Percent(100.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            display: bevy::ui::Display::Flex,
                            flex_direction: bevy::ui::FlexDirection::Column,
                            align_items: bevy::ui::AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    size: bevy::ui::Size::new(Val::Px(150.0), Val::Px(65.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .insert(ChooseGameButtonComponent(name))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    name,
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                ));
                            });
                    });
            }
        });
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[allow(clippy::type_complexity)]
fn handle_choose_game(
    mut game_chosen: ResMut<GameNameChosen>,
    mut app_state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ChooseGameButtonComponent,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, choose_game_component) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                game_chosen.0 = Some(choose_game_component.0);
                app_state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
