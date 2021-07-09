--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_Class_Unit
--

local Unit = Unit
local Object = Object
local GRPC = GRPC

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
    env.info("[GRPC] Could not determine object category of object with ID: " .. object:getID() .. ", Category: " .. category)
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
    position = GRPC.toLatLonPosition(unit:getPoint()),
  })
end
