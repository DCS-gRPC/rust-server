--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_Class_Unit
--

GRPC.methods.getRadar = function(params)
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("Could not find unit with name '" .. params.name .. "'")
  end

  local active, object = unit:getRadar()

  if object == nil then
    return GRPC.success({
      active = active
    })
  end

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
    GRPC.logWarning(
      "Could not determine object category of object with ID: " .. object:getID()
        .. ", Category: " .. category
    )
    grpcTable["object"] = GRPC.exporters.object(object)
  end

  return GRPC.success({
    active = active,
    target = grpcTable
  })
end

GRPC.methods.getUnitPosition = function(params)
  -- https://wiki.hoggitworld.com/view/DCS_func_getByName
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit does not exist")
  end

  return GRPC.success({
    -- https://wiki.hoggitworld.com/view/DCS_func_getPoint
    position = GRPC.exporters.position(unit:getPoint()),
  })
end

GRPC.methods.getUnitTransform = function(params)
  -- https://wiki.hoggitworld.com/view/DCS_func_getByName
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit does not exist")
  end

    -- https://wiki.hoggitworld.com/view/DCS_func_getPosition
  local position = unit:getPosition()
  local coords = GRPC.exporters.position(position.p)
  local coordsNorth = coord.LLtoLO(coords.lat + 1, coords.lon)

  -- Response does not exactly match what the gRPC server will return since the gRPC response
  -- will be calculated based on the information returned here.
  return GRPC.success({
    position = coords,
    u = position.p.z,
    v = position.p.x,
    positionNorth = GRPC.exporters.vector(coordsNorth),
    orientation = {
      forward = GRPC.exporters.vector(position.x),
      right = GRPC.exporters.vector(position.z),
      up = GRPC.exporters.vector(position.y),
    },
    velocity = GRPC.exporters.vector(unit:getVelocity()),
    time = timer.getTime(),
  })
end

GRPC.methods.getUnitPlayerName = function(params)
  -- https://wiki.hoggitworld.com/view/DCS_func_getByName
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit does not exist")
  end

  return GRPC.success({
    -- https://wiki.hoggitworld.com/view/DCS_func_getPlayerName
    playerName = unit:getPlayerName(),
  })
end

GRPC.methods.getUnitDescriptor = function(params)
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit does not exist")
  end

  local desc = unit:getDesc()
  local attrs = {}
  for i in pairs(desc.attributes) do
    table.insert(attrs, i)
  end

  return GRPC.success({
    attributes = attrs
  })
end

GRPC.methods.setEmission = function(params)
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit does not exist")
  end  unit:enableEmission(params.emitting)
  return GRPC.success(nil)
end

GRPC.methods.getUnit = function(params)
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit `" .. tostring(params.name) .. "` does not exist")
  end

  return GRPC.success({unit = GRPC.exporters.unit(unit)})
end

-- This method is only used by the unit stream. It returns the subset of `getUnit` that can
-- change (like position, velocity, ...). It is not exposed to the gRPC service for now.
GRPC.methods.getUnitUpdate = function(params)
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit `" .. tostring(params.name) .. "` does not exist")
  end

  return GRPC.success({
    position = GRPC.exporters.position(unit:getPoint()),
    velocity = GRPC.exporters.vector(unit:getVelocity()),
  })
end
