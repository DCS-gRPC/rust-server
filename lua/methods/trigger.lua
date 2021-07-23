--
-- RPC trigger actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_trigger
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

GRPC.methods.markToAll = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)

  trigger.action.markToAll(params.id, params.text, point, params.readOnly, params.message)

  return GRPC.success(nil)
end

GRPC.methods.markToCoalition = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)

  trigger.action.markToCoalition(params.id, params.text, point, params.coalition, params.readOnly, params.message)

  return GRPC.success(nil)
end

GRPC.methods.markToGroup = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)

  trigger.action.markToGroup(params.id, params.text, point, params.groupId, params.readOnly, params.message)

  return GRPC.success(nil)
end

GRPC.methods.removeMark = function(params)
  trigger.action.removeMark(params.id)

  return GRPC.success(nil)
end