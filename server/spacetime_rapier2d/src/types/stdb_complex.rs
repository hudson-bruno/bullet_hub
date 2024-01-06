use rapier2d::{na::Complex, prelude::*};
use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone, Debug)]
pub struct StdbComplex {
    pub re: Real,
    pub im: Real,
}

impl From<Rotation<Real>> for StdbComplex {
    fn from(item: Rotation<Real>) -> Self {
        Self {
            re: item.re,
            im: item.im,
        }
    }
}

impl Into<Rotation<Real>> for StdbComplex {
    fn into(self) -> Rotation<Real> {
        Rotation::from_complex(Complex {
            re: self.re,
            im: self.im,
        })
    }
}
