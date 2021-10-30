use std::pin::Pin;

use super::HookRpc;
use crate::shutdown::AbortableStream;
use futures_util::{Stream, TryStreamExt};
use stubs::hook::hook_service_server::HookService;
use stubs::*;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl HookService for HookRpc {
    type StreamChatMessagesStream = Pin<
        Box<
            dyn Stream<Item = Result<hook::StreamChatMessagesResponse, tonic::Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn get_mission_name(
        &self,
        request: Request<hook::GetMissionNameRequest>,
    ) -> Result<Response<hook::GetMissionNameResponse>, Status> {
        let res: hook::GetMissionNameResponse = self.request("getMissionName", request).await?;
        Ok(Response::new(res))
    }

    async fn stream_chat_messages(
        &self,
        _request: Request<hook::StreamChatMessagesRequest>,
    ) -> Result<Response<Self::StreamChatMessagesStream>, Status> {
        let rx = BroadcastStream::new(self.chat.subscribe());
        let stream = AbortableStream::new(
            self.shutdown_signal.signal(),
            rx.map_err(|err| Status::unknown(err.to_string())),
        );
        Ok(Response::new(Box::pin(stream)))
    }

    async fn eval(
        &self,
        request: Request<hook::EvalRequest>,
    ) -> Result<Response<hook::EvalResponse>, Status> {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }

        let json: serde_json::Value = self.request("hookEval", request).await?;
        let json = serde_json::to_string(&json).map_err(|err| {
            Status::internal(format!("failed to deserialize eval result: {}", err))
        })?;
        Ok(Response::new(hook::EvalResponse { json }))
    }
}
