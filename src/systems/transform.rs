use specs;
use specs::Join;

use nalgebra::{Rotation3, Isometry3, Vector3, Matrix4};
use nalgebra::{ToHomogeneous, Eye, Unit, Norm, Inverse};
use nalgebra::to_rotation_matrix;

pub struct System;

impl System {
    pub fn solve_transforms(&self, arg: &specs::RunArg) {
        use ::components::position::Component as Position;
        use ::components::isometry::Component as Isometry;
        use ::components::rotation::Component as Rotation;
        use ::components::quaternion_rotation::Component as QuaternionRotation;
        use ::components::scale::Component as Scale;
        use ::components::transform::Component as Transform;

        //Get entity ids and all the necessary component storage structures
        let (ref positions, ref isometries, ref rotations, ref quat_rotations, ref scales, ref mut transforms, ref entities) = arg.fetch(|world| {
            (
                world.read::<Position>(),
                world.read::<Isometry>(),
                world.read::<Rotation>(),
                world.read::<QuaternionRotation>(),
                world.read::<Scale>(),
                world.write::<Transform>(),
                world.entities(),
            )
        });

        for (mut transform, entity) in (transforms, entities).iter() {
            //Start off with the identity matrix
            transform.matrix = Matrix4::new_identity(4);

            ///TODO: Joint these together
            let mut scale_matrix = Matrix4::new_identity(4);
            let mut rotation_matrix = Matrix4::new_identity(4);
            let mut quat_rotation_matrix = Matrix4::new_identity(4);
            let mut translation_matrix = Matrix4::new_identity(4);
            let mut isometry_matrix = Matrix4::new_identity(4);

            if let Some(scale) = scales.get(entity) {
                scale_matrix.m11 = scale.0.x;
                scale_matrix.m22 = scale.0.y;
                scale_matrix.m33 = scale.0.z;
            }

            if let Some(isometry) = isometries.get(entity) {
                isometry_matrix = isometry.0.to_homogeneous();
            }

            if let Some(quat_rotation) = quat_rotations.get(entity) {
                //Normalize the quaternion to the UnitQuaternion,
                // then convert it to rotation matrix,
                // then convert that to a homogeneous matrix and apply it
                quat_rotation_matrix = to_rotation_matrix(&quat_rotation.unit()).to_homogeneous();
            }

            if let Some(rotation) = rotations.get(entity) {
                rotation_matrix *= rotation.0.to_homogeneous();
            }

            if let Some(position) = positions.get(entity) {
                translation_matrix.m14 = position.0.x;
                translation_matrix.m24 = position.0.y;
                translation_matrix.m34 = position.0.z;
            }

            transform.matrix = isometry_matrix * translation_matrix * rotation_matrix * quat_rotation_matrix * scale_matrix;

            transform.inverse = transform.matrix.inverse();
        }
    }
}

impl specs::System<super::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        self.solve_transforms(&arg);
    }
}