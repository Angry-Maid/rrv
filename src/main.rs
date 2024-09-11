use std::f32::consts::PI;

#[cfg(feature = "dev")]
mod dev_tools;

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
    audio::{AudioPlugin, Volume},
    color::palettes::css::CADET_BLUE,
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};
use rrv_core::prelude::{parse_replay, Geometry};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "GTFO Replay Viewer".to_string(),
                    canvas: Some("#bevy".to_string()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }
                .into(),
                ..default()
            })
            .set(AudioPlugin {
                global_volume: GlobalVolume {
                    volume: Volume::new(0.3),
                },
                ..default()
            }),
        #[cfg(not(target_arch = "wasm32"))]
        WireframePlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            #[cfg(not(target_arch = "wasm32"))]
            toggle_wireframe,
        ),
    );

    #[cfg(feature = "dev")]
    app.add_plugins(dev_tools::plugin);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.brightness = light_consts::lux::OVERCAST_DAY;

    let level_geometry_mat = materials.add(StandardMaterial {
        base_color: Color::from(CADET_BLUE),
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    let (_, replay) = parse_replay(include_bytes!("../data/R8E2 2024-08-09 19-55")).unwrap();

    for geometry in replay.header.level_geometry {
        let shape: Handle<Mesh> = meshes.add(create_level_geometry_mesh(&geometry));
        commands.spawn((PbrBundle {
            mesh: shape,
            material: level_geometry_mat.clone(),
            ..default()
        },));
    }

    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         illuminance: light_consts::lux::DIRECT_SUNLIGHT,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0., 20., 0.),
    //         rotation: Quat::from_rotation_x(-PI / 4.),
    //         ..default()
    //     },
    //     ..default()
    // });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

fn create_level_geometry_mesh(geometry: &Geometry) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        geometry
            .vertices
            .iter()
            .map(|v| v.to_array())
            .collect::<Vec<[f32; 3]>>(),
    )
    .with_inserted_indices(Indices::U16(geometry.indices.clone()))
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        wireframe_config.global = !wireframe_config.global;
    }
}

// use log::info;
// use rrv_core::prelude::parse_replay;

// fn main() {
//     env_logger::init();

//     let _ = parse_replay(include_bytes!("../data/R8E2 2024-08-09 19-55"));
// }
