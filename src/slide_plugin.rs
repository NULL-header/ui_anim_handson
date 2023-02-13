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
    dbg!(time.delta_seconds());
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

#[cfg(test)]
mod integrationtest {
    use std::time::Duration;

    use super::*;
    use bevy::{log::LogPlugin, time::TimeUpdateStrategy, window::WindowId, winit::WinitPlugin};
    use rstest::*;

    struct AppTool {
        app: App,
    }

    impl AppTool {
        fn new(app: App) -> Self {
            Self { app }
        }
        fn update(&mut self) {
            self.app.update();
        }
        fn time(&mut self) -> Mut<Time> {
            self.app.world.resource_mut::<Time>()
        }
        fn state(&self) -> &State<AnimateState> {
            self.app.world.resource::<State<AnimateState>>()
        }
    }

    #[fixture]
    fn app_tool() -> AppTool {
        let mut app = App::new();
        let window = Window::new(
            WindowId::primary(),
            &WindowDescriptor {
                width: 200.,
                height: 200.,
                ..default()
            },
            200,
            200,
            1.,
            None,
            None,
        );
        app.add_plugins(
            DefaultPlugins
                .build()
                .disable::<WinitPlugin>()
                .disable::<LogPlugin>(),
        )
        .add_plugin(SlidePlugin)
        .add_startup_system(fire_animate_system)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs(2)));
        app.world.resource_mut::<Windows>().add(window);
        app.world
            .spawn(TextBundle::from_section("Mock", TextStyle::default()))
            .insert(Marker);
        AppTool::new(app)
    }

    #[rstest]
    fn init_state(app_tool: AppTool) {
        match app_tool.state().current() {
            AnimateState::NotStarted => {}
            _ => {
                panic!("The initial state is wrong.");
            }
        }
    }

    // setup step is resolved with one frame.
    #[rstest]
    fn animating_state(mut app_tool: AppTool) {
        app_tool.update();
        match app_tool.state().current() {
            AnimateState::Animating => {}
            state => {
                panic!("The animate step is not working with {:?}.", state);
            }
        }
    }

    #[rstest]
    fn finished_state(mut app_tool: AppTool) {
        app_tool.update();
        // to move forward clock in app
        app_tool.time().update();
        app_tool.update();
        match app_tool.state().current() {
            AnimateState::Finished => {}
            state => {
                panic!("The animate has not stoped with {:?}.", state);
            }
        }
    }
}
