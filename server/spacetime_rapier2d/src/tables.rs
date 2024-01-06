use crate::types::{StdbComplex, StdbVector2};
use spacetimedb::{spacetimedb, Identity};

#[spacetimedb(table)]
#[derive(Debug)]
pub struct DynamicEntity {
    #[primarykey]
    pub owner: Identity,
    #[unique]
    pub rigid_body_id: u64,
    pub position: StdbVector2,
    pub rotation: StdbComplex,
}
