local function exporter(object)
  if object == nil then
    return nil
  end

  local category = object:getCategory()

  if category == Object.Category.BASE or object.className_ == 'Airbase' then
    -- carriers are of category unit, but are a Airbase class
    return GRPC.exporters.airbase(object)
  elseif category == Object.Category.UNIT then
    return GRPC.exporters.unit(object)
  elseif category == Object.Category.WEAPON then
    return GRPC.exporters.weapon(object)
  elseif category == Object.Category.STATIC then
    return GRPC.exporters.static(object)
  elseif category == Object.Category.SCENERY then
    return GRPC.exporters.scenery(object)
  elseif category == Object.Category.Cargo then
    return GRPC.exporters.cargo(object)
  else
    GRPC.logWarning(
      "Could not determine object category of object with ID: " .. object:getID()
        .. ", Category: " .. category
    )
    return nil
  end
end

local function typed_exporter(object)
  if object == nil then
    return nil
  end

  local grpcTable = {}
  local category = object:getCategory()

  if category == Object.Category.BASE or object.className_ == 'Airbase' then
    grpcTable["airbase"] = exporter(object)
  elseif category == Object.Category.UNIT then
    grpcTable["unit"] = exporter(object)
  elseif category == Object.Category.WEAPON then
    grpcTable["weapon"] = exporter(object)
  elseif category == Object.Category.STATIC then
    grpcTable["static"] = exporter(object)
  elseif category == Object.Category.SCENERY then
    grpcTable["scenery"] = exporter(object)
  elseif category == Object.Category.Cargo then
    grpcTable["cargo"] = exporter(object)
  else
    GRPC.logWarning(
      "Could not determine object category of object with ID: " .. object:getID()
        .. ", Category: " .. category
    )
    grpcTable["unknown"] = GRPC.exporters.unknown(object)
  end

  return grpcTable
end

