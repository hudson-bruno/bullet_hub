use rapier2d::{na::UnitComplex, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PhysicsState {
    pub islands: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub joints: ImpulseJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub integration_parameters: IntegrationParameters,
    pub gravity: Vector<f32>,
}

impl PhysicsState {
    pub fn new(gravity: Option<Vector<Real>>) -> Self {
        let mut state = PhysicsState {
            gravity: gravity.unwrap_or(vector![0.0, -9.8]),
            integration_parameters: IntegrationParameters::default(),
            islands: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joints: ImpulseJointSet::new(),
            ccd_solver: CCDSolver::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            query_pipeline: QueryPipeline::new(),
        };

        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        state.colliders.insert(collider);

        state
    }

    pub fn serialize(&self) -> Result<Vec<u8>, String> {
        bincode::serialize(&self).map_err(|e| format!("{e:?}"))
    }

    pub fn derialize<'a>(data: &'a [u8]) -> Result<Self, String> {
        bincode::deserialize(&data).map_err(|e| format!("{e:?}"))
    }

    //TODO: Handle delta time
    pub fn step(&mut self) {
        let mut physics_pipeline = PhysicsPipeline::new();
        physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut MultibodyJointSet::new(),
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );
    }

    pub fn add_physics_entity(
        &mut self,
        rigid_body: impl Into<RigidBody>,
        collider: impl Into<Collider>,
    ) -> RigidBodyHandle {
        let body_handle = self.bodies.insert(rigid_body);
        self.colliders
            .insert_with_parent(collider, body_handle, &mut self.bodies);

        body_handle
    }
}
