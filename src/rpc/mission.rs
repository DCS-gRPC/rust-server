use std::pin::Pin;

use super::MissionRpc;
use crate::shutdown::AbortableStream;
use futures_util::{Stream, StreamExt};
use stubs::mission::v0::mission_service_server::MissionService;
use stubs::timer::v0::timer_service_server::TimerService;
use stubs::trigger::v0::trigger_service_server::TriggerService;
use stubs::*;
use time::format_description::well_known::Rfc3339;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl MissionService for MissionRpc {
    type StreamEventsStream = Pin<
        Box<
            dyn Stream<Item = Result<mission::v0::StreamEventsResponse, tonic::Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;
    type StreamUnitsStream = Pin<
        Box<
            dyn Stream<Item = Result<mission::v0::StreamUnitsResponse, tonic::Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn stream_events(
        &self,
        _request: Request<mission::v0::StreamEventsRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        let events = self.events().await;
        let stream = AbortableStream::new(self.shutdown_signal.signal(), events.map(Ok));
        Ok(Response::new(Box::pin(stream)))
    }

    async fn stream_units(
        &self,
        request: Request<mission::v0::StreamUnitsRequest>,
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
                result.map(|update| mission::v0::StreamUnitsResponse {
                    update: Some(update),
                })
            }),
        );
        Ok(Response::new(Box::pin(stream)))
    }

    async fn get_scenario_start_time(
        &self,
        _: Request<mission::v0::GetScenarioStartTimeRequest>,
    ) -> Result<Response<mission::v0::GetScenarioStartTimeResponse>, Status> {
        let datetime = Self::get_scenario_start_time(self).await?;
        Ok(Response::new(mission::v0::GetScenarioStartTimeResponse {
            datetime: datetime.format(&Rfc3339).map_err(|err| {
                Status::internal(format!("failed to format date as ISO 8601 string: {}", err))
            })?,
        }))
    }

    async fn get_scenario_current_time(
        &self,
        _: Request<mission::v0::GetScenarioCurrentTimeRequest>,
    ) -> Result<Response<mission::v0::GetScenarioCurrentTimeResponse>, Status> {
        let current = self
            .get_absolute_time(Request::new(timer::v0::GetAbsoluteTimeRequest {}))
            .await?
            .into_inner();
        let datetime = to_datetime(current.year, current.month, current.day, current.time)?;
        Ok(Response::new(mission::v0::GetScenarioCurrentTimeResponse {
            datetime: datetime.format(&Rfc3339).map_err(|err| {
                Status::internal(format!("failed to format date as ISO 8601 string: {}", err))
            })?,
        }))
    }

    async fn add_mission_command(
        &self,
        request: Request<mission::v0::AddMissionCommandRequest>,
    ) -> Result<Response<mission::v0::AddMissionCommandResponse>, Status> {
        let res = self.request("addMissionCommand", request).await?;
        Ok(Response::new(res))
    }

    async fn add_mission_command_sub_menu(
        &self,
        request: Request<mission::v0::AddMissionCommandSubMenuRequest>,
    ) -> Result<Response<mission::v0::AddMissionCommandSubMenuResponse>, Status> {
        let res = self.request("addMissionCommandSubMenu", request).await?;
        Ok(Response::new(res))
    }

    async fn remove_mission_command_item(
        &self,
        request: Request<mission::v0::RemoveMissionCommandItemRequest>,
    ) -> Result<Response<mission::v0::RemoveMissionCommandItemResponse>, Status> {
        let res = self.request("removeMissionCommandItem", request).await?;
        Ok(Response::new(res))
    }

    async fn add_coalition_command(
        &self,
        request: Request<mission::v0::AddCoalitionCommandRequest>,
    ) -> Result<Response<mission::v0::AddCoalitionCommandResponse>, Status> {
        let res = self.request("addCoalitionCommand", request).await?;
        Ok(Response::new(res))
    }

    async fn add_coalition_command_sub_menu(
        &self,
        request: Request<mission::v0::AddCoalitionCommandSubMenuRequest>,
    ) -> Result<Response<mission::v0::AddCoalitionCommandSubMenuResponse>, Status> {
        let res = self.request("addCoalitionCommandSubMenu", request).await?;
        Ok(Response::new(res))
    }

    async fn remove_coalition_command_item(
        &self,
        request: Request<mission::v0::RemoveCoalitionCommandItemRequest>,
    ) -> Result<Response<mission::v0::RemoveCoalitionCommandItemResponse>, Status> {
        let res = self.request("removeCoalitionCommandItem", request).await?;
        Ok(Response::new(res))
    }

    async fn add_group_command(
        &self,
        request: Request<mission::v0::AddGroupCommandRequest>,
    ) -> Result<Response<mission::v0::AddGroupCommandResponse>, Status> {
        let res = self.request("addGroupCommand", request).await?;
        Ok(Response::new(res))
    }

    async fn add_group_command_sub_menu(
        &self,
        request: Request<mission::v0::AddGroupCommandSubMenuRequest>,
    ) -> Result<Response<mission::v0::AddGroupCommandSubMenuResponse>, Status> {
        let res = self.request("addGroupCommandSubMenu", request).await?;
        Ok(Response::new(res))
    }

    async fn remove_group_command_item(
        &self,
        request: Request<mission::v0::RemoveGroupCommandItemRequest>,
    ) -> Result<Response<mission::v0::RemoveGroupCommandItemResponse>, Status> {
        let res = self.request("removeGroupCommandItem", request).await?;
        Ok(Response::new(res))
    }

    async fn get_session_id(
        &self,
        _request: Request<mission::v0::GetSessionIdRequest>,
    ) -> Result<Response<mission::v0::GetSessionIdResponse>, Status> {
        let response = self
            .get_user_flag(Request::new(trigger::v0::GetUserFlagRequest {
                flag: String::from("dcs_grpc_session_id"),
            }))
            .await?
            .into_inner();
        let current_session_id = response.value as i64;
        if current_session_id == 0 {
            let ts = OffsetDateTime::now_utc().unix_timestamp();
            self.set_user_flag(Request::new(trigger::v0::SetUserFlagRequest {
                flag: String::from("dcs_grpc_session_id"),
                value: ts as u32,
            }))
            .await?;
            Ok(Response::new(mission::v0::GetSessionIdResponse {
                session_id: ts,
            }))
        } else {
            Ok(Response::new(mission::v0::GetSessionIdResponse {
                session_id: current_session_id,
            }))
        }
    }
}

