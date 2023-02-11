use bevy::prelude::*;

#[derive(Component)]
struct Marker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                .insert(Marker);
        });
}

fn animate(mut query: Query<&mut Transform, With<Marker>>) {
    let mut transform = query.single_mut();
    let x = transform.translation.x;
    let x = x + 1.;
    transform.translation.x = x;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate)
        .run();
}
