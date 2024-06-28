use bevy::prelude::*;
use bevy_rapier3d::{geometry::{Friction, Restitution}, dynamics::RigidBody};

use super::PrefabMarkerComponent;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Component, Reflect, Default)]
#[reflect(Component, PartialEq)]
pub enum RigidBodyPrebuf {
    /// A `RigidBody::Dynamic` body can be affected by all external forces.
    Dynamic,
    /// A `RigidBody::Fixed` body cannot be affected by external forces.
    #[default]
    Fixed,
    /// A `RigidBody::KinematicPositionBased` body cannot be affected by any external forces but can be controlled
    /// by the user at the position level while keeping realistic one-way interaction with dynamic bodies.
    ///
    /// One-way interaction means that a kinematic body can push a dynamic body, but a kinematic body
    /// cannot be pushed by anything. In other words, the trajectory of a kinematic body can only be
    /// modified by the user and is independent from any contact or joint it is involved in.
    KinematicPositionBased,
    /// A `RigidBody::KinematicVelocityBased` body cannot be affected by any external forces but can be controlled
    /// by the user at the velocity level while keeping realistic one-way interaction with dynamic bodies.
    ///
    /// One-way interaction means that a kinematic body can push a dynamic body, but a kinematic body
    /// cannot be pushed by anything. In other words, the trajectory of a kinematic body can only be
    /// modified by the user and is independent from any contact or joint it is involved in.
    KinematicVelocityBased,
}

impl PrefabMarkerComponent for RigidBodyPrebuf {
    type Component = RigidBody;
}

impl Into<RigidBody> for RigidBodyPrebuf {
    fn into(self) -> RigidBody {
        match self {
            RigidBodyPrebuf::Dynamic => RigidBody::Dynamic,
            RigidBodyPrebuf::Fixed => RigidBody::Fixed,
            RigidBodyPrebuf::KinematicPositionBased => RigidBody::KinematicPositionBased,
            RigidBodyPrebuf::KinematicVelocityBased => RigidBody::KinematicVelocityBased,
        }
    }
}

pub fn sync_rigid_body(
    mut query: Query<(&mut RigidBody, &RigidBodyPrebuf), (Changed<RigidBodyPrebuf>, With<RigidBody>)>,
) {
    for (mut body, prefab) in query.iter_mut() {
        *body = (*prefab).into();
    }
}

pub fn spawn_rigid_body(
    mut commands: Commands,
    query: Query<(Entity, &RigidBodyPrebuf), (Added<RigidBodyPrebuf>, Without<RigidBody>)>,
) {
    for (e, prefab) in query.iter() {
        let body: RigidBody = (*prefab).into();
        commands.entity(e)
            .insert(body);
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect, Default)]
#[reflect(Component, PartialEq)]
pub struct FrictionPrebuf {
    pub coefficient: f32,
    pub combine_rule: CoefficientCombineRulePrebuf,
}

impl PrefabMarkerComponent for FrictionPrebuf {
    type Component = Friction;
}

pub fn sync_friction(
    mut query: Query<(&mut Friction, &FrictionPrebuf), (Changed<FrictionPrebuf>, With<Friction>)>,
) {
    for (mut body, prefab) in query.iter_mut() {
        body.coefficient = f32::max(prefab.coefficient, 0.0);
        body.combine_rule = prefab.combine_rule.into();
    }
}

pub fn spawn_friction(
    mut commands: Commands,
    query: Query<(Entity, &FrictionPrebuf), (Added<FrictionPrebuf>, Without<Friction>)>,
) {
    for (e, prefab) in query.iter() {
        let FrictionPrebuf { coefficient, combine_rule } = *prefab;
        let friciton = Friction {coefficient, combine_rule: combine_rule.into()};
        commands.entity(e)
            .insert(friciton);
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect, Default)]
#[reflect(Component, PartialEq)]
pub struct RestitutionPrebuf {
    pub coefficient: f32,
    pub combine_rule: CoefficientCombineRulePrebuf,
}

impl PrefabMarkerComponent for RestitutionPrebuf {
    type Component = Restitution;
}

pub fn sync_restitution(
    mut query: Query<(&mut Restitution, &RestitutionPrebuf), (Changed<RestitutionPrebuf>, With<Restitution>)>,
) {
    for (mut body, prefab) in query.iter_mut() {
        body.coefficient = f32::max(prefab.coefficient, 0.0);
        body.combine_rule = prefab.combine_rule.into();
    }
}

pub fn spawn_restitution(
    mut commands: Commands,
    query: Query<(Entity, &RestitutionPrebuf), (Added<RestitutionPrebuf>, Without<Restitution>)>,
) {
    for (e, prefab) in query.iter() {
        let RestitutionPrebuf { coefficient, combine_rule } = *prefab;
        let restitution = Restitution {coefficient, combine_rule: combine_rule.into()};
        commands.entity(e)
            .insert(restitution);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Reflect, Default, Component)]
pub enum CoefficientCombineRulePrebuf {
    #[default]
    /// The two coefficients are averaged.
    Average = 0,
    /// The smallest coefficient is chosen.
    Min,
    /// The two coefficients are multiplied.
    Multiply,
    /// The greatest coefficient is chosen.
    Max,
}

impl From<bevy_rapier3d::dynamics::CoefficientCombineRule> for CoefficientCombineRulePrebuf {
    fn from(value: bevy_rapier3d::dynamics::CoefficientCombineRule) -> Self {
        match value {
            bevy_rapier3d::dynamics::CoefficientCombineRule::Average => CoefficientCombineRulePrebuf::Average, 
            bevy_rapier3d::dynamics::CoefficientCombineRule::Min => CoefficientCombineRulePrebuf::Min, 
            bevy_rapier3d::dynamics::CoefficientCombineRule::Multiply => CoefficientCombineRulePrebuf::Multiply, 
            bevy_rapier3d::dynamics::CoefficientCombineRule::Max => CoefficientCombineRulePrebuf::Max, 
        }
    }
}

impl Into<bevy_rapier3d::dynamics::CoefficientCombineRule> for CoefficientCombineRulePrebuf {
    fn into(self) -> bevy_rapier3d::dynamics::CoefficientCombineRule {
        match self {
            CoefficientCombineRulePrebuf::Average => bevy_rapier3d::dynamics::CoefficientCombineRule::Average, 
            CoefficientCombineRulePrebuf::Min => bevy_rapier3d::dynamics::CoefficientCombineRule::Min, 
            CoefficientCombineRulePrebuf::Multiply =>  bevy_rapier3d::dynamics::CoefficientCombineRule::Multiply, 
            CoefficientCombineRulePrebuf::Max => bevy_rapier3d::dynamics::CoefficientCombineRule::Max, 
        }
    }
}