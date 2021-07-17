local function exporter(object)
  if object == nil then
    return nil
  end

  local category = object:getCategory()

  if(category == Object.Category.UNIT) then
    return GRPC.exporters.unit(object)
  elseif(category == Object.Category.WEAPON) then
    return GRPC.exporters.weapon(object)
  end

  return object:getName()
end

local function typed_exporter(object)
  local category = object:getCategory()
  local grpcTable = {}

  if(category == Object.Category.UNIT) then
    grpcTable["unit"] = GRPC.exporters.unit(object)
  elseif(category == Object.Category.WEAPON) then
    grpcTable["weapon"] = GRPC.exporters.weapon(object)
  elseif(category == Object.Category.STATIC) then
    grpcTable["static"] = GRPC.exporters.static(object)
  elseif(category == Object.Category.BASE) then
    grpcTable["airbase"] = GRPC.exporters.airbase(object)
  elseif(category == Object.Category.SCENERY) then
    grpcTable["scenery"] = GRPC.exporters.scenery(object)
  elseif(category == Object.Category.Cargo) then
    grpcTable["cargo"] = GRPC.exporters.cargo(object)
  else
    env.info("[GRPC] Could not determine object category of object with ID: " .. object:getID() .. ", Category: " .. category)
    grpcTable["object"] = GRPC.exporters.object(object)
  end

  return grpcTable

end

GRPC.onDcsEvent = function(event)
  if (event.id ~= world.event.S_EVENT_MISSION_START and event.id ~= world.event.S_EVENT_MISSION_END and event.id ~= world.event.S_EVENT_TOOK_CONTROL and event.id ~= world.event.S_EVENT_MARK_ADDED and event.id ~= world.event.S_EVENT_MARK_CHANGE and event.id ~= S_EVENT_MARK_REMOVED) and event.initiator == nil then
    env.info("[GRPC] Ignoring event (id: "..tostring(event.id)..") with missing initiator")

  elseif event.id == world.event.S_EVENT_SHOT then
    grpc.event({
      time = event.time,
      event = {
        type = "shot",
        initiator = exporter(event.initiator),
        weapon = exporter(event.weapon)
      },
    })

  elseif event.id == world.event.S_EVENT_HIT then
    if event.target ~= nil then
      local result = {}
      result.target = typed_exporter(event.target)

      grpc.event({
        time = event.time,
        event = {
          type = "hit",
          initiator = exporter(event.initiator),
          weapon = exporter(event.weapon),
          target = result,
        },
      })
    else
      env.error("[GRPC] Ignoring HIT event without target")
    end

  elseif event.id == world.event.S_EVENT_TAKEOFF then
    grpc.event({
      time = event.time,
      event = {
        type = "takeoff",
        initiator = exporter(event.initiator),
        place = exporter(event.place),
      },
    })

  elseif event.id == world.event.S_EVENT_LAND then
    grpc.event({
      time = event.time,
      event = {
        type = "land",
        initiator = exporter(event.initiator),
        place = exporter(event.place),
      },
    })

  elseif event.id == world.event.S_EVENT_CRASH then
    grpc.event({
      time = event.time,
      event = {
        type = "crash",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_EJECTION then
    grpc.event({
      time = event.time,
      event = {
        type = "ejection",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_REFUELING then
    grpc.event({
      time = event.time,
      event = {
        type = "refueling",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_DEAD then
    local payload = {
      type = "dead",
    }
    if event.target:getCategory() == 2 then -- weapon
      payload.id = event.target:getName()
    else
      payload.name = event.target:getName()
    end

    grpc.event({
      time = event.time,
      event = payload,
    })

  elseif event.id == world.event.S_EVENT_PILOT_DEAD then
    grpc.event({
      time = event.time,
      event = {
        type = "pilotDead",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_BASE_CAPTURED then
    grpc.event({
      time = event.time,
      event = {
        type = "baseCapture",
        initiator = exporter(event.initiator),
        place = exporter(event.place),
      },
    })

  elseif event.id == world.event.S_EVENT_MISSION_START then
    grpc.event({
      time = event.time,
      event = {
        type = "missionStart",
      },
    })

  elseif event.id == world.event.S_EVENT_MISSION_END then
    grpc.event({
      time = event.time,
      event = {
        type = "missionEnd",
      },
    })

    grpc.stop()
    GRPC.stopped = true

  -- unimplemented: S_EVENT_TOOK_CONTROL

  elseif event.id == world.event.S_EVENT_REFUELING_STOP then
    grpc.event({
      time = event.time,
      event = {
        type = "refuelingStop",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_BIRTH then
    grpc.event({
      time = event.time,
      event = {
        type = "birth",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_HUMAN_FAILURE then
    grpc.event({
      time = event.time,
      event = {
        type = "systemFailure",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_ENGINE_STARTUP then
    grpc.event({
      time = event.time,
      event = {
        type = "engineStartup",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_ENGINE_SHUTDOWN  then
    grpc.event({
      time = event.time,
      event = {
        type = "engineShutdown",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_PLAYER_ENTER_UNIT then
    grpc.event({
      time = event.time,
      event = {
        type = "playerEnterUnit",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_PLAYER_LEAVE_UNIT then
    grpc.event({
      time = event.time,
      event = {
        type = "playerLeaveUnit",
        initiator = exporter(event.initiator),
      },
    })

    -- unimplemented: S_EVENT_PLAYER_COMMENT

  elseif event.id == world.event.S_EVENT_SHOOTING_START then
    grpc.event({
      time = event.time,
      event = {
        type = "shootingStart",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_SHOOTING_END then
    grpc.event({
      time = event.time,
      event = {
        type = "shootingEnd",
        initiator = exporter(event.initiator),
      },
    })

  elseif event.id == world.event.S_EVENT_MARK_ADDED then
    local payload = {
      type = "markAdd",
      initiator = exporter(event.initiator),
      id = event.idx,
      pos = GRPC.toLatLonPosition (event.pos),
      text = event.text,
    }
    if event.groupID > -1 and event.groupID then
      payload.groupId = event.groupId
    elseif event.coalition > -1 and event.coalition then
      payload.coalition = event.coalition
    end
    grpc.event({
      time = event.time,
      event = payload,
    })

  elseif event.id == world.event.S_EVENT_MARK_CHANGE then
    local payload = {
      type = "markChange",
      initiator = exporter(event.initiator),
      id = event.idx,
      pos = GRPC.toLatLonPosition (event.pos),
      text = event.text,
    }
    if event.groupID > -1 and event.groupID then
      payload.groupId = event.groupId
    elseif event.coalition > -1 and event.coalition then
      payload.coalition = event.coalition
    end
    grpc.event({
      time = event.time,
      event = payload,
    })

  elseif event.id == world.event.S_EVENT_MARK_REMOVED then
    local payload = {
      type = "markRemove",
      initiator = exporter(event.initiator),
      id = event.idx,
      pos = GRPC.toLatLonPosition (event.pos),
      text = event.text,
    }
    if event.groupID > -1 and event.groupID then
      payload.groupId = event.groupId
    elseif event.coalition > -1 and event.coalition then
      payload.coalition = event.coalition
    end
    grpc.event({
      time = event.time,
      event = payload,
    })

  else
    env.info("[GRPC] Skipping unimplemented event id "..tostring(event.id))
  end
end