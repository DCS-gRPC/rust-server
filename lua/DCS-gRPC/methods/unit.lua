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
    grpcTable.unit = GRPC.exporters.unit(object)
  elseif(category == Object.Category.WEAPON) then
    grpcTable.weapon = GRPC.exporters.weapon(object)
  elseif(category == Object.Category.STATIC) then
    grpcTable.static = GRPC.exporters.static(object)
  elseif(category == Object.Category.BASE) then
    grpcTable.airbase = GRPC.exporters.airbase(object)
  elseif(category == Object.Category.SCENERY) then
    grpcTable.scenery = GRPC.exporters.scenery(object)
  elseif(category == Object.Category.Cargo) then
    grpcTable.cargo = GRPC.exporters.cargo(object)
  else
    GRPC.logWarning(
      "Could not determine object category of object with ID: " .. object:getID()
        .. ", Category: " .. category
    )
    grpcTable.object = GRPC.exporters.object(object)
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

  return GRPC.success({
    time = timer.getTime(),
    rawTransform = GRPC.exporters.rawTransform(unit),
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
  return GRPC.success({})
end

GRPC.methods.getUnit = function(params)
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit `" .. tostring(params.name) .. "` does not exist")
  end

  return GRPC.success({unit = GRPC.exporters.unit(unit)})
end

GRPC.methods.unitDestroy = function(params)
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit `" .. tostring(params.name) .. "` does not exist")
  end

  unit:destroy()
  return GRPC.success({})
end
