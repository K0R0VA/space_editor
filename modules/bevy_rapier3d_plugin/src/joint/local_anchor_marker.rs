use bevy::prelude::*;

use super::prebaf::ImpulseJointPrefab;

#[derive(Component)]
pub struct LocalAnchor1Marker {
    pub(crate) entity: Entity,
}

pub fn sync_local_anchor1(
    mut joint: Query<&mut ImpulseJointPrefab>,
    query: Query<(&Transform, &LocalAnchor1Marker), (Changed<Transform>, With<LocalAnchor1Marker>)>,
) {
    for (&transform, &LocalAnchor1Marker {entity}) in query.iter() {
        let Ok(mut joint) = joint.get_mut(entity) else {continue};
        joint.local_anchor1 = transform.translation;
    }   
}

#[derive(Component)]
pub struct LocalAnchor2Marker {
    pub(crate) entity: Entity,
}

pub fn sync_local_anchor2(
    mut joint: Query<&mut ImpulseJointPrefab>,
    query: Query<(&Transform, &LocalAnchor2Marker), (Changed<Transform>, With<LocalAnchor2Marker>)>,
) {
    for (&transform, &LocalAnchor2Marker {entity}) in query.iter() {
        let Ok(mut joint) = joint.get_mut(entity) else {continue};
        joint.local_anchor2 = transform.translation;
    }   
}

#[derive(Component, Debug, Reflect)]
pub struct LocalAnchorEntityStorage {
    pub(crate) anchor_2: Entity,
    pub(crate) anchor_1: Entity
}

impl Default for LocalAnchorEntityStorage {
    fn default() -> Self {
        LocalAnchorEntityStorage {
            anchor_2: Entity::PLACEHOLDER,
            anchor_1: Entity::PLACEHOLDER
        }
    }
}

impl LocalAnchorEntityStorage {
    pub fn change_parent(&mut self, commands: &mut Commands, new_parent: Entity, local_anchor: Vec3) {
        commands.get_entity(self.anchor_1).map(|mut e| e.despawn());
        self.anchor_1 = commands.spawn((
            Name::new("LocalAnchor1Marker"),
            LocalAnchor1Marker { entity: self.anchor_2 },
            Transform::from_translation(local_anchor)
        ))
        .id();
        commands.entity(new_parent).add_child(self.anchor_1);
    }

    pub fn despawn(&self, commands: &mut Commands) {
        commands.get_entity(self.anchor_2).map(|mut e| e.despawn());
        commands.get_entity(self.anchor_1).map(|mut e| e.despawn());
    }

    pub fn swawn(this: Option<&mut Self>, commands: &mut Commands, prefab: &ImpulseJointPrefab, entity: Entity, parent_entity: Entity) {
        let mut new_storage = Self::default();

        new_storage.anchor_2 = commands.spawn((
            Name::new("LocalAnchor2Marker"),
            LocalAnchor2Marker { entity },
            Transform::from_translation(prefab.local_anchor2),
        )).id();

        new_storage.anchor_1 = commands.spawn((
            Name::new("LocalAnchor1Marker"),
            LocalAnchor1Marker { entity },
            Transform::from_translation(prefab.local_anchor1)
        )).id();

        let Some(this) = this else {
            commands.entity(parent_entity).add_child(new_storage.anchor_1);
            commands.entity(entity).add_child(new_storage.anchor_2).insert(new_storage);
            return;
        };
        this.despawn(commands);
        *this = new_storage;
        commands.entity(parent_entity).add_child(this.anchor_1);
        commands.entity(entity).add_child(this.anchor_2);
    }
}
