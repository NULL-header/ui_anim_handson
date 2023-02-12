mod percent;
use bevy::prelude::*;

#[derive(Component)]
pub struct Marker;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AnimateState {
    NotStarted,
    Setup,
    Animating,
    Finished,
}

fn animate(
    mut query: Query<&mut Style, With<Marker>>,
    time: Res<Time>,
    mut state: ResMut<State<AnimateState>>,
) {
    let mut style = query.single_mut();
    let position = &mut style.position;
    let mut left = percent::Percent::new(position.left);
    let left = left.add(25. * time.delta_seconds()).round();
    if left.should_finish() {
        state.set(AnimateState::Finished).unwrap();
    }
    position.left = left.get();
}

fn setup(
    mut query: Query<&mut Style, With<Marker>>,
    windows: Res<Windows>,
    mut state: ResMut<State<AnimateState>>,
) {
    let position = &mut query.single_mut().position;
    let left = position.left;
    let left = match left {
        Val::Percent(percent) => Val::Percent(percent),
        Val::Px(px) => {
            let width_window = windows.get_primary().unwrap().width();
            Val::Percent(px / width_window * 100.)
        }
        _ => Val::Percent(10.),
    };
    position.left = left;
    state.overwrite_set(AnimateState::Animating).unwrap();
}

pub struct SlidePlugin;

impl Plugin for SlidePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AnimateState::NotStarted)
            .add_system_set(SystemSet::on_enter(AnimateState::Setup).with_system(setup))
            .add_system_set(SystemSet::on_update(AnimateState::Animating).with_system(animate));
    }
}

pub fn fire_animate_system(mut state: ResMut<State<AnimateState>>) {
    state.overwrite_set(AnimateState::Setup).unwrap();
}
