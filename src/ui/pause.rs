use crate::MenuState;
use bevy::{app::AppExit, prelude::*};

use super::button::{render_button_on_parent, render_text_on_parent, ButtonColors, ButtonMarker};

#[derive(Component)]
pub struct PauseMenu;

pub fn render_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            PauseMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            render_text_on_parent(parent, "Adventure Paused");
            render_button_on_parent(parent, ButtonMarker::Resume, "Resume");
            render_button_on_parent(parent, ButtonMarker::Close, "Close");
        });
}

pub fn close_pause_menu(mut commands: Commands, pause_menu: Query<Entity, With<PauseMenu>>) {
    for entity in pause_menu.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn pause_menu_interactions(
    mut interaction_query: Query<
        (&Interaction, &ButtonMarker, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_game_state: ResMut<NextState<MenuState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, marker, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = ButtonColors::Pressed.into();
                handle_click(marker, &mut next_game_state, &mut exit);
            }
            Interaction::Hovered => {
                *color = ButtonColors::Hover.into();
            }
            Interaction::None => {
                *color = ButtonColors::Normal.into();
            }
        }
    }
}

fn handle_click(
    marker: &ButtonMarker,
    next_game_state: &mut ResMut<NextState<MenuState>>,
    exit: &mut EventWriter<AppExit>,
) {
    match marker {
        ButtonMarker::Resume => {
            next_game_state.set(MenuState::Ship);
        }
        ButtonMarker::Close => {
            exit.send(AppExit);
        }
    };
}
