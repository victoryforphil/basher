use rerun::{AsComponents, Points3D, Vec3D};

use crate::types::QuadPose;

pub struct RerunQuadPose {
    pub points_3d: Points3D,
    pub pose: QuadPose,
}

impl RerunQuadPose {
    pub fn new(pose: QuadPose) -> RerunQuadPose {
        let pos = pose.position;
        let points_3d = Points3D::new([Vec3D::new(pos.x as f32, pos.y as f32, pos.z as f32)]);
        RerunQuadPose {
            points_3d,
            pose: pose,
        }
    }
}

impl From<QuadPose> for RerunQuadPose {
    fn from(pose: QuadPose) -> Self {
        RerunQuadPose::new(pose)
    }
}

impl AsComponents for RerunQuadPose {
    fn as_component_batches(&self) -> Vec<rerun::MaybeOwnedComponentBatch<'_>> {
        self.points_3d.as_component_batches()
    }
}
