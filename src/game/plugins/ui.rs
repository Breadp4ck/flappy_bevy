use bevy::prelude::*;

use crate::game::{components::Scores, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(
                FixedUpdate,
                update_score_label.run_if(in_state(GameState::Game)),
            )
            .add_systems(OnEnter(GameState::Greet), show_ui::<GreetUi>)
            .add_systems(OnExit(GameState::Greet), hide_ui::<GreetUi>)
            .add_systems(OnEnter(GameState::Pause), show_ui::<PauseUi>)
            .add_systems(OnExit(GameState::Pause), hide_ui::<PauseUi>)
            .add_systems(OnEnter(GameState::Game), show_ui::<GameUi>)
            .add_systems(OnExit(GameState::Game), hide_ui::<GameUi>)
            .add_systems(OnEnter(GameState::End), show_ui::<EndUi>)
            .add_systems(OnExit(GameState::End), hide_ui::<EndUi>);
    }
}

#[derive(Component)]
struct GreetUi;

#[derive(Component)]
struct PauseUi;

#[derive(Component)]
struct GameUi;

#[derive(Component)]
struct EndUi;

#[derive(Component)]
struct ScoreLabel;

fn spawn_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        top: Val::Percent(20.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    font_size: 32.0,
                                    color: Color::ORANGE_RED,
                                    ..default()
                                },
                            )
                            .with_text_alignment(TextAlignment::Center)
                            .with_style(Style {
                                display: Display::None,
                                bottom: Val::Px(70.),
                                ..default()
                            }),
                        )
                        .insert(EndUi)
                        .insert(ScoreLabel);

                    parent
                        .spawn(
                            TextBundle::from_section(
                                "YOU CRASHED",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::CRIMSON,
                                    ..default()
                                },
                            )
                            .with_text_alignment(TextAlignment::Center)
                            .with_style(Style {
                                display: Display::None,
                                bottom: Val::Px(30.),
                                ..default()
                            }),
                        )
                        .insert(EndUi);

                    parent
                        .spawn(
                            TextBundle::from_section(
                                "Press <Space> to start game",
                                TextStyle {
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )
                            .with_text_alignment(TextAlignment::Center)
                            .with_style(Style {
                                display: Display::None,
                                bottom: Val::Px(10.),
                                ..default()
                            }),
                        )
                        .insert(GreetUi)
                        .insert(EndUi);

                    parent
                        .spawn(
                            TextBundle::from_section(
                                "Then <Space> to jump and <Esc> to pause",
                                TextStyle {
                                    font_size: 12.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )
                            .with_text_alignment(TextAlignment::Center)
                            .with_style(Style {
                                display: Display::None,
                                ..default()
                            }),
                        )
                        .insert(GreetUi)
                        .insert(EndUi);

                    parent
                        .spawn(
                            TextBundle::from_section(
                                "Press <Space> or <Esc> to resume game",
                                TextStyle {
                                    font_size: 12.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )
                            .with_text_alignment(TextAlignment::Center)
                            .with_style(Style {
                                display: Display::None,
                                ..default()
                            }),
                        )
                        .insert(PauseUi);
                });

            parent
                .spawn(
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 22.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        display: Display::None,
                        top: Val::Px(20.),
                        right: Val::Px(20.),
                        ..default()
                    }),
                )
                .insert(GameUi)
                .insert(PauseUi)
                .insert(ScoreLabel);
        });
}

fn show_ui<T: Component>(mut query: Query<(&T, &mut Style)>) {
    for (_, mut style) in &mut query {
        style.display = Display::Flex;
    }
}

fn hide_ui<T: Component>(mut query: Query<(&T, &mut Style)>) {
    for (_, mut style) in &mut query {
        style.display = Display::None;
    }
}

fn update_score_label(scores: Res<Scores>, mut query: Query<(&ScoreLabel, &mut Text)>) {
    for (_, mut text) in &mut query {
        text.sections[0].value = scores.0.to_string();
    }
}
