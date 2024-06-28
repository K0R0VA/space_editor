use bevy::prelude::*;

use bevy_rapier3d::geometry::Collider;

use crate::PrefabMarkerComponent;

type Vector = Vec3;
type Scalar = f32;


pub fn sync_collider(
    mut query: Query<(&mut Collider, &ColliderPrefab), (Changed<ColliderPrefab>, With<Collider>)>,
) {
    for (mut collider, prefab) in query.iter_mut() {
        *collider = prefab.to_collider();
    }
}

pub fn spawn_collider(
    mut commands: Commands,
    query: Query<(Entity,  &ColliderPrefab), (Added<ColliderPrefab>, Without<Collider>)>,
) {
    for (e, prefab) in query.iter() {
        commands.entity(e)
            .insert(prefab.to_collider());
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Component)]
#[reflect(Component, Default)]
pub enum ColliderPrefab {
    ComplexCollider,
    Cuboid(Vector),
    RoundCuboid {
        border_radius: Scalar,
        shape: Vector
    },
    CapsuleEndpoints {
        a: Vector,
        b: Vector,
        radius: Scalar,
    },
    Cone {
        half_height: Scalar,
        radius: Scalar,
    },
    RoundCone {
        border_radius: Scalar,
        half_height: Scalar,
        radius: Scalar,
    },
    Cylinder {
        half_height: Scalar,
        radius: Scalar,
    },
    RoundCylinder {
        border_radius: Scalar,
        half_height: Scalar,
        radius: Scalar,
    },
    Halfspace {
        outward_normal: Vector,
    },
    Triangle {
        a: Vector,
        b: Vector,
        c: Vector,
    },
    RoundTriangle {
        border_radius: Scalar,
        a: Vector,
        b: Vector,
        c: Vector,
    },
    Ball(Scalar),
    Segment {
        a: Vector,
        b: Vector,
    },
}

impl PrefabMarkerComponent for ColliderPrefab {
    type Component = Collider;
}

impl Default for ColliderPrefab {
    fn default() -> Self {
        Self::Cuboid(Vector::new(1.0, 1.0, 1.0))
    }
}

impl ColliderPrefab {
    pub fn to_collider(&self) -> Collider {
        match self {
            Self::Cuboid(bbox) => Collider::cuboid(bbox.x, bbox.y, bbox.z),
            Self::CapsuleEndpoints { a, b, radius } => Collider::capsule(*a, *b, *radius),
            Self::Cone { half_height: height, radius } => Collider::cone(*height, *radius),
            Self::Cylinder { half_height: height, radius } => Collider::cylinder(*height, *radius),
            Self::Halfspace { outward_normal } => Collider::halfspace(*outward_normal).unwrap_or_default(),
            Self::Triangle { a, b, c } => Collider::triangle(*a, *b, *c),
            Self::Ball(radius) => Collider::ball(*radius),
            Self::Segment { a, b } => Collider::segment(*a, *b),
            Self::RoundCuboid { border_radius, shape } => Collider::round_cuboid(shape.x, shape.y, shape.z, *border_radius),
            Self::RoundCone { border_radius, half_height, radius } => Collider::round_cone(*half_height, *radius, *border_radius),
            Self::RoundCylinder { border_radius, half_height, radius } => Collider::round_cylinder(*half_height, *radius, *border_radius),
            Self::RoundTriangle { border_radius, a, b, c } => Collider::round_triangle(*a, *b, *c, *border_radius),
            Self::ComplexCollider => Collider::default(),
        }
    }
}
