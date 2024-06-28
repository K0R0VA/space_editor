use bevy::prelude::*;

use bevy_rapier3d::{
    dynamics::{
    Ccd, GravityScale, LockedAxes,  Sleeping, Velocity, Damping, ExternalForce, ExternalImpulse, Dominance
}, 
    geometry::{ColliderMassProperties, Sensor }};
use space_editor_ui::prelude::EditorRegistryExt;

use crate::{geometry::{spawn_friction, spawn_restitution, spawn_rigid_body, sync_friction, sync_restitution, sync_rigid_body, CoefficientCombineRulePrebuf, FrictionPrebuf, RestitutionPrebuf, RigidBodyPrebuf}, joint::{generic_joint::{AdvancedSettings, GenericJointPrefab, LimitsPrebuf, List, MotorModelPrebuf, MotorPositionPrebuf, MotorPrebuf, MotorVelocityPrebuf, SphericalSettings}, local_anchor_marker::{sync_local_anchor1, sync_local_anchor2, LocalAnchorEntityStorage}, name_map::{load_map, sync_map, JointBodyBuilderHashMap, JointBodyBuilderPrefab}, prebaf::{spawn_joint, sync_joint, ImpulseJointPrefab}}, prelude::{spawn_collider, sync_collider, ColliderPrefab}, PrefabMarkerComponent};


pub struct BevyRapierPlugin;

impl Plugin for BevyRapierPlugin {
    fn build(&self, app: &mut App) {
        // colliders
        app
            .editor_registry::<ColliderPrefab>()
            .editor_registry::<RigidBodyPrebuf>()
            .editor_registry::<ColliderMassProperties>()
            .editor_registry::<FrictionPrebuf>()
            .editor_registry::<RestitutionPrebuf>()
            .editor_registry::<Velocity>()
            .editor_registry::<Damping>()
            .editor_registry::<Dominance>()
            .editor_registry::<ExternalForce>()
            .editor_registry::<ExternalImpulse>()
            .editor_registry::<LockedAxes>()
            .editor_registry::<GravityScale>()
            .editor_registry::<Sensor>()
            .editor_registry::<Sleeping>()
            .editor_registry::<Ccd>()
            // TODO: add enums for this types
            // .editor_registry::<ActiveCollisionTypes>()
            // .editor_registry::<ActiveHooks>()
            // .editor_registry::<ActiveEvents>()
            // .editor_registry::<Group>()
            .editor_silent_registry::<CoefficientCombineRulePrebuf>();

        app
            .editor_registry::<JointBodyBuilderPrefab>()
            .editor_relation::<JointBodyBuilderPrefab, JointBodyBuilderHashMap>();

        app.register_type::<JointBodyBuilderHashMap>();
        app.register_type::<LocalAnchorEntityStorage>();

        app.add_systems(Update, (
                load_map,
                sync_map,
                spawn_rigid_body, 
                spawn_collider, 
                spawn_joint, 
                sync_collider, 
                sync_rigid_body, 
                sync_restitution,
                spawn_restitution,
                spawn_friction,
                sync_friction,
            ));

        app.add_systems(Update, (sync_local_anchor1, sync_local_anchor2, sync_joint));

        app
            .editor_registry::<ImpulseJointPrefab>()
            .editor_silent_registry::<GenericJointPrefab>()
            .editor_silent_registry::<MotorModelPrebuf>()
            .editor_silent_registry::<MotorPositionPrebuf>()
            .editor_silent_registry::<MotorVelocityPrebuf>()   
            .editor_silent_registry::<MotorPrebuf>()
            .editor_silent_registry::<LimitsPrebuf>()
            .editor_silent_registry::<AdvancedSettings>()
            .register_type::<List<AdvancedSettings>>()
            .editor_silent_registry::<crate::joint::generic_joint::Axis>()
            .editor_silent_registry::<crate::joint::generic_joint::JointAxisPrefab>()
            .editor_silent_registry::<SphericalSettings>()
            .register_type::<List<SphericalSettings>>();

        app.add_systems(Update, (
            detect_removals::<ColliderPrefab>, 
            detect_removals::<RigidBodyPrebuf>,
            detect_removals::<ImpulseJointPrefab>,
        ));
    }
}

fn detect_removals<P: PrefabMarkerComponent>(
    mut commands: Commands,
    mut removals: RemovedComponents<P>,
) {
    for prebuf in removals.read() {
        let Some(mut entity) = commands.get_entity(prebuf) else {continue};
        P::remove_component(&mut entity);
    }
}