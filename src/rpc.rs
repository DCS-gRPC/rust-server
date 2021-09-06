use std::pin::Pin;

use crate::shutdown::{AbortableStream, ShutdownHandle};
use dcs::atmosphere_server::Atmosphere;
use dcs::coalitions_server::Coalitions;
use dcs::controllers_server::Controllers;
use dcs::custom_server::Custom;
use dcs::groups_server::Groups;
use dcs::hook_server::Hook;
use dcs::mission_server::Mission;
use dcs::timer_server::Timer;
use dcs::triggers_server::Triggers;
use dcs::units_server::Units;
use dcs::world_server::World;
use dcs::*;
use dcs_module_ipc::IPC;
use futures_util::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub mod dcs {
    tonic::include_proto!("dcs");

    pub mod group {
        tonic::include_proto!("dcs.group");
    }

    pub mod hook {
        tonic::include_proto!("dcs.hook");
    }
}

#[derive(Clone)]
pub struct MissionRpc {
    ipc: IPC<Event>,
    shutdown_signal: ShutdownHandle,
}

#[derive(Clone)]
pub struct HookRpc {
    ipc: IPC<()>,
    shutdown_signal: ShutdownHandle,
}

impl MissionRpc {
    pub fn new(ipc: IPC<Event>, shutdown_signal: ShutdownHandle) -> Self {
        MissionRpc {
            ipc,
            shutdown_signal,
        }
    }

