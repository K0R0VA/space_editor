// Remove after update to newer rust version
#![allow(clippy::type_complexity)]
use bevy::{ecs::system::EntityCommands, prelude::*};

pub type ChangedOrAdded<T> = Or<(Changed<T>, Added<T>)>;

pub mod collider;
pub mod joint;
pub mod registry;
pub mod geometry;
// pub mod spatial_query;

/// Community module containing bevy_xpbd_3d plugin
pub struct RapierPlugin;

impl Plugin for RapierPlugin {
    fn build(&self, app: &mut App) {
        {
            info!("Add rapier plugin to editor");
            app.add_plugins(registry::BevyRapierPlugin);
        }
    }
}

pub trait PrefabMarkerComponent: Component {
    type Component: Component;
    fn remove_component(entity: &mut EntityCommands) {
        entity.remove::<Self::Component>();
    }
}

pub mod prelude {
    pub use crate::collider::*;
    pub use crate::registry::*;
    // pub use crate::spatial_query::*;
    pub use crate::RapierPlugin;
}
