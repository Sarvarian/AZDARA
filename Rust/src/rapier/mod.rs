use rapier2d::{
    crossbeam::{self, channel::Receiver},
    dynamics::{CCDSolver, IntegrationParameters, JointSet, RigidBodySet},
    geometry::{BroadPhase, ColliderSet, ContactEvent, IntersectionEvent, NarrowPhase},
    math::Real,
    na,
    pipeline::{ChannelEventCollector, PhysicsPipeline},
};

pub struct RapierPhysics {
    pipeline: PhysicsPipeline,
    gravity: na::Vector2<Real>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: ChannelEventCollector,
    contact_recv: Receiver<ContactEvent>,
    intersection_recv: Receiver<IntersectionEvent>,
}

impl RapierPhysics {
    pub fn new() -> Self {
        // Initialize the event collector.
        let (contact_send, contact_recv) = crossbeam::channel::unbounded();
        let (intersection_send, intersection_recv) = crossbeam::channel::unbounded();
        RapierPhysics {
            pipeline: PhysicsPipeline::new(),
            gravity: na::Vector2::identity(),
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            joints: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: ChannelEventCollector::new(intersection_send, contact_send),
            contact_recv,
            intersection_recv,
        }
    }

    pub fn step(&mut self) {
        let physics_hooks = ();
        let event_handler = ();
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        while let Ok(contact_event) = self.contact_recv.try_recv() {
            // Handle the contact event.
        }

        while let Ok(intersection_event) = self.intersection_recv.try_recv() {
            // Handle the intersection event.
        }
    }

    pub fn set_delta(&mut self, delta: Real) {
        self.integration_parameters.dt = delta;
    }
}
