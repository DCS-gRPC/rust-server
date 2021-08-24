local function exporter(object)
  if object == nil then
    return nil
  end

  local category = object:getCategory()

  if(category == Object.Category.UNIT) then
    return GRPC.exporters.unit(object)
  elseif(category == Object.Category.WEAPON) then
    return GRPC.exporters.weapon(object)
  elseif(category == Object.Category.STATIC) then
    return GRPC.exporters.static(object)
  elseif(category == Object.Category.BASE) then
    return GRPC.exporters.airbase(object)
  elseif(category == Object.Category.SCENERY) then
    return GRPC.exporters.scenery(object)
  elseif(category == Object.Category.Cargo) then
    return GRPC.exporters.cargo(object)
  else
    env.info("[GRPC] Could not determine object category of object with ID: " .. object:getID() .. ", Category: " .. category)
    return nil
  end
end

local function typed_exporter(object)
  local grpcTable = {}
  if object == nil then
    grpcTable["unknown"] = nil
    return grpcTable
  end

  local category = object:getCategory()

  if(category == Object.Category.UNIT) then
    grpcTable["unit"] = exporter(object)
  elseif(category == Object.Category.WEAPON) then
    grpcTable["weapon"] = exporter(object)
  elseif(category == Object.Category.STATIC) then
    grpcTable["static"] = exporter(object)
  elseif(category == Object.Category.BASE) then
    grpcTable["airbase"] = exporter(object)
  elseif(category == Object.Category.SCENERY) then
    grpcTable["scenery"] = exporter(object)
  elseif(category == Object.Category.Cargo) then
    grpcTable["cargo"] = exporter(object)
  else
    env.info("[GRPC] Could not determine object category of object with ID: " .. object:getID() .. ", Category: " .. category)
    grpcTable["unknown"] = GRPC.exporters.unknown(object)
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
        initiator = {initiator = typed_exporter(event.initiator)},
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
          initiator = {initiator = typed_exporter(event.initiator)},
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
        initiator = {initiator = typed_exporter(event.initiator)},
        place = exporter(event.place),
      },
    })

  elseif event.id == world.event.S_EVENT_LAND then
    grpc.event({
      time = event.time,
      event = {
        type = "land",
        initiator = {initiator = typed_exporter(event.initiator)},
        place = exporter(event.place),
      },
    })

  elseif event.id == world.event.S_EVENT_CRASH then
    grpc.event({
      time = event.time,
      event = {
        type = "crash",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_EJECTION then
    grpc.event({
      time = event.time,
      event = {
        type = "ejection",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_REFUELING then
    grpc.event({
      time = event.time,
      event = {
        type = "refueling",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_DEAD then
    grpc.event({
      time = event.time,
      event = {
        type = "dead",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_PILOT_DEAD then
    grpc.event({
      time = event.time,
      event = {
        type = "pilotDead",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_BASE_CAPTURED then
    grpc.event({
      time = event.time,
      event = {
        type = "baseCapture",
        initiator = {initiator = typed_exporter(event.initiator)},
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
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_BIRTH then
    grpc.event({
      time = event.time,
      event = {
        type = "birth",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_HUMAN_FAILURE then
    grpc.event({
      time = event.time,
      event = {
        type = "systemFailure",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_ENGINE_STARTUP then
    grpc.event({
      time = event.time,
      event = {
        type = "engineStartup",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_ENGINE_SHUTDOWN  then
    grpc.event({
      time = event.time,
      event = {
        type = "engineShutdown",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_PLAYER_ENTER_UNIT then
    grpc.event({
      time = event.time,
      event = {
        type = "playerEnterUnit",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_PLAYER_LEAVE_UNIT then
    grpc.event({
      time = event.time,
      event = {
        type = "playerLeaveUnit",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

    -- unimplemented: S_EVENT_PLAYER_COMMENT

  elseif event.id == world.event.S_EVENT_SHOOTING_START then
    grpc.event({
      time = event.time,
      event = {
        type = "shootingStart",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_SHOOTING_END then
    grpc.event({
      time = event.time,
      event = {
        type = "shootingEnd",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    })

  elseif event.id == world.event.S_EVENT_MARK_ADDED then
    local payload = {
      type = "markAdd",
      initiator = exporter(event.initiator),
      id = event.idx,
      pos = GRPC.toLatLonPosition(event.pos),
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
      pos = GRPC.toLatLonPosition(event.pos),
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
      pos = GRPC.toLatLonPosition(event.pos),
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