use rapier3d::{na::Quaternion, prelude::*};
use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone, Debug)]
pub struct StdbQuaternion {
    pub w: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

impl From<Rotation<Real>> for StdbQuaternion {
    fn from(item: Rotation<Real>) -> Self {
        Self {
            w: item.w,
            i: item.i,
            j: item.j,
            k: item.k,
        }
    }
}

impl Into<Rotation<Real>> for StdbQuaternion {
    fn into(self) -> Rotation<Real> {
        Rotation::from_quaternion(Quaternion::new(self.w, self.i, self.j, self.k))
    }
}
