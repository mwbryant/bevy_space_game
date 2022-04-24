use crate::prelude::*;

use super::UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_setup)
            .add_startup_system(setup_gas_ui);
    }
}

fn setup_gas_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello bevy!",
                TextStyle {
                    font: asset_server.load("QuattrocentoSans-Bold.ttf"),
                    font_size: 36.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(GasText);
}

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    //commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position: Rect {
                    bottom: Val::Percent(1.0),
                    ..Default::default()
                },
                size: Size::new(Val::Percent(100.0), Val::Auto),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        margin: Rect {
                            left: Val::Px(10.),
                            right: Val::Px(10.),
                            ..Default::default()
                        },
                        justify_content: JustifyContent::FlexEnd,
                        size: Size::new(Val::Px(50.), Val::Px(50.)),
                        ..default()
                    },
                    color: Color::RED.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::FlexStart,
                            margin: Rect {
                                right: Val::Px(3.),
                                bottom: Val::Px(3.),
                                ..default()
                            },
                            ..default()
                        },
                        text: Text::with_section(
                            "10",
                            TextStyle {
                                font: asset_server.load("QuattrocentoSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                            TextAlignment::default(),
                        ),

                        ..default()
                    });
                });
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    margin: Rect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(50.), Val::Px(50.)),
                    ..default()
                },
                color: Color::BLUE.into(),
                ..default()
            });
        });
}
