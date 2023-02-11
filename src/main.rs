use bevy::prelude::*;
use thiserror::Error;

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

// match position.left {
//   Val::Percent(percent) => {
//       if percent > 50. {
//           return;
//       }
//       Val::Percent(percent + 25. * time.delta_seconds())
//   }
//   _ => {
//       return;
//   }
// }

#[derive(Error, Debug)]
enum NextValError {
    #[error("The animation has finished.")]
    Finished,
    #[error("The current value is not supported. Maybe init is wrong.")]
    NotPercent,
}

fn next_percent(current: Val, delta: f32) -> Result<Val, NextValError> {
    let current = match current {
        Val::Percent(p) => p,
        _ => return Err(NextValError::NotPercent),
    };
    if current >= 50. {
        return Err(NextValError::Finished);
    }
    Ok(Val::Percent(current + 25. * delta))
}

fn animate(mut query: Query<&mut Style, With<Marker>>, time: Res<Time>) {
    let mut style = query.single_mut();
    let position = &mut style.position;
    let left = next_percent(position.left, time.delta_seconds());
    let left = match left {
        Ok(l) => l,
        _ => {
            return;
        }
    };
    position.left = left;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate)
        .run();
}
