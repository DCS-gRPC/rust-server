--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_Class_Unit
--

local Unit = Unit
local Object = Object
local next = next 

GRPC.methods.getRadar = function(params)
  local unit = Unit.getByName(params.name)
  if unit == nil then
    return GRPC.error("Could not find unit with name '" .. params.name .. "'")
  end
    
  local active, object = unit:getRadar() 
  
  if object == nil then
    env.info("[GRPC]" .. params.name .. " has no radar target")
    return GRPC.success({   
      active = active
    })
  end
  
  env.info("[GRPC]" .. params.name .. "'s Target is " .. object:getCallsign() .. ", ID: " .. object:getID() .. ", Category: " .. object:getCategory() )
   
  local category = object:getCategory()
  local grpc_hash = {}
  local object_hash = {}
  
  if(category == Object.Category.UNIT) then
    -- TODO, fill out the hash for this object
    grpc_hash["unit"] = object_hash 
  elseif(category == Object.Category.WEAPON) then
    -- TODO, fill out the hash for this object
    grpc_hash["weapon"] = object_hash 
  elseif(category == Object.Category.STATIC) then
    -- TODO, fill out the hash for this object
    grpc_hash["static"] = object_hash 
  elseif(category == Object.Category.BASE) then
    -- TODO, fill out the hash for this object
    grpc_hash["airbase"] = object_hash 
  elseif(category == Object.Category.SCENERY) then
    -- TODO, fill out the hash for this object
    grpc_hash["scenery"] = object_hash 
  elseif(category == Object.Category.Cargo) then
    -- TODO, fill out the hash for this object
    grpc_hash["cargo"] = object_hash 
  else
    env.info("[GRPC] Could not determine object category of object with ID: " .. object:getID() .. ", Category: " .. category)   
    grpc_hash["object"] = {}   
  end
  
  return GRPC.success({   
    active = active,
    target = grpc_hash 
  })
end