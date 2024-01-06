use rapier3d::prelude::*;
use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone, Debug)]
pub struct StdbVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vector<Real>> for StdbVector3 {
    fn from(item: Vector<Real>) -> Self {
        Self {
            x: item.x,
            y: item.y,
            z: item.z,
        }
    }
}

impl Into<Vector<Real>> for StdbVector3 {
    fn into(self) -> Vector<Real> {
        Vector::new(self.x, self.y, self.z)
    }
}
