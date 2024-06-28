use bevy::prelude::*;
use bevy_rapier3d::dynamics::{FixedJointBuilder, GenericJoint, GenericJointBuilder, JointAxesMask, JointAxis, MotorModel, PrismaticJointBuilder, RevoluteJointBuilder, RopeJointBuilder, SphericalJointBuilder, SpringJointBuilder};

#[derive(Reflect, Debug, Clone, Component)]
#[reflect(Component, Default)]
pub enum GenericJointPrefab {
    FixedJoint {
        settings: Vec<BasicSettings>,
    },
    PrismaticJoint {
        local_axis1: Axis,
        motor_settings: List<AdvancedSettings>,
        basic_settings: List<BasicSettings>
    },
    RevoluteJoint {
        axis: Axis,
        settings: List<AdvancedSettings>
    },
    SphericalJoint(List<SphericalSettings>),
    RopeJoint {
        max_distance: f32,
        settings: List<AdvancedSettings>
    },
    SpringJoint {
        rest_length: f32, 
        stiffness: f32, 
        damping: f32,
        contacts_enabled: bool,
        motor_model: MotorModelPrebuf,
    },
    GenericJoint {
        locked_axes: List<JointAxisPrefab>,
        advanced_settings: List<SphericalSettings>,
        basis_settings: List<BasicSettings>,
    }
}


#[derive(Reflect, Debug, Clone, Component, Default)]
#[reflect(Component, Default)]
pub struct List<T: Reflect + Component + Default>(pub Vec<T>);

