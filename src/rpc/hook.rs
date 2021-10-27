use std::pin::Pin;

use super::dcs::hook::hook_service_server::HookService;
use super::dcs::*;
use super::HookRpc;
use crate::shutdown::AbortableStream;
use futures_util::{Stream, TryStreamExt};
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl HookService for HookRpc {
    type StreamChatStream = Pin<
        Box<dyn Stream<Item = Result<hook::ChatMessage, tonic::Status>> + Send + Sync + 'static>,
    >;

    async fn get_mission_name(
        &self,
        request: Request<hook::GetMissionNameRequest>,
    ) -> Result<Response<hook::GetMissionNameResponse>, Status> {
        let res: hook::GetMissionNameResponse = self.request("getMissionName", request).await?;
        Ok(Response::new(res))
    }

    async fn stream_chat(
        &self,
        _request: Request<hook::StreamChatRequest>,
    ) -> Result<Response<Self::StreamChatStream>, Status> {
        let rx = BroadcastStream::new(self.chat.subscribe());
        let stream = AbortableStream::new(
            self.shutdown_signal.signal(),
            rx.map_err(|err| Status::unknown(err.to_string())),
        );
        Ok(Response::new(Box::pin(stream)))
    }

    async fn eval(
        &self,
        request: Request<custom::EvalRequest>,
    ) -> Result<Response<custom::EvalResponse>, Status> {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }

        let json: serde_json::Value = self.request("hookEval", request).await?;
        let json = serde_json::to_string(&json).map_err(|err| {
            Status::internal(format!("failed to deserialize eval result: {}", err))
        })?;
        Ok(Response::new(custom::EvalResponse { json }))
    }
}
