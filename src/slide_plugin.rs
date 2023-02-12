use bevy::prelude::*;
use thiserror::Error;

#[derive(Component)]
pub struct Marker;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AnimateState {
    NotStarted,
    Animating,
    Finished,
}

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

fn animate(
    mut query: Query<&mut Style, With<Marker>>,
    time: Res<Time>,
    mut state: ResMut<State<AnimateState>>,
) {
    let mut style = query.single_mut();
    let position = &mut style.position;
    let left = next_percent(position.left, time.delta_seconds());
    let left = match left {
        Ok(l) => l,
        _ => {
            state.set(AnimateState::Finished).unwrap();
            return;
        }
    };
    position.left = left;
}

pub struct SlidePlugin;

impl Plugin for SlidePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AnimateState::NotStarted)
            .add_system_set(SystemSet::on_update(AnimateState::Animating).with_system(animate));
    }
}
