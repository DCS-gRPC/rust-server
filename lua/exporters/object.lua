--
-- Converts DCS tables in the Object hierarchy into tables suitable for
-- serialization into GRPC responses
-- Each exporter has an equivalent .proto Message defined and they must
-- be kept in sync
--

GRPC.exporters.unit = function(unit)
  local vector = unit:getVelocity()

  local heading = math.deg(math.atan2(vector.z, vector.x))
  if heading < 0 then
    heading = heading + 360
  end

  local speed = math.sqrt((vector.x)^2+(vector.z)^2)

  return {
    id = tonumber(unit:getID()),
    name = unit:getName(),
    callsign = unit:getCallsign(),
    coalition = unit:getCoalition() + 1, -- Increment for non zero-indexed gRPC enum
    type = unit:getTypeName(),
    position = GRPC.toLatLonPosition(unit:getPoint()),
    playerName = Unit.getPlayerName(unit),
    groupName = Unit.getGroup(unit):getName(),
    numberInGroup = unit:getNumber(),
    heading = heading,
    speed = speed
  }
end

GRPC.exporters.group = function(group)
  return {
    id = tonumber(group:getID()),
    name = group:getName(),
    coalition = group:getCoalition() + 1, -- Increment for non zero-indexed gRPC enum
    category = group:getCategory(),
  }
end

GRPC.exporters.weapon = function(weapon)
  return {
    id = tonumber(weapon:getName()),
    type = weapon:getTypeName(),
    position = GRPC.toLatLonPosition(weapon:getPoint()),
  }
end

GRPC.exporters.static = function()
  return {}
end

GRPC.exporters.airbase = function(airbase)
  local a = {
    name = airbase:getName(),
    callsign = airbase:getCallsign(),
    coalition = airbase:getCoalition() + 1, -- Increment for non zero-indexed gRPC enum
    category = airbase:getDesc()['category'],
    displayName = airbase:getDesc()['displayName'],
    position = GRPC.toLatLonPosition(airbase:getPoint())
  }

  if airbase:getUnit() then
    a.id = tonumber(airbase:getUnit():getID())
  end

  return a
end

GRPC.exporters.scenery = function()
  return {}
end

GRPC.exporters.cargo = function()
  return {}
end

-- every object, even an unknown one, should at least have getName implemented as it is
-- in the base object of the hierarchy
-- https://wiki.hoggitworld.com/view/DCS_Class_Object
GRPC.exporters.unknown = function(object)
  return {
    name = object:getName(),
  }
end

GRPC.exporters.markPanel = function(markPanel)
  local mp = {
    id = markPanel.idx,
    time = markPanel.time,
    initiator = GRPC.exporters.unit(markPanel.initiator),
    text = markPanel.text,
    position = GRPC.toLatLonPosition(markPanel.pos)
  }

  if (markPanel.coalition >= 0 and markPanel.coalition <= 2) then
    mp["coalition"] = markPanel.coalition + 1; -- Increment for non zero-indexed gRPC enum
  end

  if (markPanel.groupID > 0) then
    mp["groupId"] = markPanel.groupID;
  end

  return mp
end
