use amethyst::ecs::world::Index;
use amethyst::ecs::{Component, FlaggedStorage};
use nalgebra::Matrix3;
use nphysics3d::math::{Point, Velocity};
use nphysics3d::object::BodyHandle;
use std::collections::HashMap;

/// Physics body component for describing (currently) rigid body dynamics.
pub enum DynamicBody {
    RigidBody(RigidPhysicsBody),
    Multibody(PhysicsMultibody),
}

impl DynamicBody {
    pub fn new_rigidbody(
        mass: f32,
        angular_mass: Matrix3<f32>,
        center_of_mass: Point<f32>,
    ) -> Self {
        DynamicBody::new_rigidbody_with_velocity(
            Velocity::<f32>::zero(),
            mass,
            angular_mass,
            center_of_mass,
        )
    }

    pub fn new_rigidbody_with_velocity(
        velocity: Velocity<f32>,
        mass: f32,
        angular_mass: Matrix3<f32>,
        center_of_mass: Point<f32>,
    ) -> Self {
        DynamicBody::RigidBody(RigidPhysicsBody {
            handle: None,
            velocity,
            mass,
            angular_mass,
            center_of_mass,
        })
    }

    pub fn handle(&self) -> Option<BodyHandle> {
        match self {
            DynamicBody::RigidBody(x) => x.handle,
            DynamicBody::Multibody(x) => x.handle,
        }
    }
}

impl Component for DynamicBody {
    type Storage = FlaggedStorage<Self>;
}

/// Rigid physics body, for use in `PhysicsBody` Component.
/// Currently only the velocity is read and updated at runtime.
/// The properties of mass are only written at physics body creation time.
pub struct RigidPhysicsBody {
    pub(crate) handle: Option<BodyHandle>,
    pub velocity: Velocity<f32>,

    // TODO: update these in the physics system below.
    pub mass: f32,
    pub angular_mass: Matrix3<f32>,
    pub center_of_mass: Point<f32>,
}

/// Multipart physics body, for use in `PhysicsBody` Component. Not implemented yet.
pub struct PhysicsMultibody {
    pub handle: Option<BodyHandle>,
}

pub type DynamicsBodyRelations = HashMap<Index, BodyHandle>;
