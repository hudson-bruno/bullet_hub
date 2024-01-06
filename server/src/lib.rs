use rapier2d::prelude::*;
use spacetime_rapier2d::state::PhysicsState;
use spacetime_rapier2d::tables::DynamicEntity;
use spacetime_rapier2d::utils::concat_u32_to_u64;
use spacetimedb::{schedule, spacetimedb, ReducerContext, Timestamp};

#[spacetimedb(table)]
pub struct Global {
    #[primarykey]
    pub id: u8,
    pub start: Timestamp,
    pub raw_physics: Vec<u8>,
}

#[spacetimedb(init)]
pub fn init(_ctx: ReducerContext) {
    Global::insert(Global {
        id: 0,
        raw_physics: PhysicsState::new(None).serialize().unwrap(),
        start: Timestamp::now(),
    })
    .expect("Error initializing globals");

    schedule!("0ms", _tick_physics(_));
}

#[spacetimedb(connect)]
pub fn identity_connected(ctx: ReducerContext) {
    handle_physics(|physics_state| {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();

        let body_handle = physics_state.add_physics_entity(rigid_body, collider);

        let rigid_body = physics_state.bodies.get(body_handle).unwrap();
        let dynamic_entity = DynamicEntity {
            owner: ctx.sender,
            position: rigid_body.translation().to_owned().into(),
            rotation: rigid_body.rotation().to_owned().into(),
            rigid_body_id: concat_u32_to_u64(body_handle.0.into_raw_parts()),
        };

        DynamicEntity::insert(dynamic_entity).expect("Error while creating a new DynamicEntity");
    });
}

#[spacetimedb(disconnect)]
pub fn identity_disconnected(_ctx: ReducerContext) {}

#[spacetimedb(reducer, repeat=0ms)]
fn _tick_physics(_ctx: ReducerContext) {
    handle_physics(|physics_state| {
        for (handle, rigid_body) in physics_state.bodies.iter() {
            log::debug!("Rigidbody: {:?}", rigid_body.translation());

            let rigid_body_id = &concat_u32_to_u64(handle.0.into_raw_parts());
            if let Some(dynamic_entity) = DynamicEntity::filter_by_rigid_body_id(rigid_body_id) {
                DynamicEntity::update_by_rigid_body_id(
                    rigid_body_id,
                    DynamicEntity {
                        position: rigid_body.translation().to_owned().into(),
                        rotation: rigid_body.rotation().to_owned().into(),
                        ..dynamic_entity
                    },
                );
            }
        }
    })
}

fn handle_physics(handle: impl Fn(&mut PhysicsState) -> ()) {
    let global = Global::filter_by_id(&0).unwrap();
    let mut physics_state = PhysicsState::derialize(&global.raw_physics[..]).unwrap();

    physics_state.step();

    handle(&mut physics_state);

    Global::update_by_id(
        &0,
        Global {
            raw_physics: physics_state.serialize().unwrap(),
            ..global
        },
    );
}
