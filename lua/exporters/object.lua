--
-- Converts DCS tables in the Object hierarchy into tables suitable for
-- serialization into GRPC responses
-- Each exporter has an equivalent .proto Message defined and they must
-- be kept in sync
--

local GRPC = GRPC
local coord = coord

local function toLatLonPosition(pos)
  local lat, lon, alt = coord.LOtoLL(pos)
  return {
    lat = lat,
    lon = lon,
    alt = alt,
  }
end

GRPC.exporters.unit = function(unit)
  return {
    id = tonumber(unit:getID()),
    name = unit:getName(),
    callsign = unit:getCallsign(),
    coalition = unit:getCoalition(),
    type = unit:getTypeName(),
    position = toLatLonPosition(unit:getPoint()),
    playerName = unit:getPlayerName()
  }
end

GRPC.exporters.weapon = function(weapon)
  return {
    id = tonumber(weapon:getName()),
    type = weapon:getTypeName(),
    position = toLatLonPosition(weapon:getPoint()),
  }
end

GRPC.exporters.static = function(static)
  return {}
end

GRPC.exporters.airbase = function(airbase)
  local a = {
    name = airbase:getName(),
    callsign = airbase:getCallsign(),
    coalition = airbase:getCoalition(),
    category = airbase:getDesc()['category'],
    displayName = airbase:getDesc()['displayName'],
    position = toLatLonPosition(airbase:getPoint())
  }

  if airbase:getUnit() then
    a.id = tonumber(airbase:getUnit():getID())
  end

  return a
end

GRPC.exporters.scenery = function(scenery)
  return {}
end

GRPC.exporters.cargo = function(cargo)
  return {}
end

GRPC.exporters.object = function(object)
  return {}
end