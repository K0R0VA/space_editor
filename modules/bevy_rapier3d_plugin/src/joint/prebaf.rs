
use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use bevy::prelude::*;
use bevy_rapier3d::dynamics::{ImpulseJoint, Sleeping};

use crate::{joint::local_anchor_marker::LocalAnchorEntityStorage, ChangedOrAdded, PrefabMarkerComponent};
use super::{generic_joint::GenericJointPrefab, name_map::JointBodyBuilderHashMap};


#[derive(Reflect, Debug, Clone, Component, Default)]
#[reflect(Component, Default)]
pub struct ImpulseJointPrefab {
    pub parent_name: String,
    pub enabled: bool,
    #[reflect(ignore)]
    pub previus_parent: u64,
    pub joint: GenericJointPrefab,
    pub local_anchor1: Vec3,
    pub local_anchor2: Vec3,
}

impl PrefabMarkerComponent for ImpulseJointPrefab {
    type Component = ImpulseJoint;
}

impl ImpulseJointPrefab {
    fn update_joint(&mut self, impulse_joint: &mut ImpulseJoint, parent_entity: Entity) {
        let new_generic_joint = self.joint.as_joint(self.local_anchor1, self.local_anchor2);
        if self.should_respawn(&self.parent_name) {
            info!("create new impulse");
            *impulse_joint = ImpulseJoint::new(parent_entity, new_generic_joint);
            return;
        }
        impulse_joint.data = new_generic_joint;
    }
    fn should_respawn(&self, parent_entity: &str) -> bool {
        let Self {previus_parent, ..} = self;
        *previus_parent != 0 && to_hash(parent_entity) != *previus_parent
    }
}

pub fn sync_joint(
    mut commands: Commands,
    mut map: Query<&JointBodyBuilderHashMap>,
    mut query: Query<
        (Entity, &mut ImpulseJoint, Option<&mut LocalAnchorEntityStorage>, &mut ImpulseJointPrefab, &Parent, Option<&mut Sleeping>), 
        (Changed<ImpulseJointPrefab>, With<ImpulseJoint>)
    >,
) {
    for (entity, mut joint, storage, mut prefab, parent, sleeping) in query.iter_mut() {
        let parent_entity = parent.get();
        let Ok(map) = map.get_mut(parent_entity) else {continue;};
        let Some(&parent_joint_entity) = map.0.get(&prefab.parent_name) else {
            warn!("Parent entity not found by {:?} with global parent {:?}", prefab.parent_name, parent_entity);
            continue 
        };
        if prefab.should_respawn(&prefab.parent_name) {
            if let Some(mut storage) = storage {
                storage.change_parent(&mut commands, parent_joint_entity, prefab.local_anchor1);
            }
        }
        prefab.previus_parent = to_hash(&prefab.parent_name);
        if !prefab.enabled { 
            commands.entity(entity).remove::<ImpulseJoint>();
            continue 
        };
        prefab.update_joint(&mut joint, parent_joint_entity);
        if let Some(mut sleeping) = sleeping {
            sleeping.sleeping = false;
        }
    }
}

pub fn spawn_joint(
    mut commands: Commands,
    map: Query<&JointBodyBuilderHashMap>,
    mut query: Query
        <
            (Entity, &ImpulseJointPrefab, Option<&mut LocalAnchorEntityStorage>, &Parent), 
            (
                ChangedOrAdded<ImpulseJointPrefab>, 
                Without<ImpulseJoint>
            )
        >,
) {
    
    for (entity, prefab, mut storage, parent) in query.iter_mut() {
        if !prefab.enabled { continue };
        let parent_entity = parent.get();
        let Some(map) = map.get(parent_entity).ok() else {continue;};
        let Some(&parent_entity) = map.0.get(&prefab.parent_name) else {
            warn!("Can't spawn joint; Parent entity not found by {:?} with global parent {:?}", prefab.parent_name, parent_entity);
            continue
        };
        LocalAnchorEntityStorage::swawn(storage.as_deref_mut(), &mut commands, prefab, entity, parent_entity);
        let generic_joint = prefab.joint.as_joint(prefab.local_anchor1, prefab.local_anchor2);
        commands.entity(entity)
            .insert(ImpulseJoint::new(parent_entity, generic_joint));
    }
}

fn to_hash(str: &str) -> u64 {
    let mut hasher = DefaultHasher::default();
    str.hash(&mut hasher);
    hasher.finish()
}