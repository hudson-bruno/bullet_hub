use rapier2d::prelude::*;
use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone, Debug)]
pub struct StdbVector2 {
    pub x: f32,
    pub y: f32,
}

impl From<Vector<Real>> for StdbVector2 {
    fn from(item: Vector<Real>) -> Self {
        Self {
            x: item.x,
            y: item.y,
        }
    }
}

impl Into<Vector<Real>> for StdbVector2 {
    fn into(self) -> Vector<Real> {
        Vector::new(self.x, self.y)
    }
}
