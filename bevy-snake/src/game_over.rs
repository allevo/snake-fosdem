use bevy::{prelude::{EventReader, Plugin, SystemSet, ResMut, State, Res, Commands, AssetServer, NodeBundle, default, BuildChildren, TextBundle, Color, ButtonBundle, Query, Changed, Button, With, EventWriter}, ui::{Style, Val, JustifyContent, Display, AlignContent, AlignItems, Interaction, BackgroundColor}, text::TextStyle, app::AppExit};

use crate::{events::GameOver, AppState, resources::Assets, components::QuitComponent};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system(wait_for_game_over_event)
            .add_system_set(
                SystemSet::on_enter(AppState::GameOver(""))
                    // init draw
                    .with_system(show_game_over_screen),
            )
            .add_system_set(
                SystemSet::on_update(AppState::GameOver(""))
                    // init draw
                    .with_system(handle_quit),
            );;
    }
}

fn wait_for_game_over_event(
    mut game_over_reader: EventReader<GameOver>,
    mut app_state: ResMut<State<AppState>>,
) {
    if let Some(reason) = game_over_reader.iter().last() {
        app_state.set(AppState::GameOver(reason.0)).unwrap();
    }
}

fn show_game_over_screen(
    mut commands: Commands,
    app_state: Res<State<AppState>>,
    assets: Res<Assets>,
) {
    let font = assets.font.clone();

    let reason = match app_state.current() {
        AppState::GameOver(reason) => reason,
        _ => unreachable!("`show_game_over_screen` should be called only if the state is in gameover"),
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: bevy::ui::PositionType::Absolute,
                size: bevy::ui::Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                display: Display::Flex,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                flex_direction: bevy::ui::FlexDirection::Column,
                ..default()
            },
            background_color: assets.overlay_background_color.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                reason.to_owned(),
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: bevy::ui::Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: assets.normal_button_color.into(),
                    ..default()
                })
                .insert(QuitComponent)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: assets.text_button_color,
                        },
                    ));
                });
        });

}

#[allow(clippy::type_complexity)]
fn handle_quit(
    assets: Res<Assets>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
        ),
        (Changed<Interaction>, With<Button>, With<QuitComponent>),
    >,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = assets.pressed_button_color.into();
                exit.send(AppExit);
            }
            Interaction::Hovered => {
                *color = assets.hovered_button_color.into();
            }
            Interaction::None => {
                *color = assets.normal_button_color.into();
            }
        }
    }
}
