--
-- RPC trigger actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_trigger
--

local world = world

-- All MarkPanels must have a unique ID but there is no way of
-- delegating the creationg of this ID to the game, so we have
-- to have the following code to make sure we always get a new
-- unique id
local MarkId = 0

local function getMarkId()
    local panels =  world.getMarkPanels()
    local idx = MarkId
    if panels then
        local l_max = math.max
        for _,panel in ipairs(panels) do
            idx = l_max(panel.idx, idx)
        end
    end
    idx = idx + 1
    MarkId = idx
    return idx
end


GRPC.methods.outText = function(params)
  trigger.action.outText(params.text, params.displayTime, params.clearView)

  return GRPC.success(nil)
end

GRPC.methods.outTextForCoalition = function(params)
  trigger.action.outTextForCoalition(params.coalition, params.text, params.displayTime, params.clearView)

  return GRPC.success(nil)
end

GRPC.methods.outTextForGroup = function(params)
  trigger.action.outTextForGroup(params.groupId, params.text, params.displayTime, params.clearView)

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
  local idx = getMarkId()

  trigger.action.markToAll(idx, params.text, point, params.readOnly, params.message)

  return GRPC.success({
    id = idx
  })
end

GRPC.methods.markToCoalition = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)
  local idx = getMarkId()

  trigger.action.markToCoalition(idx, params.text, point, params.coalition, params.readOnly, params.message)

  return GRPC.success({
    id = idx
  })
end

GRPC.methods.markToGroup = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)
  local idx = getMarkId()

  trigger.action.markToGroup(idx, params.text, point, params.groupId, params.readOnly, params.message)

  return GRPC.success({
    id = idx
  })
end

GRPC.methods.removeMark = function(params)
  trigger.action.removeMark(params.id)

  return GRPC.success(nil)
end

GRPC.methods.explosion = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)

  trigger.action.explosion(point, params.power)

  return GRPC.success(nil)
end

GRPC.methods.smoke = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)

  trigger.action.smoke(point, params.color)

  return GRPC.success(nil)
end

GRPC.methods.illuminationBomb = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)

  trigger.action.illuminationBomb(point, params.power)

  return GRPC.success(nil)
end

GRPC.methods.signalFlare = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)

  trigger.action.signalFlare(point, params.color, params.azimuth)

  return GRPC.success(nil)
end