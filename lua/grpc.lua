env.info("[GRPC] loading ...")

--
-- load and start RPC
--
package.loaded["dcs_grpc_server"] = nil
grpc = require "dcs_grpc_server"
grpc.start()
local stopped = false

--
-- Helper methods
--

GRPC.success = function(result)
  return {
    result = result
  }
end

GRPC.error = function(msg)
  return {
    error = msg
  }
end

--
-- RPC methods
--

GRPC.methods = {}
dofile(GRPC.basePath .. [[methods\trigger.lua]])

--
-- RPC request handler
--
local function handleRequest(method, params)
  local fn = GRPC.methods[method]

  if type(fn) == "function" then
    local ok, result = pcall(fn, params)
    if ok then
      return result
    else
      env.error("[GRPC] error executing "..method..": "..tostring(result))
      return {
        error = tostring(result)
      }
    end
  else
    return {
      error = "unsupported method "..method
    }
  end
end

--
-- execute gRPC requests every ~0.02 seconds
--
local function next()
  local i = 0
  while grpc.next(handleRequest) do
    i = i + 1
    if i > 10 then
      break
    end
  end
end

timer.scheduleFunction(function()
  if not stopped then
    local ok, err = pcall(next)
    if not ok then
      env.error("[GRPC] Error retrieving next command: "..tostring(err))
    end

    return timer.getTime() + .02 -- return time of next call
  end
end, nil, timer.getTime() + .02)

--
-- listen to DCS events
--
local function identifier(obj)
  if obj == nil then
    return nil
  end
  return obj:getName()
end

local function onEvent(event)
  if (event.id ~= world.event.S_EVENT_MISSION_START and event.id ~= world.event.S_EVENT_MISSION_END and event.id ~= world.event.S_EVENT_TOOK_CONTROL and event.id ~= world.event.S_EVENT_MARK_ADDED and event.id ~= world.event.S_EVENT_MARK_CHANGE and event.id ~= S_EVENT_MARK_REMOVED) and event.initiator == nil then
    env.info("[GRPC] Ignoring event (id: "..tostring(event.id)..") with missing initiator")

  elseif event.id == world.event.S_EVENT_SHOT then
    grpc.event({
      time = event.time,
      shot = {
        initiator = identifier(event.initiator),
        weapon = event.weapon:getName(),
      },
    })

  elseif event.id == world.event.S_EVENT_HIT then
    if event.target ~= nil then
      local target = {
        -- minus one, because protobuf enums must start at zero
        category = event.target:getCategory() - 1,
      }
      if target.category == 1 then -- weapon
        target.id = event.target:getName()
      else
        target.name = event.target:getName()
      end

      local weapon = nil
      if event.weapon ~= nil then
        weapon = event.weapon:getName()
      end
      grpc.event({
        time = event.time,
        hit = {
          initiator = identifier(event.initiator),
          weapon = weapon,
          target = target,
        },
      })
    else
      env.error("[GRPC] Ignoring HIT event without target")
    end

  elseif event.id == world.event.S_EVENT_TAKEOFF then
    grpc.event({
      time = event.time,
      takeoff = {
        initiator = identifier(event.initiator),
        place = identifier(event.place),
      },
    })

  elseif event.id == world.event.S_EVENT_LAND then
    grpc.event({
      time = event.time,
      land = {
        initiator = identifier(event.initiator),
        place = identifier(event.place),
      },
    })

  elseif event.id == world.event.S_EVENT_CRASH then
    grpc.event({
      time = event.time,
      crash = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_EJECTION then
    grpc.event({
      time = event.time,
      ejection = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_REFUELING then
    grpc.event({
      time = event.time,
      refueling = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_DEAD then
    local payload = {}
    if event.target:getCategory() == 2 then -- weapon
      payload.id = event.target:getName()
    else
      payload.name = event.target:getName()
    end

    grpc.event({
      time = event.time,
      dead = payload,
    })

  elseif event.id == world.event.S_EVENT_PILOT_DEAD then
    grpc.event({
      time = event.time,
      pilotDead = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_BASE_CAPTURED then
    grpc.event({
      time = event.time,
      baseCapture = {
        initiator = identifier(event.initiator),
        place = identifier(event.place),
      },
    })

  elseif event.id == world.event.S_EVENT_MISSION_START then
    grpc.event({
      time = event.time,
      missionStart = {},
    })

  elseif event.id == world.event.S_EVENT_MISSION_END then
    grpc.event({
      time = event.time,
      missionEnd = {},
    })

    grpc.stop()
    stopped = true

  -- unimplemented: S_EVENT_TOOK_CONTROL

  elseif event.id == world.event.S_EVENT_REFUELING_STOP then
    grpc.event({
      time = event.time,
      refuelingStop = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_BIRTH then
    grpc.event({
      time = event.time,
      birth = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_HUMAN_FAILURE then
    grpc.event({
      time = event.time,
      systemFailure = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_ENGINE_STARTUP then
    grpc.event({
      time = event.time,
      engineStartup = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_ENGINE_SHUTDOWN  then
    grpc.event({
      time = event.time,
      engineShutdown = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_PLAYER_ENTER_UNIT then
    grpc.event({
      time = event.time,
      playerEnterUnit = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_PLAYER_LEAVE_UNIT then
    grpc.event({
      time = event.time,
      playerLeaveUnit = {
        initiator = identifier(event.initiator),
      },
    })

    -- unimplemented: S_EVENT_PLAYER_COMMENT

  elseif event.id == world.event.S_EVENT_SHOOTING_START then
    grpc.event({
      time = event.time,
      shootingStart = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_SHOOTING_END then
    grpc.event({
      time = event.time,
      shootingEnd = {
        initiator = identifier(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_MARK_ADDED then
    grpc.event({
      time = event.time,
      markAdd = {
        initiator = identifier(event.initiator),
        groupId = event.groupID > -1 and event.groupID or nil,
        coalition = event.coalition > -1 and event.coalition or nil,
        id = event.idx,
        -- x and z are rotated here compared to group/unit coords
        pos = { x = event.pos.z, y = event.pos.y, z = event.pos.x },
        text = event.text,
      },
    })

  elseif event.id == world.event.S_EVENT_MARK_CHANGE then
    grpc.event({
      time = event.time,
      markChange = {
        initiator = identifier(event.initiator),
        groupId = event.groupID > -1 and event.groupID or nil,
        coalition = event.coalition > -1 and event.coalition or nil,
        id = event.idx,
        -- x and z are rotated here compared to group/unit coords
        pos = { x = event.pos.z, y = event.pos.y, z = event.pos.x },
        text = event.text,
      },
    })

  elseif event.id == world.event.S_EVENT_MARK_REMOVED then
    grpc.event({
      time = event.time,
      markRemove = {
        initiator = identifier(event.initiator),
        groupId = event.groupID > -1 and event.groupID or nil,
        coalition = event.coalition > -1 and event.coalition or nil,
        id = event.idx,
        -- x and z are rotated here compared to group/unit coords
        pos = { x = event.pos.z, y = event.pos.y, z = event.pos.x },
        text = event.text,
      },
    })

  else
    env.info("[GRPC] Skipping unimplemented event id "..tostring(event.id))
  end
end

local eventHandler = {}
function eventHandler:onEvent(event)
  if not stopped then
    local ok, err = pcall(onEvent, event)
    if not ok then
      env.error("[GRPC] Error in event handler: "..tostring(err))
    end
  end
end
world.addEventHandler(eventHandler)

env.info("[GRPC] loaded ...")
