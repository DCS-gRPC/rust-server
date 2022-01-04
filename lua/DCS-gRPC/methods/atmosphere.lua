--
-- RPC atmosphere actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_atmosphere
--

GRPC.methods.getWind = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)

  return GRPC.success(GRPC.exporters.vector(atmosphere.getWind(point)))
end

GRPC.methods.getWindWithTurbulence = function(params)
  local point = coord.LLtoLO(params.position.lat, params.position.lon, params.position.alt)

  return GRPC.success(GRPC.exporters.vector(atmosphere.getWindWithTurbulence(point)))
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