    pub async fn request<I, O>(&self, method: &str, request: Request<I>) -> Result<O, Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
        for<'de> O: serde::Deserialize<'de> + Send + Sync + std::fmt::Debug + 'static,
    {
        self.ipc
            .request(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }

    pub async fn notification<I>(&self, method: &str, request: Request<I>) -> Result<(), Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
    {
        self.ipc
            .notification(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }

    pub async fn events(&self) -> impl Stream<Item = Event> {
        self.ipc.events().await
    }
}

impl HookRpc {
    pub fn new(ipc: IPC<()>, shutdown_signal: ShutdownHandle) -> Self {
        HookRpc {
            ipc,
            shutdown_signal,
        }
    }

    pub async fn request<I, O>(&self, method: &str, request: Request<I>) -> Result<O, Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
        for<'de> O: serde::Deserialize<'de> + Send + Sync + std::fmt::Debug + 'static,
    {
        self.ipc
            .request(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }

    pub async fn notification<I>(&self, method: &str, request: Request<I>) -> Result<(), Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
    {
        self.ipc
            .notification(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }
}

#[tonic::async_trait]
impl Mission for MissionRpc {
    type StreamEventsStream =
        Pin<Box<dyn Stream<Item = Result<Event, tonic::Status>> + Send + Sync + 'static>>;
    type StreamUnitsStream =
        Pin<Box<dyn Stream<Item = Result<UnitUpdate, tonic::Status>> + Send + Sync + 'static>>;

    async fn stream_events(
        &self,
        _request: Request<StreamEventsRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        let events = self.events().await;
        let stream = AbortableStream::new(self.shutdown_signal.signal(), events.map(Ok));
        Ok(Response::new(Box::pin(stream)))
    }

    async fn stream_units(
        &self,
        request: Request<StreamUnitsRequest>,
    ) -> Result<Response<Self::StreamUnitsStream>, Status> {
        let rpc = self.clone();
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            if let Err(crate::stream::Error::Status(err)) =
                crate::stream::stream_units(request.into_inner(), rpc, tx.clone()).await
            {
                // ignore error, as we don't care at this point whether the channel is closed or not
                let _ = tx.send(Err(err)).await;
            }
        });

        let stream = AbortableStream::new(
            self.shutdown_signal.signal(),
            ReceiverStream::new(rx).map(|result| {
                result.map(|update| UnitUpdate {
                    update: Some(update),
                })
            }),
        );
        Ok(Response::new(Box::pin(stream)))
    }
}

#[tonic::async_trait]
impl Timer for MissionRpc {
    async fn get_time(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<GetTimeResponse>, Status> {
        let res: GetTimeResponse = self.request("getTime", request).await?;
        Ok(Response::new(res))
    }

    async fn get_absolute_time(
        &self,
        request: Request<GetAbsoluteTimeRequest>,
    ) -> Result<Response<GetAbsoluteTimeResponse>, Status> {
        let res: GetAbsoluteTimeResponse = self.request("getAbsoluteTime", request).await?;
        Ok(Response::new(res))
    }

    async fn get_time_zero(
        &self,
        request: Request<GetTimeZeroRequest>,
    ) -> Result<Response<GetTimeZeroResponse>, Status> {
        let res: GetTimeZeroResponse = self.request("getTimeZero", request).await?;
        Ok(Response::new(res))
    }
}

#[tonic::async_trait]
impl Triggers for MissionRpc {
    async fn out_text(
        &self,
        request: Request<OutTextRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("outText", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn out_text_for_coalition(
        &self,
        request: Request<OutTextForCoalitionRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("outTextForCoalition", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn out_text_for_group(
        &self,
        request: Request<OutTextForGroupRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("outTextForGroup", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn get_user_flag(
        &self,
        request: Request<GetUserFlagRequest>,
    ) -> Result<Response<GetUserFlagResponse>, Status> {
        let res: GetUserFlagResponse = self.request("getUserFlag", request).await?;
        Ok(Response::new(res))
    }

    async fn set_user_flag(
        &self,
        request: Request<SetUserFlagRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("setUserFlag", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn mark_to_all(
        &self,
        request: Request<MarkToAllRequest>,
    ) -> Result<Response<MarkToAllResponse>, Status> {
        let res: MarkToAllResponse = self.request("markToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_coalition(
        &self,
        request: Request<MarkToCoalitionRequest>,
    ) -> Result<Response<MarkToCoalitionResponse>, Status> {
        let res: MarkToCoalitionResponse = self.request("markToCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_group(
        &self,
        request: Request<MarkToGroupRequest>,
    ) -> Result<Response<MarkToGroupResponse>, Status> {
        let res: MarkToGroupResponse = self.request("markToGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn remove_mark(
        &self,
        request: Request<RemoveMarkRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("removeMark", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn explosion(
        &self,
        request: Request<ExplosionRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("explosion", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn smoke(
        &self,
        request: Request<SmokeRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("smoke", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn illumination_bomb(
        &self,
        request: Request<IlluminationBombRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("illuminationBomb", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn signal_flare(
        &self,
        request: Request<SignalFlareRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("signalFlare", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }
}

#[tonic::async_trait]
impl Atmosphere for MissionRpc {
    async fn get_wind(
        &self,
        request: Request<AtmosphereRequest>,
    ) -> Result<Response<GetWindResponse>, Status> {
        let res: GetWindResponse = self.request("getWind", request).await?;
        Ok(Response::new(res))
    }

    async fn get_wind_with_turbulence(
        &self,
        request: Request<AtmosphereRequest>,
    ) -> Result<Response<GetWindResponse>, Status> {
        let res: GetWindResponse = self.request("getWindWithTurbulence", request).await?;
        Ok(Response::new(res))
    }

    async fn get_temperature_and_pressure(
        &self,
        request: Request<AtmosphereRequest>,
    ) -> Result<Response<GetTemperatureAndPressureResponse>, Status> {
        let res: GetTemperatureAndPressureResponse =
            self.request("getTemperatureAndPressure", request).await?;
        Ok(Response::new(res))
    }
}

#[tonic::async_trait]
impl World for MissionRpc {
    async fn get_airbases(
        &self,
        request: Request<GetAirbasesRequest>,
    ) -> Result<Response<GetAirbasesResponse>, Status> {
        let res: GetAirbasesResponse = self.request("getAirbases", request).await?;
        Ok(Response::new(res))
    }

    async fn get_mark_panels(
        &self,
        request: Request<GetMarkPanelsRequest>,
    ) -> Result<Response<GetMarkPanelsResponse>, Status> {
        let res: GetMarkPanelsResponse = self.request("getMarkPanels", request).await?;
        Ok(Response::new(res))
    }
}

#[tonic::async_trait]
impl Coalitions for MissionRpc {
    async fn get_players(
        &self,
        request: Request<GetPlayersRequest>,
    ) -> Result<Response<GetPlayersResponse>, Status> {
        let res: GetPlayersResponse = self.request("getPlayers", request).await?;
        Ok(Response::new(res))
    }

    async fn get_groups(
        &self,
        request: Request<GetGroupsRequest>,
    ) -> Result<Response<GetGroupsResponse>, Status> {
        let res: GetGroupsResponse = self.request("getGroups", request).await?;
        Ok(Response::new(res))
    }
}

#[tonic::async_trait]
impl Controllers for MissionRpc {
    async fn set_alarm_state(
        &self,
        request: Request<SetAlarmStateRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("setAlarmState", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }
}

#[tonic::async_trait]
impl Groups for MissionRpc {
    async fn get_units(
        &self,
        request: Request<group::GetUnitsRequest>,
    ) -> Result<Response<group::GetUnitsResponse>, Status> {
        let res: group::GetUnitsResponse = self.request("getUnits", request).await?;
        Ok(Response::new(res))
    }
}

#[tonic::async_trait]
impl Units for MissionRpc {
    async fn get_radar(
        &self,
        request: Request<GetRadarRequest>,
    ) -> Result<Response<GetRadarResponse>, Status> {
        let res: GetRadarResponse = self.request("getRadar", request).await?;
        Ok(Response::new(res))
    }

    async fn get_position(
        &self,
        request: Request<GetUnitPositionRequest>,
    ) -> Result<Response<GetUnitPositionResponse>, Status> {
        let res: GetUnitPositionResponse = self.request("getUnitPosition", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_name(
        &self,
        request: Request<GetUnitPlayerNameRequest>,
    ) -> Result<Response<GetUnitPlayerNameResponse>, Status> {
        let res: GetUnitPlayerNameResponse = self.request("getUnitPlayerName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_unit_descriptor(
        &self,
        request: Request<GetUnitDescriptorRequest>,
    ) -> Result<Response<GetUnitDescriptorResponse>, Status> {
        let res: GetUnitDescriptorResponse = self.request("getUnitDescriptor", request).await?;
        Ok(Response::new(res))
    }
}

#[tonic::async_trait]
impl Custom for MissionRpc {
    async fn request_mission_assignment(
        &self,
        request: Request<MissionAssignmentRequest>,
    ) -> Result<Response<MissionAssignmentResponse>, Status> {
        self.notification("requestMissionAssignment", request)
            .await?;
        Ok(Response::new(MissionAssignmentResponse {}))
    }

    async fn join_mission(
        &self,
        request: Request<MissionJoinRequest>,
    ) -> Result<Response<MissionJoinResponse>, Status> {
        self.notification("joinMission", request).await?;
        Ok(Response::new(MissionJoinResponse {}))
    }

    async fn eval(&self, request: Request<EvalRequest>) -> Result<Response<EvalResponse>, Status> {
        let json: serde_json::Value = self.request("eval", request).await?;
        let json = serde_json::to_string(&json).map_err(|err| {
            Status::internal(format!("failed to deserialize eval result: {}", err))
        })?;
        Ok(Response::new(EvalResponse { json }))
    }
}

#[tonic::async_trait]
impl Hook for HookRpc {
    async fn get_mission_name(
        &self,
        request: Request<hook::GetMissionNameRequest>,
    ) -> Result<Response<hook::GetMissionNameResponse>, Status> {
        let res: hook::GetMissionNameResponse = self.request("getMissionName", request).await?;
        Ok(Response::new(res))
    }
}

fn to_status(err: dcs_module_ipc::Error) -> Status {
    use dcs_module_ipc::Error;
    match err {
        Error::Script { kind, message } => match kind.as_deref() {
            Some("INVALID_ARGUMENT") => Status::invalid_argument(message),
            Some("NOT_FOUND") => Status::not_found(message),
            Some("ALREADY_EXISTS") => Status::already_exists(message),
            Some("UNIMPLEMENTED") => Status::unimplemented(message),
            _ => Status::internal(message),
        },
        err => Status::internal(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::dcs::{
        event, initiator, Airbase, AirbaseCategory, Coalition, Event, GetAirbasesResponse,
        Initiator, Position, Unit,
    };

    #[test]
    fn test_event_deserialization() {
        let event: Event =
            serde_json::from_str(r#"{"time":4.2,"event":{"type":"missionStart"}}"#).unwrap();
        assert_eq!(
            event,
            Event {
                time: 4.2,
                event: Some(event::Event::MissionStart(event::MissionStartEvent {})),
            }
        );
    }

    #[test]
    fn test_enum_deserialization() {
        let event: Event = serde_json::from_str(
            r#"
                {
	                "time": 4.2,
	                "event": {
		                "type": "markAdd",
		                "initiator": {
                            "initiator": {
                                "unit": {
                                    "id": 1,
                                    "name": "Aerial-1-1",
                                    "callsign": "Enfield11",
                                    "coalition": 2,
                                    "type": "FA-18C_hornet",
                                    "position": {
                                        "lat": 3,
                                        "lon": 2,
                                        "alt": 1
                                    },
                                    "playerName": "New callsign",
                                    "groupName": "Group 1",
                                    "numberInGroup": 1,
                                    "heading": 0.5,
                                    "speed": 0.8
                                }
                            }
		                },
		                "coalition": 2,
		                "id": 42,
		                "pos": {
			                "lat": 1,
			                "lon": 2,
			                "alt": 3
		                },
		                "text": "Test"
	                }
                }
            "#,
        )
        .unwrap();
        assert_eq!(
            event,
            Event {
                time: 4.2,
                event: Some(event::Event::MarkAdd(event::MarkAddEvent {
                    initiator: Some(Initiator {
                        initiator: Some(initiator::Initiator::Unit(Unit {
                            id: 1,
                            name: "Aerial-1-1".to_string(),
                            callsign: "Enfield11".to_string(),
                            r#type: "FA-18C_hornet".to_string(),
                            coalition: Coalition::Blue.into(),
                            position: Some(Position {
                                lat: 3.0,
                                lon: 2.0,
                                alt: 1.0
                            }),
                            player_name: Some("New callsign".to_string()),
                            group_name: "Group 1".to_string(),
                            number_in_group: 1,
                            heading: 0.5,
                            speed: 0.8
                        }))
                    }),
                    visibility: Some(event::mark_add_event::Visibility::Coalition(
                        Coalition::Blue.into()
                    )),
                    id: 42,
                    pos: Some(Position {
                        lat: 1.0,
                        lon: 2.0,
                        alt: 3.0
                    }),
                    text: "Test".to_string(),
                })),
            }
        );
    }

    #[test]
    fn test_optional_field_deserialization() {
        let resp: GetAirbasesResponse = serde_json::from_str(
            r#"

                {
                    "airbases": [
                        {
                            "coalition": 0,
                            "name": "Anapa-Vityazevo",
                            "callsign": "Anapa-Vityazevo",
                            "position": {
                                "lon": 37.35978347755592,
                                "lat": 45.01317473377168,
                                "alt": 43.00004196166992
                            },
                            "category": 0,
                            "displayName": "Anapa-Vityazevo"
                        }
                    ]
                }
            "#,
        )
        .unwrap();
        assert_eq!(
            resp,
            GetAirbasesResponse {
                airbases: vec![Airbase {
                    id: None,
                    name: "Anapa-Vityazevo".to_string(),
                    callsign: "Anapa-Vityazevo".to_string(),
                    coalition: Coalition::Neutral.into(),
                    position: Some(Position {
                        lon: 37.35978347755592,
                        lat: 45.01317473377168,
                        alt: 43.00004196166992
                    }),
                    category: AirbaseCategory::Airdrome.into(),
                    display_name: "Anapa-Vityazevo".to_string(),
                }]
            }
        );
    }
}
