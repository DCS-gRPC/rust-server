use std::pin::Pin;

use super::dcs::mission::mission_service_server::MissionService;
use super::dcs::*;
use super::MissionRpc;
use crate::rpc::dcs::timer::timer_service_server::TimerService;
use crate::shutdown::AbortableStream;
use chrono::Duration;
use chrono::{TimeZone, Utc};
use futures_util::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl MissionService for MissionRpc {
    type StreamEventsStream =
        Pin<Box<dyn Stream<Item = Result<mission::Event, tonic::Status>> + Send + Sync + 'static>>;
    type StreamUnitsStream = Pin<
        Box<dyn Stream<Item = Result<mission::UnitUpdate, tonic::Status>> + Send + Sync + 'static>,
    >;

    async fn stream_events(
        &self,
        _request: Request<mission::StreamEventsRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        let events = self.events().await;
        let stream = AbortableStream::new(self.shutdown_signal.signal(), events.map(Ok));
        Ok(Response::new(Box::pin(stream)))
    }

    async fn stream_units(
        &self,
        request: Request<mission::StreamUnitsRequest>,
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
                result.map(|update| mission::UnitUpdate {
                    update: Some(update),
                })
            }),
        );
        Ok(Response::new(Box::pin(stream)))
    }

    async fn get_scenario_start_time(
        &self,
        _: Request<mission::GetScenarioStartTimeRequest>,
    ) -> Result<Response<mission::GetScenarioStartTimeResponse>, Status> {
        let start = self
            .get_time_zero(Request::new(timer::GetTimeZeroRequest {}))
            .await?
            .into_inner();

        let dt = Utc.ymd(start.year, start.month, start.day).and_hms(0, 0, 0);
        let dt = dt + Duration::seconds(start.time as i64);

        Ok(Response::new(mission::GetScenarioStartTimeResponse {
            datetime: dt.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        }))
    }
}
