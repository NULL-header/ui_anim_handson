mod slide_plugin;
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<slide_plugin::AnimateState>>,
) {
    commands.spawn(Camera3dBundle::default());

    let font: Handle<Font> = asset_server.load("Roboto-Regular.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "Text",
                        TextStyle {
                            font,
                            font_size: 50.,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        position: UiRect {
                            left: Val::Percent(10.),
                            ..default()
                        },
                        ..default()
                    }),
                )
                .insert(slide_plugin::Marker);
        });
    state
        .overwrite_set(slide_plugin::AnimateState::Animating)
        .unwrap();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(slide_plugin::SlidePlugin)
        .add_startup_system(setup)
        .run();
}
