use crate::rpc::{HookRpc, MissionRpc};
use stubs::atmosphere::v0::atmosphere_service_server::AtmosphereServiceServer;
use stubs::coalition::v0::coalition_service_server::CoalitionServiceServer;
use stubs::controller::v0::controller_service_server::ControllerServiceServer;
use stubs::custom::v0::custom_service_server::CustomServiceServer;
use stubs::group::v0::group_service_server::GroupServiceServer;
use stubs::hook::v0::hook_service_server::HookServiceServer;
use stubs::mission::v0::mission_service_server::MissionServiceServer;
use stubs::net::v0::net_service_server::NetServiceServer;
use stubs::timer::v0::timer_service_server::TimerServiceServer;
use stubs::trigger::v0::trigger_service_server::TriggerServiceServer;
use stubs::unit::v0::unit_service_server::UnitServiceServer;
use stubs::world::v0::world_service_server::WorldServiceServer;
use tonic::body::BoxBody;
use tonic::codegen::{http, Never, Service};
use tonic::transport::{self, NamedService};

/// The gRPC server is usually constructed via:
/// ```rust
/// transport::Server::builder()
///     .add_service(AtmosphereServiceServer::new(mission_rpc.clone()))
///     .add_service(CoalitionServiceServer::new(mission_rpc.clone()))
///     // ...
/// ```
///
/// However, this leads to an exponential increase in compile time (supposetly due to the deeply
/// nesting of generics). Until this improves in future Rust releases, we are building our own
/// service wrapper that is not based on generics.
///
/// This brings the re-compile time down from 5min to 5sec.
#[derive(Clone)]
pub struct DcsServices {
    atmosphere: AtmosphereServiceServer<MissionRpc>,
    coalition: CoalitionServiceServer<MissionRpc>,
    controller: ControllerServiceServer<MissionRpc>,
    custom: CustomServiceServer<MissionRpc>,
    group: GroupServiceServer<MissionRpc>,
    hook: HookServiceServer<HookRpc>,
    mission: MissionServiceServer<MissionRpc>,
    net: NetServiceServer<MissionRpc>,
    timer: TimerServiceServer<MissionRpc>,
    trigger: TriggerServiceServer<MissionRpc>,
    unit: UnitServiceServer<MissionRpc>,
    world: WorldServiceServer<MissionRpc>,
}

impl DcsServices {
    pub fn new(mission_rpc: MissionRpc, hook_rpc: HookRpc) -> Self {
        Self {
            atmosphere: AtmosphereServiceServer::new(mission_rpc.clone()),
            coalition: CoalitionServiceServer::new(mission_rpc.clone()),
            controller: ControllerServiceServer::new(mission_rpc.clone()),
            custom: CustomServiceServer::new(mission_rpc.clone()),
            group: GroupServiceServer::new(mission_rpc.clone()),
            hook: HookServiceServer::new(hook_rpc),
            mission: MissionServiceServer::new(mission_rpc.clone()),
            net: NetServiceServer::new(mission_rpc.clone()),
            timer: TimerServiceServer::new(mission_rpc.clone()),
            trigger: TriggerServiceServer::new(mission_rpc.clone()),
            unit: UnitServiceServer::new(mission_rpc.clone()),
            world: WorldServiceServer::new(mission_rpc),
        }
    }
}

impl NamedService for DcsServices {
    const NAME: &'static str = "";
}

impl Service<http::Request<transport::Body>> for DcsServices {
    type Response = http::Response<BoxBody>;
    type Error = Never;
    type Future = tonic::codegen::BoxFuture<Self::Response, Self::Error>;

    #[inline]
    fn poll_ready(
        &mut self,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: tonic::codegen::http::Request<transport::Body>) -> Self::Future {
        let path = req.uri().path().trim_start_matches('/');
        if path.starts_with(AtmosphereServiceServer::<MissionRpc>::NAME) {
            self.atmosphere.call(req)
        } else if path.starts_with(CoalitionServiceServer::<MissionRpc>::NAME) {
            self.coalition.call(req)
        } else if path.starts_with(ControllerServiceServer::<MissionRpc>::NAME) {
            self.controller.call(req)
        } else if path.starts_with(CustomServiceServer::<MissionRpc>::NAME) {
            self.custom.call(req)
        } else if path.starts_with(GroupServiceServer::<MissionRpc>::NAME) {
            self.group.call(req)
        } else if path.starts_with(HookServiceServer::<HookRpc>::NAME) {
            self.hook.call(req)
        } else if path.starts_with(MissionServiceServer::<MissionRpc>::NAME) {
            self.mission.call(req)
        } else if path.starts_with(NetServiceServer::<MissionRpc>::NAME) {
            self.net.call(req)
        } else if path.starts_with(TimerServiceServer::<MissionRpc>::NAME) {
            self.timer.call(req)
        } else if path.starts_with(TriggerServiceServer::<MissionRpc>::NAME) {
            self.trigger.call(req)
        } else if path.starts_with(UnitServiceServer::<MissionRpc>::NAME) {
            self.unit.call(req)
        } else if path.starts_with(WorldServiceServer::<MissionRpc>::NAME) {
            self.world.call(req)
        } else {
            Box::pin(std::future::ready(Ok(http::Response::builder()
                .status(200)
                .header("grpc-status", "12")
                .header("content-type", "application/grpc")
                .body(tonic::codegen::empty_body())
                .unwrap())))
        }
    }
}