GRPC.onDcsEvent = function(event)
  if event.id == world.event.S_EVENT_INVALID then
    return nil

  elseif event.id == world.event.S_EVENT_SHOT then
    return {
      time = event.time,
      event = {
        type = "shot",
        initiator = {initiator = typed_exporter(event.initiator)},
        weapon = exporter(event.weapon)
      },
    }

  elseif event.id == world.event.S_EVENT_HIT then
    return {
      time = event.time,
      event = {
        type = "hit",
        initiator = {initiator = typed_exporter(event.initiator)},
        weapon = exporter(event.weapon),
        target = {target = typed_exporter(event.target)},
        weaponName = event.weapon_name,
      },
    }

  elseif event.id == world.event.S_EVENT_TAKEOFF then
    return {
      time = event.time,
      event = {
        type = "takeoff",
        initiator = {initiator = typed_exporter(event.initiator)},
        place = exporter(event.place),
      },
    }

  elseif event.id == world.event.S_EVENT_LAND then
    return {
      time = event.time,
      event = {
        type = "land",
        initiator = {initiator = typed_exporter(event.initiator)},
        place = exporter(event.place),
      },
    }

  elseif event.id == world.event.S_EVENT_CRASH then
    return {
      time = event.time,
      event = {
        type = "crash",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    }

  elseif event.id == world.event.S_EVENT_EJECTION then
    return {
      time = event.time,
      event = {
        type = "ejection",
        initiator = {initiator = typed_exporter(event.initiator)},
        target = {target = typed_exporter(event.target)},
      },
    }

  elseif event.id == world.event.S_EVENT_REFUELING then
    return {
      time = event.time,
      event = {
        type = "refueling",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    }

  elseif event.id == world.event.S_EVENT_DEAD then
    return {
      time = event.time,
      event = {
        type = "dead",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    }

  elseif event.id == world.event.S_EVENT_PILOT_DEAD then
    return {
      time = event.time,
      event = {
        type = "pilotDead",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    }

  elseif event.id == world.event.S_EVENT_BASE_CAPTURED then
    return {
      time = event.time,
      event = {
        type = "baseCapture",
        initiator = {initiator = typed_exporter(event.initiator)},
        place = exporter(event.place),
      },
    }

  elseif event.id == world.event.S_EVENT_MISSION_START then
    return {
      time = event.time,
      event = {
        type = "missionStart",
      },
    }

  elseif event.id == world.event.S_EVENT_MISSION_END then
    return {
      time = event.time,
      event = {
        type = "missionEnd",
      },
    }

  -- S_EVENT_TOOK_CONTROL: not implemented as apparently not used anymore

  elseif event.id == world.event.S_EVENT_REFUELING_STOP then
    return {
      time = event.time,
      event = {
        type = "refuelingStop",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    }

  elseif event.id == world.event.S_EVENT_BIRTH then
    return {
      time = event.time,
      event = {
        type = "birth",
        initiator = {initiator = typed_exporter(event.initiator)},
        place = exporter(event.place),
      },
    }

  elseif event.id == world.event.S_EVENT_HUMAN_FAILURE then
    return {
      time = event.time,
      event = {
        type = "humanFailure",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    }

  elseif event.id == world.event.S_EVENT_DETAILED_FAILURE then
    return {
      time = event.time,
      event = {
        type = "detailedFailure",
        target = {target = typed_exporter(event.target)},
      },
    }

  elseif event.id == world.event.S_EVENT_ENGINE_STARTUP then
    return {
      time = event.time,
      event = {
        type = "engineStartup",
        initiator = {initiator = typed_exporter(event.initiator)},
        place = exporter(event.place),
      },
    }

  elseif event.id == world.event.S_EVENT_ENGINE_SHUTDOWN  then
    return {
      time = event.time,
      event = {
        type = "engineShutdown",
        initiator = {initiator = typed_exporter(event.initiator)},
        place = exporter(event.place),
      },
    }

  elseif event.id == world.event.S_EVENT_PLAYER_ENTER_UNIT then
    return {
      time = event.time,
      event = {
        type = "playerEnterUnit",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    }

  elseif event.id == world.event.S_EVENT_PLAYER_LEAVE_UNIT then
    return {
      time = event.time,
      event = {
        type = "playerLeaveUnit",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    }

  -- S_EVENT_PLAYER_COMMENT: not implemented as apparently not used anymore

  elseif event.id == world.event.S_EVENT_SHOOTING_START then
    return {
      time = event.time,
      event = {
        type = "shootingStart",
        initiator = {initiator = typed_exporter(event.initiator)},
        weaponName = event.weapon_name,
      },
    }

  elseif event.id == world.event.S_EVENT_SHOOTING_END then
    return {
      time = event.time,
      event = {
        type = "shootingEnd",
        initiator = {initiator = typed_exporter(event.initiator)},
        weaponName = event.weapon_name,
      },
    }

  elseif event.id == world.event.S_EVENT_MARK_ADDED then
    local payload = {
      type = "markAdd",
      initiator = {initiator = typed_exporter(event.initiator)},
      id = event.idx,
      pos = GRPC.exporters.position(event.pos),
      text = event.text,
    }
    if event.groupID > -1 and event.groupID then
      payload.groupId = event.groupId
    elseif event.coalition > -1 and event.coalition then
      payload.coalition = event.coalition + 1  -- Increment for non zero-indexed gRPC enum
    end
    return {
      time = event.time,
      event = payload,
    }

  elseif event.id == world.event.S_EVENT_MARK_CHANGE then
    local payload = {
      type = "markChange",
      initiator = {initiator = typed_exporter(event.initiator)},
      id = event.idx,
      pos = GRPC.exporters.position(event.pos),
      text = event.text,
    }
    if event.groupID > -1 and event.groupID then
      payload.groupId = event.groupId
    elseif event.coalition > -1 and event.coalition then
      payload.coalition = event.coalition + 1 -- Increment for non zero-indexed gRPC enum
    end
    return {
      time = event.time,
      event = payload,
    }

  elseif event.id == world.event.S_EVENT_MARK_REMOVED then
    local payload = {
      type = "markRemove",
      initiator = {initiator = typed_exporter(event.initiator)},
      id = event.idx,
      pos = GRPC.exporters.position(event.pos),
      text = event.text,
    }
    if event.groupID > -1 and event.groupID then
      payload.groupId = event.groupId
    elseif event.coalition > -1 and event.coalition then
      payload.coalition = event.coalition + 1 -- Increment for non zero-indexed gRPC enum
    end
    return {
      time = event.time,
      event = payload,
    }

  elseif event.id == world.event.S_EVENT_KILL then
    return {
      time = event.time,
      event = {
        type = "kill",
        initiator = {initiator = typed_exporter(event.initiator)},
        weapon = exporter(event.weapon),
        target = {target = typed_exporter(event.target)},
        weaponName = event.weapon_name
      },
    }

  elseif event.id == world.event.S_EVENT_SCORE then
    return {
      time = event.time,
      event = {
        type = "score",
      },
    }

  elseif event.id == world.event.S_EVENT_UNIT_LOST then
    return {
      time = event.time,
      event = {
        type = "unitLost",
        initiator = {initiator = typed_exporter(event.initiator)},
      },
    }

  elseif event.id == world.event.S_EVENT_LANDING_AFTER_EJECTION then
    return {
      time = event.time,
      event = {
        type = "landingAfterEjection",
        initiator = {initiator = typed_exporter(event.initiator)},
        place = GRPC.exporters.position(event.place),
      },
    }

  -- S_EVENT_PARATROOPER_LENDING: apparently not used yet

  elseif event.id == world.event.S_EVENT_DISCARD_CHAIR_AFTER_EJECTION then
    return {
      time = event.time,
      event = {
        type = "discardChairAfterEjection",
        initiator = {initiator = typed_exporter(event.initiator)},
        target = {target = typed_exporter(event.target)},
      },
    }

  elseif event.id == world.event.S_EVENT_WEAPON_ADD then
    return {
      time = event.time,
      event = {
        type = "weaponAdd",
        initiator = {initiator = typed_exporter(event.initiator)},
        weaponName = event.weapon_name
      },
    }

  -- S_EVENT_TRIGGER_ZONE: apparently not used yet

  elseif event.id == world.event.S_EVENT_LANDING_QUALITY_MARK then
    return {
      time = event.time,
      event = {
        type = "landingQualityMark",
        initiator = {initiator = typed_exporter(event.initiator)},
        place = exporter(event.place),
        comment = event.comment
      },
    }

  -- S_EVENT_BDA: apparently not used yet
  -- S_EVENT_MAX: assumingly an end marker for the events enum and thus not a real event

  else
    GRPC.logWarning("Skipping unimplemented event id "..tostring(event.id))
    return nil
  end
end
