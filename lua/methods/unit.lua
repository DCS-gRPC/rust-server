--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_Class_Unit
--

local Unit = Unit
local Object = Object
local GRPC = GRPC

-- Convert a lua table into a lua syntactically correct string
function table_to_string(tbl)
    local result = "{"
    for k, v in pairs(tbl) do
        -- Check the key type (ignore any numerical keys - assume its an array)
        if type(k) == "string" then
            result = result.."[\""..k.."\"]".."="
        end

        -- Check the value type
        if type(v) == "table" then
            result = result..table_to_string(v)
        elseif type(v) == "boolean" then
            result = result..tostring(v)
        else
            result = result.."\""..v.."\""
        end
        result = result..","
    end
    -- Remove leading commas from the result
    if result ~= "" then
        result = result:sub(1, result:len()-1)
    end
    return result.."}"
end


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
    GRPC.logWarning("Could not determine object category of object with ID: " .. object:getID() .. ", Category: " .. category)
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

  env.info("[GRPC]".. table_to_string(desc.attributes))

  local attrs = {}

  for i, v in pairs(desc.attributes) do
    table.insert(attrs, i)
  end

  return GRPC.success({
    attributes = attrs
  })
end
