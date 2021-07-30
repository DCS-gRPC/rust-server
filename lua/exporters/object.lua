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

GRPC.exporters.markPanel = function(markPanel)
  local mp = {
    id = markPanel.idx, 
    time = markPanel.time,
    initiator = GRPC.exporters.unit(markPanel.initiator),
    text = markPanel.text,
    position = toLatLonPosition(markPanel.pos)
  }

  if (markPanel.coalition >= 0 and markPanel.coalition <= 2) then
  	mp["coalition"] = markPanel.coalition;
  end

  if (markPanel.groupID > 0) then
  	mp["groupId"] = markPanel.groupID;
  end

  return mp
end