impl MissionRpc {
    pub(super) async fn get_scenario_start_time(&self) -> Result<OffsetDateTime, Status> {
        let cache = self.cache.read().await;
        if let Some(datetime) = &cache.scenario_start_time {
            return Ok(*datetime);
        }
        std::mem::drop(cache);

        let start = self
            .get_time_zero(Request::new(timer::v0::GetTimeZeroRequest {}))
            .await?
            .into_inner();
        let datetime = to_datetime(start.year, start.month, start.day, start.time)?;

        let mut cache = self.cache.write().await;
        cache.scenario_start_time = Some(datetime);

        Ok(datetime)
    }
}

fn to_datetime(year: i32, month: u32, day: u32, time: f64) -> Result<OffsetDateTime, Status> {
    let month = u8::try_from(month)
        .map_err(|err| Status::internal(format!("received invalid month: {}", err)))?;
    let month = Month::try_from(month)
        .map_err(|err| Status::internal(format!("received invalid month: {}", err)))?;
    let day = u8::try_from(day)
        .map_err(|err| Status::internal(format!("received invalid day: {}", err)))?;
    let date = Date::from_calendar_date(year, month, day)
        .map_err(|err| Status::internal(format!("received invalid date: {}", err)))?;
    let time = Time::from_hms(0, 0, 0).unwrap() + Duration::seconds(time as i64);
    let datetime = PrimitiveDateTime::new(date, time).assume_offset(UtcOffset::UTC);
    Ok(datetime)
}
