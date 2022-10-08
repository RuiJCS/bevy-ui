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


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
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
                        size: Size::new(Val::Percent(15.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        border: UiRect::right(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(generate_left_side_buttons())
                        .with_children(|parent| {
                            // text
                            parent
                                .spawn(
                                    ImageBundle {
                                        style: Style {
                                            size: Size::new(Val::Auto, Val::Auto),
                                            ..default()
                                        },
                                        image: asset_server.load("images/back.png").into(),
                                        ..default()
                                    }
                                );
                            });
                    parent
                        .spawn(generate_left_side_buttons())
                        .with_children(|parent| {
                            // text
                        parent
                            .spawn(
                                ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Auto, Val::Auto),
                                        ..default()
                                    },
                                    focus_policy: bevy::ui::FocusPolicy::Pass,
                                    image: asset_server.load("images/home.png").into(),
                                    ..default()
                                }
                            );
                        });
                        parent
                        .spawn(generate_left_side_buttons())
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Auto, Val::Auto),
                                    ..default()
                                },
                                image: asset_server.load("images/menu.png").into(),
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
