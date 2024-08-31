// use std::f32::consts::PI;

// #[cfg(not(target_arch = "wasm32"))]
// use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
// use bevy::{
//     audio::{AudioPlugin, Volume},
//     color::palettes::css::SILVER,
//     prelude::*,
//     render::{
//         render_asset::RenderAssetUsages,
//         render_resource::{Extent3d, TextureDimension, TextureFormat},
//     },
// };
// use leafwing_input_manager::prelude::*;

// fn main() {
//     env_logger::init();

//     App::new()
//         .add_plugins((
//             DefaultPlugins
//                 .set(AssetPlugin {
//                     meta_check: bevy::asset::AssetMetaCheck::Never,
//                     ..default()
//                 })
//                 .set(WindowPlugin {
//                     primary_window: Window {
//                         title: "GTFO Replay Viewer".to_string(),
//                         canvas: Some("#bevy".to_string()),
//                         fit_canvas_to_parent: true,
//                         prevent_default_event_handling: true,
//                         ..default()
//                     }
//                     .into(),
//                     ..default()
//                 })
//                 .set(AudioPlugin {
//                     global_volume: GlobalVolume {
//                         volume: Volume::new(0.3),
//                     },
//                     ..default()
//                 }),
//             #[cfg(not(target_arch = "wasm32"))]
//             WireframePlugin,
//         ))
//         .add_plugins(InputManagerPlugin::<CameraMovement>::default())
//         .add_systems(Startup, setup)
//         .add_systems(
//             Update,
//             (
//                 pan_camera,
//                 #[cfg(not(target_arch = "wasm32"))]
//                 toggle_wireframe,
//             ),
//         )
//         .run();
// }

// #[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
// enum CameraMovement {
//     #[actionlike(DualAxis)]
//     Pan,
// }

// #[derive(Component)]
// struct Shape;

// const SHAPES_X_EXTENT: f32 = 14.0;
// const EXTRUSION_X_EXTENT: f32 = 16.0;
// const Z_EXTENT: f32 = 5.0;

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut images: ResMut<Assets<Image>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let input_map = InputMap::default().with_dual_axis(CameraMovement::Pan, MouseMove::default());

//     let debug_material = materials.add(StandardMaterial {
//         base_color_texture: Some(images.add(uv_debug_texture())),
//         ..default()
//     });

//     let shapes = [
//         meshes.add(Cuboid::default()),
//         meshes.add(Tetrahedron::default()),
//         meshes.add(Capsule3d::default()),
//         meshes.add(Torus::default()),
//         meshes.add(Cylinder::default()),
//         meshes.add(Cone::default()),
//         meshes.add(ConicalFrustum::default()),
//         meshes.add(Sphere::default().mesh().ico(5).unwrap()),
//         meshes.add(Sphere::default().mesh().uv(32, 18)),
//     ];

//     let extrusions = [
//         meshes.add(Extrusion::new(Rectangle::default(), 1.)),
//         meshes.add(Extrusion::new(Capsule2d::default(), 1.)),
//         meshes.add(Extrusion::new(Annulus::default(), 1.)),
//         meshes.add(Extrusion::new(Circle::default(), 1.)),
//         meshes.add(Extrusion::new(Ellipse::default(), 1.)),
//         meshes.add(Extrusion::new(RegularPolygon::default(), 1.)),
//         meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
//     ];

//     let num_shapes = shapes.len();

//     for (i, shape) in shapes.into_iter().enumerate() {
//         commands.spawn((
//             PbrBundle {
//                 mesh: shape,
//                 material: debug_material.clone(),
//                 transform: Transform::from_xyz(
//                     -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
//                     2.0,
//                     Z_EXTENT / 2.,
//                 )
//                 .with_rotation(Quat::from_rotation_x(-PI / 4.)),
//                 ..default()
//             },
//             Shape,
//         ));
//     }

//     let num_extrusions = extrusions.len();

//     for (i, shape) in extrusions.into_iter().enumerate() {
//         commands.spawn((
//             PbrBundle {
//                 mesh: shape,
//                 material: debug_material.clone(),
//                 transform: Transform::from_xyz(
//                     -EXTRUSION_X_EXTENT / 2.
//                         + i as f32 / (num_extrusions - 1) as f32 * EXTRUSION_X_EXTENT,
//                     2.0,
//                     -Z_EXTENT / 2.,
//                 )
//                 .with_rotation(Quat::from_rotation_x(-PI / 4.)),
//                 ..default()
//             },
//             Shape,
//         ));
//     }

//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             shadows_enabled: true,
//             intensity: 10_000_000.,
//             range: 100.0,
//             shadow_depth_bias: 0.2,
//             ..default()
//         },
//         transform: Transform::from_xyz(8.0, 16.0, 8.0),
//         ..default()
//     });

//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
//         material: materials.add(Color::from(SILVER)),
//         ..default()
//     });

//     commands
//         .spawn(Camera3dBundle {
//             transform: Transform::from_xyz(0.0, 7., 14.0)
//                 .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
//             ..default()
//         })
//         .insert(InputManagerBundle::with_map(input_map));

//     #[cfg(not(target_arch = "wasm32"))]
//     commands.spawn(
//         TextBundle::from_section("Press space to toggle wireframes", TextStyle::default())
//             .with_style(Style {
//                 position_type: PositionType::Absolute,
//                 top: Val::Px(12.0),
//                 left: Val::Px(12.0),
//                 ..default()
//             }),
//     );
// }

// fn uv_debug_texture() -> Image {
//     const TEXTURE_SIZE: usize = 8;

//     let mut palette: [u8; 32] = [
//         255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
//         198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
//     ];

//     let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
//     for y in 0..TEXTURE_SIZE {
//         let offset = TEXTURE_SIZE * y * 4;
//         texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
//         palette.rotate_right(4);
//     }

//     Image::new_fill(
//         Extent3d {
//             width: TEXTURE_SIZE as u32,
//             height: TEXTURE_SIZE as u32,
//             depth_or_array_layers: 1,
//         },
//         TextureDimension::D2,
//         &texture_data,
//         TextureFormat::Rgba8UnormSrgb,
//         RenderAssetUsages::RENDER_WORLD,
//     )
// }

// fn pan_camera(mut query: Query<(&mut Transform, &ActionState<CameraMovement>), With<Camera2d>>) {
//     const CAMERA_PAN_RATE: f32 = 0.5;

//     let (mut camera_transform, action_state) = query.single_mut();

//     let camera_pan_vector = action_state.axis_pair(&CameraMovement::Pan);

//     // Because we're moving the camera, not the object, we want to pan in the opposite direction.
//     // However, UI coordinates are inverted on the y-axis, so we need to flip y a second time.
//     camera_transform.translation.x -= CAMERA_PAN_RATE * camera_pan_vector.x;
//     camera_transform.translation.y += CAMERA_PAN_RATE * camera_pan_vector.y;
// }

// #[cfg(not(target_arch = "wasm32"))]
// fn toggle_wireframe(
//     mut wireframe_config: ResMut<WireframeConfig>,
//     keyboard: Res<ButtonInput<KeyCode>>,
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         wireframe_config.global = !wireframe_config.global;
//     }
// }

use log::info;
use rrv_core::prelude::parse_replay_file;

fn main() {
    env_logger::init();

    let _ = parse_replay_file(include_bytes!("../data/R8E2 2024-08-09 19-55"));
}