impl GenericJointPrefab {
    pub fn as_joint(&self, local_anchor1: Vec3, local_anchor2: Vec3) -> GenericJoint {
        match self {
            GenericJointPrefab::FixedJoint { settings,  } => {
                let mut builder = FixedJointBuilder::new()
                    .local_anchor1(local_anchor1)
                    .local_anchor2(local_anchor2);
                for &setting in settings {
                    match setting {
                        BasicSettings::LocalBasic1(quat) => {
                            builder = builder.local_basis1(quat);
                        }
                        BasicSettings::LocalBasic2(quat) => {
                            builder = builder.local_basis1(quat);
                        }
                        BasicSettings::NotSet | _ => {},
                    }
                }
                builder.into()
            },
            GenericJointPrefab::PrismaticJoint { 
                local_axis1, 
                basic_settings: List (basic_settings),
                motor_settings: List(settings) 
            } => {
                let mut builder = PrismaticJointBuilder::new((*local_axis1).into())
                    .local_anchor1(local_anchor1)
                    .local_anchor2(local_anchor2);
                for &setting in settings {
                    match setting {
                        AdvancedSettings::Limits(LimitsPrebuf {min, max})  => {
                            builder = builder.limits([min, max]);
                        }
                        AdvancedSettings::MotorModel(model) => {
                            builder = builder.motor_model( model.into());
                        },
                        AdvancedSettings::MotorVelocity(MotorVelocityPrebuf {factor, target_vel}) => {
                            builder = builder.motor_velocity( factor, target_vel);
                        },
                        AdvancedSettings::MotorPosition(MotorPositionPrebuf {damping, stiffness, target_pos}) => {
                            builder = builder.motor_position( target_pos, stiffness, damping);
                        },
                        AdvancedSettings::MotorMaxForce(max_force) => {
                            builder = builder.motor_max_force( max_force);
                        },
                        AdvancedSettings::NotSet | _ => {},
                    }
                }
                for &setting in basic_settings {
                    match setting {
                        BasicSettings::LocalAxis2(axis)  => {
                            builder = builder.local_axis2(axis.into());
                        }
                        BasicSettings::NotSet | _ => {},
                    }
                }
                builder.into()
            },
            GenericJointPrefab::RevoluteJoint { axis, settings: List(settings) } => {
                let mut builder = RevoluteJointBuilder::new((*axis).into())
                    .local_anchor1(local_anchor1)
                    .local_anchor2(local_anchor2);
                for &setting in settings {
                    match setting {
                        AdvancedSettings::Limits(LimitsPrebuf {min, max})  => {
                            builder = builder.limits([min, max]);
                        }
                        AdvancedSettings::MotorModel(model) => {
                            builder = builder.motor_model( model.into());
                        },
                        AdvancedSettings::MotorVelocity(MotorVelocityPrebuf {factor, target_vel}) => {
                            builder = builder.motor_velocity( factor, target_vel);
                        },
                        AdvancedSettings::MotorPosition(MotorPositionPrebuf {damping, stiffness, target_pos}) => {
                            builder = builder.motor_position( target_pos, stiffness, damping);
                        },
                        AdvancedSettings::Motor(MotorPrebuf {target_pos, target_vel, stiffness, damping}) => {
                            builder = builder.motor( target_pos, target_vel, stiffness, damping);
                        },
                        AdvancedSettings::MotorMaxForce(max_force) => {
                            builder = builder.motor_max_force( max_force);
                        },
                        AdvancedSettings::NotSet => {},
                    }
                }
                builder.into()
            },
            GenericJointPrefab::SphericalJoint (List(settings)) => {
                let mut builder = SphericalJointBuilder::new()
                    .local_anchor1(local_anchor1)
                    .local_anchor2(local_anchor2);
                for setting in settings {
                    let axis = setting.axis.into();
                    match setting.setting {
                        AdvancedSettings::Limits(LimitsPrebuf {min, max})  => {
                            builder = builder.limits(axis, [min, max]);
                        }
                        AdvancedSettings::MotorModel(model) => {
                            builder = builder.motor_model(axis, model.into());
                        },
                        AdvancedSettings::MotorVelocity(MotorVelocityPrebuf {factor, target_vel}) => {
                            builder = builder.motor_velocity(axis, factor, target_vel);
                        },
                        AdvancedSettings::MotorPosition(MotorPositionPrebuf {damping, stiffness, target_pos}) => {
                            builder = builder.motor_position(axis, target_pos, stiffness, damping);
                        },
                        AdvancedSettings::Motor(MotorPrebuf {target_pos, target_vel, stiffness, damping}) => {
                            builder = builder.motor(axis, target_pos, target_vel, stiffness, damping);
                        },
                        AdvancedSettings::MotorMaxForce(max_force) => {
                            builder = builder.motor_max_force(axis, max_force);
                        },
                        AdvancedSettings::NotSet => {},
                    }
                }
                builder.into()
            },
            GenericJointPrefab::RopeJoint { max_distance,  settings: List(settings) } => {
                let mut builder = RopeJointBuilder::new(*max_distance)
                    .local_anchor1(local_anchor1)
                    .local_anchor2(local_anchor2);
                for &setting in settings {
                    match setting {
                        AdvancedSettings::MotorModel(model) => {
                            builder = builder.motor_model(model.into());
                        },
                        AdvancedSettings::MotorVelocity(MotorVelocityPrebuf {factor, target_vel}) => {
                            builder = builder.motor_velocity( factor, target_vel);
                        },
                        AdvancedSettings::MotorPosition(MotorPositionPrebuf {damping, stiffness, target_pos}) => {
                            builder = builder.motor_position( target_pos, stiffness, damping);
                        },
                        AdvancedSettings::MotorMaxForce(max_force) => {
                            builder = builder.motor_max_force( max_force);
                        },
                        AdvancedSettings::NotSet | _ => {},
                    }
                }
                builder.into()
            },
            GenericJointPrefab::SpringJoint { rest_length, stiffness, damping, contacts_enabled, motor_model   } => 
                SpringJointBuilder::new(*rest_length, *stiffness, *damping)
                    .local_anchor1(local_anchor1)
                    .local_anchor2(local_anchor2)
                    .spring_model((*motor_model).into())
                    .contacts_enabled(*contacts_enabled)
                    .into(),
            GenericJointPrefab::GenericJoint { 
                locked_axes, 
                advanced_settings: List(advanced_settings),
                basis_settings: List (basic_settings)
             } => {
                let locked_axes = locked_axes.0.iter()
                    .copied()
                    .map(|axis| axis.into())
                    .reduce(|a, b| a | b)
                    .unwrap_or_default();
                let mut builder = GenericJointBuilder::new(locked_axes)
                    .local_anchor1(local_anchor1)
                    .local_anchor2(local_anchor2);
                for setting in advanced_settings {
                    let axis = setting.axis.into();
                    match setting.setting {
                        AdvancedSettings::Limits(LimitsPrebuf {min, max})  => {
                            builder = builder.limits(axis, [min, max]);
                        }
                        AdvancedSettings::MotorModel(model) => {
                            builder = builder.motor_model(axis, model.into());
                        },
                        AdvancedSettings::MotorVelocity(MotorVelocityPrebuf {factor, target_vel}) => {
                            builder = builder.motor_velocity(axis, factor, target_vel);
                        },
                        AdvancedSettings::MotorPosition(MotorPositionPrebuf {damping, stiffness, target_pos}) => {
                            builder = builder.motor_position(axis, target_pos, stiffness, damping);
                        },
                        AdvancedSettings::Motor(MotorPrebuf {target_pos, target_vel, stiffness, damping}) => {
                            builder = builder.set_motor(axis, target_pos, target_vel, stiffness, damping);
                        },
                        AdvancedSettings::MotorMaxForce(max_force) => {
                            builder = builder.motor_max_force(axis, max_force);
                        },
                        AdvancedSettings::NotSet => {},
                    }
                }
                for &setting in basic_settings {
                    match setting {
                        BasicSettings::LocalAxis1(axis) => {
                            builder = builder.local_axis1(axis.into());
                        },
                        BasicSettings::LocalAxis2(axis) => {
                            builder = builder.local_axis2(axis.into());
                        },
                        BasicSettings::LocalBasic1(quat) => {
                            builder = builder.local_basis1(quat);
                        },
                        BasicSettings::LocalBasic2(quat) => {
                            builder = builder.local_basis2(quat);
                        },
                        BasicSettings::NotSet  => {},
                    }
                }
                builder.into()
            }
        }
    }
}


