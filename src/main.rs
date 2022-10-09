//! This example illustrates the various features of Bevy UI.

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(mouse_scroll)
        .add_system(button_system)
        .run();
}

const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const CADET_BLUE: Color = Color::rgb(95. / 255., 158. / 255., 160. / 255.);
const TEAL: Color = Color::rgb(0. / 255., 128. / 255., 128. / 255.);
const TEAL_GREEN: Color = Color::rgb(0. / 255., 130. / 255., 127. / 255.);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    // mut image_query: Query<&mut Image>
) {
    for (interaction, mut color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = CADET_BLUE.into();
            }
            Interaction::None => {
                *color = TEAL.into();
            }
        }
    }
}

fn generate_left_side_panel() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(33.0)),
            align_items: AlignItems::FlexEnd,
            margin: UiRect::top(Val::Px(3.0)),
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    }
}

fn generate_left_side_buttons() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(33.0)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: TEAL_GREEN.into(),
        ..default()
    }
}

fn generate_imageBundle(asset_server: &Res<AssetServer>, image: &str) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            ..default()
        },
        focus_policy: bevy::ui::FocusPolicy::Pass,
        image: asset_server.load(image).into(),
        ..default()
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(10.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        border: UiRect::right(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(generate_left_side_buttons())
                        .with_children(|parent| {
                            // image
                            parent.spawn(generate_imageBundle(&asset_server, "images/back.png"));
                        });
                    parent
                        .spawn(generate_left_side_buttons())
                        .with_children(|parent| {
                            // text
                            parent.spawn(generate_imageBundle(&asset_server, "images/home.png"));
                        });
                    parent
                        .spawn(generate_left_side_buttons())
                        .with_children(|parent| {
                            parent.spawn(generate_imageBundle(&asset_server, "images/menu.png"));
                        });
                });
            //right vertical split
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(90.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    background_color: CADET_BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    //top bar
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
                                flex_direction: FlexDirection::RowReverse,
                                margin: UiRect::bottom(Val::Px(5.0)),
                                ..default()
                            },
                            background_color: TEAL.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "04:20",
                                    TextStyle {
                                        font: asset_server.load(
                                            "fonts/Code New Roman Nerd Font Complete Mono.otf",
                                        ),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(5.0), Val::Percent(100.0)),
                                    padding: UiRect::right(Val::Percent(1.0)),
                                    ..default()
                                },
                                calculated_size: CalculatedSize {
                                    size: Size {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                    },
                                },
                                image: asset_server.load("images/network.png").into(),
                                ..default()
                            });
                            parent.spawn(
                                TextBundle::from_section(
                                    "5G",
                                    TextStyle {
                                        font: asset_server.load(
                                            "fonts/Code New Roman Nerd Font Complete Mono.otf",
                                        ),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    padding: UiRect::right(Val::Percent(1.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(5.0), Val::Percent(100.0)),
                                    padding: UiRect::right(Val::Percent(1.0)),
                                    position_type: PositionType::Absolute,

                                    position: UiRect::left(Val::Percent(0.0)),
                                    ..default()
                                },
                                calculated_size: CalculatedSize {
                                    size: Size {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                    },
                                },
                                image: asset_server.load("images/user.png").into(),
                                ..default()
                            });
                        });
                });
        });
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
