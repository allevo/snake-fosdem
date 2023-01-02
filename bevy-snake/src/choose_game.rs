use bevy::{
    prelude::{
        default, BuildChildren, Button, ButtonBundle, Changed, Color, Commands,
        DespawnRecursiveExt, Entity, EventWriter, NodeBundle, Plugin, Query, Res, SystemSet,
        TextBundle, With,
    },
    text::TextStyle,
    ui::{AlignItems, BackgroundColor, Interaction, JustifyContent, Style, UiRect, Val},
};

use crate::{
    components::{ChooseGameButtonComponent, ChooseGameComponent},
    events::GameChosen,
    AppState, LEVELS, resources::Assets,
};

pub struct ChooseGamePlugin;

impl Plugin for ChooseGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<GameChosen>()
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

fn draw_choose_game(
    mut commands: Commands,
    assets: Res<Assets>,
) {
    let font = assets.font.clone();

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
                                background_color: assets.normal_button_color.into(),
                                ..default()
                            })
                            .insert(ChooseGameButtonComponent(name))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    name,
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 30.0,
                                        color: assets.text_button_color,
                                    },
                                ));
                            });
                    });
            }
        });
}

#[allow(clippy::type_complexity)]
fn handle_choose_game(
    mut game_chosen_writer: EventWriter<GameChosen>,
    assets: Res<Assets>,
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
                *color = assets.pressed_button_color.into();

                let game_name = choose_game_component.0;
                game_chosen_writer.send(GameChosen(game_name));
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