#[derive(Copy, Clone, Debug, Component, Reflect, Default)]
#[reflect(Component, Default)]
pub enum Axis {
    #[default]
    X,
    Y, 
    Z,
    NegY,
    NegX,
    NegZ,
}

impl Into<Vec3> for Axis {
    fn into(self) -> Vec3 {
        match self {
            Self::X => Vec3::X,
            Self::Y => Vec3::Y,
            Self::Z => Vec3::Z,
            Self::NegX => - Vec3::X,
            Self::NegY => - Vec3::Y,
            Self::NegZ => - Vec3::Z
        }
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct LimitsPrebuf {
    pub max: f32,
    pub min: f32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Component, Reflect, Default)]
#[reflect(Component, Default)]
pub enum MotorModelPrebuf {
    AccelerationBased,
    #[default]
    ForceBased,
}

impl Into<MotorModel> for MotorModelPrebuf {
    fn into(self) -> MotorModel {
        match self {
            MotorModelPrebuf::AccelerationBased => MotorModel::AccelerationBased,
            MotorModelPrebuf::ForceBased => MotorModel::ForceBased,
        }
    }
}


#[derive(Copy, Clone, Debug,  Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct SphericalSettings {
    pub axis: JointAxisPrefab,
    pub setting: AdvancedSettings
}

#[derive(Copy, Clone, Debug,  Component, Reflect, Default)]
#[reflect(Component, Default)]
pub enum JointAxisPrefab {
    #[default]
    X = 0,
    Y,
    Z,
    AngX,
    AngY,
    AngZ,
}

impl Into<JointAxis> for JointAxisPrefab {
    fn into(self) -> JointAxis {
        match self {
            JointAxisPrefab::X => JointAxis::X,
            JointAxisPrefab::Y => JointAxis::Y,
            JointAxisPrefab::Z => JointAxis::Z,
            JointAxisPrefab::AngX => JointAxis::AngX,
            JointAxisPrefab::AngY => JointAxis::AngY,
            JointAxisPrefab::AngZ => JointAxis::AngZ,
        }
    }
}

impl Into<JointAxesMask> for JointAxisPrefab {
    fn into(self) -> JointAxesMask {
        match self {
            JointAxisPrefab::X => JointAxesMask::X,
            JointAxisPrefab::Y => JointAxesMask::Y,
            JointAxisPrefab::Z => JointAxesMask::Z,
            JointAxisPrefab::AngX => JointAxesMask::ANG_X,
            JointAxisPrefab::AngY => JointAxesMask::ANG_Y,
            JointAxisPrefab::AngZ => JointAxesMask::ANG_Z,
        }
    }
}


#[derive(Copy, Clone, Debug,  Component, Reflect, Default)]
#[reflect(Component, Default)]
pub enum AdvancedSettings {
    #[default]
    NotSet,
    MotorModel(MotorModelPrebuf),
    MotorVelocity(MotorVelocityPrebuf),
    MotorPosition(MotorPositionPrebuf),
    Motor(MotorPrebuf),
    MotorMaxForce(f32),
    Limits(LimitsPrebuf)
}

#[derive(Copy, Clone, Debug,  Component, Reflect, Default)]
#[reflect(Component, Default)]
pub enum BasicSettings {
    #[default]
    NotSet,
    LocalAxis2(Axis),
    LocalAxis1(Axis),
    LocalBasic1(Quat),
    LocalBasic2(Quat),
}

#[derive(Copy, Clone, Debug,  Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct MotorVelocityPrebuf {
    pub target_vel: f32,
    pub factor: f32,
}

#[derive(Copy, Clone, Debug,  Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct MotorPositionPrebuf {
    pub target_pos: f32,
    pub stiffness: f32,
    pub damping: f32,
}

#[derive(Copy, Clone, Debug,  Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct MotorPrebuf {
    pub target_vel: f32,
    pub target_pos: f32,
    pub stiffness: f32,
    pub damping: f32,
}



impl Default for GenericJointPrefab {
    fn default() -> Self {
        GenericJointPrefab::FixedJoint { 
            settings: Vec::default()
        }
    }
}