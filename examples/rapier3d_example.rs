use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::{dynamics::ImpulseJoint, geometry::Collider, plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use space_bevy_rapier3d_plugin::joint::generic_joint::{AdvancedSettings, Axis, BasicSettings, GenericJointPrefab, JointAxisPrefab, List, MotorPrebuf, SphericalSettings};
use space_editor::prelude::*;
use space_editor_ui::ext::bevy_panorbit_camera;

//This example loading prefab with bevy_xpbd types

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(SpaceEditorPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands
    //     .spawn(PrefabBundle::new("rapier3d.scn.ron"));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(bevy_panorbit_camera::PanOrbitCamera::default())
        .insert(EditorCameraMarker);

    let step = materials.add(StandardMaterial {
            base_color: Color::BLACK,
            perceptual_roughness: 1.0,
            ..default()
    });
    let i = 2.5;
    let size = i as f32 / 2.0 + 3.0;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Box {
                min_x: -size,
                max_x: size,
                min_z: -size,
                max_z: size,
                min_y: -0.25,
                max_y: 0.25,
            })),
            material: step.clone(),
            transform: Transform::from_translation(Vec3::new(0., -0.25, 0.)),
            ..default()
        }, 
        bevy_rapier3d::dynamics::RigidBody::Fixed, 
        Collider::cuboid(size, 0.25, size),
        Name::new("platform")
    ));

    let car_transfrom = Transform::default();

    commands.spawn((
        Name::new("car"),
        car_transfrom,
    )).with_children(|car| {
        pub struct CarSize {
            pub hw: f32,
            pub hh: f32,
            pub hl: f32,
        }
        let size = CarSize {
            hw: 1.,
            hh: 0.35,
            hl: 2.2,
        };
        
        let body = car.spawn((
            bevy_rapier3d::dynamics::RigidBody::Dynamic, 
            Collider::cuboid(size.hh, size.hh, size.hw),
            Transform::from_translation(Vec3::new(0., 0.2, 0.)),
            Name::new("body")
        )).id();

        let ride_height = 0.06;
        let wheel_radius: f32 = 0.35;
        let wheel_width: f32 = 0.34;

        let shift = Vec3::new(
            size.hw - wheel_width / 2. - 0.1,
            -size.hh + wheel_radius - ride_height,
            size.hl - wheel_radius - 0.5,
        );

        let anchors: [(Vec3, bool, bool); 4] = [
            (Vec3::new(shift.x, shift.y, shift.z), true, false), // front right
            (Vec3::new(-shift.x, shift.y, shift.z), true, true), // front left
            (Vec3::new(shift.x, shift.y, -shift.z), false, false), // rear right
            (Vec3::new(-shift.x, shift.y, -shift.z), false, true), // rear left
        ];
        for (anchor,_, is_left ) in anchors {
            let generic_joint_prefab = build_prefab_joint(anchor, is_left);
            let generic_joint = generic_joint_prefab.as_joint(anchor, Vec3::ZERO);
            let joint = ImpulseJoint::new(body, generic_joint);
            let translation =  car_transfrom.rotation.mul_vec3(anchor);
            let transform = Transform::from_translation(translation)
                .with_rotation(Quat::from_axis_angle(Vec3::Y, PI));
            let collider = Collider::round_cylinder(
                0.05,
                0.25,
                0.01,
            );
            car.spawn((
                Name::new("wheel"),
                bevy_rapier3d::dynamics::RigidBody::Dynamic, 
                collider,
                joint,
                transform,
                generic_joint_prefab
            ));
        }
    });
}


pub fn build_prefab_joint(anchor: Vec3, is_left: bool) -> GenericJointPrefab {
    let joint = GenericJointPrefab::GenericJoint { 
        locked_axes: List (vec![JointAxisPrefab::AngY , JointAxisPrefab::AngZ , JointAxisPrefab::X , JointAxisPrefab::Z]), 
        basis_settings: List(vec![
            BasicSettings::LocalAxis1(Axis::X),
            BasicSettings::LocalAxis2(match is_left {
                true => Axis::NegY,
                false => Axis::Y,
            }),
            BasicSettings::LocalBasic1(Quat::from_axis_angle(Vec3::Y, 0.)),
        ]),
        advanced_settings: List(vec![
            SphericalSettings {
                axis: JointAxisPrefab::Y,
                setting: AdvancedSettings::Motor(MotorPrebuf {
                    target_pos: 0.,
                    target_vel: 0.,
                    stiffness: 1e6,
                    damping: 1e3
                })
            }
        ]) 
    };
    joint
}