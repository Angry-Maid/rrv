//! Development tools. This plugin is only enabled in dev builds.

use bevy::prelude::*;
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};

pub const FLYCAM_SPEED: f32 = 10.;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DevState {
    #[default]
    Off,
    On,
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<DevState>()
        .add_plugins((NoCameraPlayerPlugin,))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                switch_to_dev_mode,
                (change_cams).run_if(state_changed::<DevState>),
            ),
        );
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                is_active: false,
                ..Default::default()
            },
            transform: Transform::from_xyz(-10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCam,
    ));
}

fn switch_to_dev_mode(
    mut r_dmode: ResMut<NextState<DevState>>,
    r_dev_state: Res<State<DevState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyF) {
        match **r_dev_state {
            DevState::Off => r_dmode.set(DevState::On),
            DevState::On => r_dmode.set(DevState::Off),
        }
    }
}

fn change_cams(
    mut q_cams: Query<(Entity, &mut Camera)>,
    mut r_m: ResMut<MovementSettings>,
    r_dev_state: Res<State<DevState>>,
    q_f: Query<&FlyCam>,
) {
    let val = *r_dev_state == DevState::On;

    q_cams.iter_mut().for_each(|(e, mut c)| {
        if q_f.contains(e) {
            c.is_active = val;
        } else {
            c.is_active = !val;
        }
    });

    if val {
        r_m.speed = FLYCAM_SPEED;
    } else {
        r_m.speed = 0.;
    }
}
