use bevy::prelude::*;

pub enum ButtonColors {
    Normal,
    Hover,
    Pressed,
}

impl Into<BackgroundColor> for ButtonColors {
    fn into(self) -> BackgroundColor {
        match self {
            ButtonColors::Normal => Color::rgb(0.15, 0.15, 0.15).into(),
            ButtonColors::Hover => Color::rgb(0.25, 0.25, 0.25).into(),
            ButtonColors::Pressed => Color::rgb(0.75, 0.75, 0.75).into(),
        }
    }
}

#[derive(Component)]
pub enum ButtonMarker {
    Resume,
    Close,
}

pub fn render_button_on_parent(
    parent: &mut ChildBuilder,
    marker: ButtonMarker,
    text: &'static str,
) {
    parent
        .spawn((
            marker,
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: ButtonColors::Normal.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Default::default()
                },
            ));
        });
}

pub fn render_text_on_parent(parent: &mut ChildBuilder, text: &'static str) {
    parent.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font_size: 40.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            ..Default::default()
        },
    ));
}
