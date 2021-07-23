--
-- RPC atmosphere actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_atmosphere
--

local atmosphere = atmosphere
local coord = coord
local GRPC = GRPC

local function convertWindResults(windvec3)
  local direction = math.deg(math.atan2(windvec3.z, windvec3.x))
  
  if direction < 0 then
    direction = direction + 360
  end
  
  -- Convert TO direction to FROM direction. 
  if direction > 180 then
    direction = direction-180
  else
    direction = direction+180
  end
  
  -- Calc 2D strength.
  local strength = math.sqrt((windvec3.x)^2+(windvec3.z)^2)

  return  {
    heading = direction, -- Western style. Heading the wind is coming FROM
    strength = strength -- m/s
  }
end

GRPC.methods.getWind = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)
     
  return GRPC.success(convertWindResults(atmosphere.getWind(point)))
end

GRPC.methods.getWindWithTurbulence = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)
     
  return GRPC.success(convertWindResults(atmosphere.getWindWithTurbulence(point)))
end

GRPC.methods.getTemperatureAndPressure = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)
  
  local temperature, pressure = atmosphere.getTemperatureAndPressure(point)

  return GRPC.success(
    {
      temperature = temperature, -- Kelvin
      pressure = pressure -- Pascals
    }
  )
end