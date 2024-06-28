
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::dynamics::ImpulseJoint;

use crate::{joint::local_anchor_marker::LocalAnchorEntityStorage, ChangedOrAdded};

use super::prebaf::ImpulseJointPrefab;

#[derive(Reflect, Debug, Clone, Component, Default, Copy, Hash, PartialEq, Eq)]
#[reflect(Component, Default)]
pub enum JointBodyBuilderPrefab {
    #[default]
    Default
}

#[derive(Component, Default, Reflect)]
pub struct JointBodyBuilderHashMap (pub HashMap<String, Entity>);

pub fn sync_map(
    mut commands: Commands,
    mut map: Query<&mut JointBodyBuilderHashMap>,
    parent_joint_query: Query
    <
        (Entity, &Name, &Parent),
        (ChangedOrAdded<Name>, With<Parent>), 
    >,
    mut entity_joint_query: Query<(Entity, &ImpulseJointPrefab, Option<&mut LocalAnchorEntityStorage>, &Parent), Without<ImpulseJoint>>
) {
    for (parent_joint_entity, name, parent_body) in parent_joint_query.iter() {
        let parent_body_entity = parent_body.get();
        for (joint_entity, prefab, mut storage, entity_parent) in entity_joint_query.iter_mut() {
            if !(entity_parent == parent_body && prefab.parent_name == name.as_str()) { continue; }
            LocalAnchorEntityStorage::swawn(storage.as_deref_mut(), &mut commands, prefab, joint_entity, parent_joint_entity);
            if !prefab.enabled { continue };
            let generic_joint = prefab.joint.as_joint(prefab.local_anchor1, prefab.local_anchor2);
            commands.entity(joint_entity)
                .insert(ImpulseJoint::new(parent_joint_entity, generic_joint));
        }
        let Ok(mut map) = map.get_mut(parent_body_entity) else {continue};
        let Some(saved_entity) = map.0.get_mut(name.as_str()) else {
            map.0.insert(name.as_str().to_owned(), parent_joint_entity);
            continue
        };
        *saved_entity = parent_joint_entity;
    }
}

pub fn load_map(
    mut commands: Commands,
    mut map: Query<&mut JointBodyBuilderHashMap>,
    parent_joint_query: Query
    <
        (Entity, &Name, &Parent),
        (Added<Parent>, With<Name>), 
    >,
    mut entity_joint_query: Query<(Entity, &ImpulseJointPrefab, Option<&mut LocalAnchorEntityStorage>, &Parent), Without<ImpulseJoint>>
) {
    for (parent_joint_entity, name, parent_body) in parent_joint_query.iter() {
        let parent_body_entity = parent_body.get();
        for (joint_entity, prefab, mut storage, entity_parent) in entity_joint_query.iter_mut() {
            if !(entity_parent == parent_body && prefab.parent_name == name.as_str()) { continue; }
            LocalAnchorEntityStorage::swawn(storage.as_deref_mut(), &mut commands, prefab, joint_entity, parent_joint_entity);
            if !prefab.enabled { continue };
            let generic_joint = prefab.joint.as_joint(prefab.local_anchor1, prefab.local_anchor2);
            commands.entity(joint_entity)
                .insert(ImpulseJoint::new(parent_joint_entity, generic_joint));
        }
        let Ok(mut map) = map.get_mut(parent_body_entity) else {continue};
        map.0.insert(name.as_str().to_owned(), parent_joint_entity);        
    }
}