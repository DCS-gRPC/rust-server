--
-- RPC trigger actions
--

GRPC.methods.outText = function(params)
  trigger.action.outText(params.text, params.displayTime, params.clearView)

  return GRPC.success(nil)
end

