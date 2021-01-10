--
-- RPC trigger actions
--

GRPC.methods.outText = function(params)
  trigger.action.outText(params.text, params.displayTime, params.clearView)

  return GRPC.success(nil)
end

GRPC.methods.getUserFlag = function(params)
  return GRPC.success({
    value = trigger.misc.getUserFlag(params.flag),
  })
end

GRPC.methods.setUserFlag = function(params)
  trigger.action.setUserFlag(params.flag, params.value)
  return GRPC.success(nil)
end
