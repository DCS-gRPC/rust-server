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

  local category = Object.getCategory(object)-- change for DCS API fixes in getcategory()
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

GRPC.methods.getDrawArgumentValue = function (params)
  -- https://wiki.hoggitworld.com/view/DCS_func_getDrawArgumentValue
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit does not exist")
  end

  return GRPC.success({
    value = unit:getDrawArgumentValue(params.argument)
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

GRPC.methods.getUnitById = function(params)
  local unit = Unit.getByName(Unit.getName({ id_ = params.id }))
  if unit == nil then
    return GRPC.errorNotFound("unit with id `" .. tostring(params.id) .. "` does not exist")
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

GRPC.methods.getSensors = function(params)
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.errorNotFound("unit does not exist")
  end

  local s = unit:getSensors()
  local categories = {}

  if s == nil then
    return GRPC.success({ sensors = categories })
  end

  -- include category 0 (optical) by iterating and sorting all keys
  local catIndices = {}
  for k, _ in pairs(s) do
    table.insert(catIndices, k)
  end
  table.sort(catIndices, function(a, b)
    if type(a) == "number" and type(b) == "number" then return a < b end
    return tostring(a) < tostring(b)
  end)

  for _, catIndex in ipairs(catIndices) do
    local list = s[catIndex]
    local cat = { category = catIndex, sensors = {} }

    for i, sensor in ipairs(list or {}) do
      local out = {
        type = sensor.type,
        typeName = sensor.typeName,
      }

      -- Prefer explicit type ids when present: 0=Optical, 1=Radar, 2=IRST, 3=RWR
      if sensor.type == 1 then
        -- Radar
        local dda = sensor.detectionDistanceAir or {}
        local radar = {
          detectionDistanceAir = {
            upperHemisphere = {
              tailOn = dda.upperHemisphere and dda.upperHemisphere.tailOn,
              headOn = dda.upperHemisphere and dda.upperHemisphere.headOn,
            },
            lowerHemisphere = {
              tailOn = dda.lowerHemisphere and dda.lowerHemisphere.tailOn,
              headOn = dda.lowerHemisphere and dda.lowerHemisphere.headOn,
            },
          }
        }
        out.radar = radar
        out.sensor = { radar = radar }
      elseif sensor.type == 2 then
        -- IRST
        local irst = {
          detectionDistanceIdle = sensor.detectionDistanceIdle,
          detectionDistanceAfterburner = sensor.detectionDistanceAfterburner,
          detectionDistanceMaximal = sensor.detectionDistanceMaximal,
        }
        out.irst = irst
        out.sensor = { irst = irst }
      elseif sensor.type == 0 then
        -- Optical (generic EO/TV)
        out.optical = {}
        out.sensor = { optical = {} }
      elseif sensor.type == 3 then
        -- RWR
        out.rwr = {}
        out.sensor = { rwr = {} }
      elseif sensor.detectionDistanceAir ~= nil then
        local dda = sensor.detectionDistanceAir
        local radar = {
          detectionDistanceAir = {
            upperHemisphere = {
              tailOn = dda.upperHemisphere and dda.upperHemisphere.tailOn,
              headOn = dda.upperHemisphere and dda.upperHemisphere.headOn,
            },
            lowerHemisphere = {
              tailOn = dda.lowerHemisphere and dda.lowerHemisphere.tailOn,
              headOn = dda.lowerHemisphere and dda.lowerHemisphere.headOn,
            },
          }
        }
        out.radar = radar
        out.sensor = { radar = radar }
      elseif sensor.detectionDistanceIdle ~= nil
        or sensor.detectionDistanceAfterburner ~= nil
        or sensor.detectionDistanceMaximal ~= nil then
        local irst = {
          detectionDistanceIdle = sensor.detectionDistanceIdle,
          detectionDistanceAfterburner = sensor.detectionDistanceAfterburner,
          detectionDistanceMaximal = sensor.detectionDistanceMaximal,
        }
        out.irst = irst
        out.sensor = { irst = irst }
      end

      cat.sensors[i] = out
    end

    categories[#categories + 1] = cat
  end

  return GRPC.success({ sensors = categories })
end