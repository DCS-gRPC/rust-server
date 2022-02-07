--
-- Converts DCS tables in the Object hierarchy into tables suitable for
-- serialization into GRPC responses
-- Each exporter has an equivalent .proto Message defined and they must
-- be kept in sync
--

local function mergeTable(t1, t2)
  for k,v in pairs(t2) do
    t1[k] = v
  end
  return t1
end

-- Convert DCS's unusual right-hand coordinate system where +x points north to a more common
-- left-hand coordinate system where +z points north (and +x points east).
GRPC.exporters.vector = function(v)
  return {
    x = v.z,
    y = v.y,
    z = v.x
  }
end

GRPC.exporters.position = function(pos)
  local lat, lon, alt = coord.LOtoLL(pos)
  return {
    lat = lat,
    lon = lon,
    alt = alt,
  }
end

local function exportObject(object, extendedObject)
  local vector = object:getVelocity()

  local heading = math.deg(math.atan2(vector.z, vector.x))
  if heading < 0 then
    heading = heading + 360
  end

  local speed = math.sqrt((vector.x)^2+(vector.z)^2)

  return mergeTable(extendedObject, {
    type = object:getTypeName(),
    name = object:getName(),
    category = object:getCategory() + 1, -- Increment for non zero-indexed gRPC enum
    position = GRPC.exporters.position(object:getPoint()),
    heading = heading,
    speed = speed,
  })
end

local function exportCoalitionObject(coalitionObject, extendedObject)
  return exportObject(coalitionObject, mergeTable(extendedObject, {
    coalition = coalitionObject:getCoalition() + 1, -- Increment for non zero-indexed gRPC enum
  }))
end

GRPC.exporters.unit = function(unit)
  return exportCoalitionObject(unit, {
    id = tonumber(unit:getID()),
    callsign = unit:getCallsign(),
    playerName = unit:getPlayerName(),
    groupName = unit:getGroup():getName(),
    numberInGroup = unit:getNumber(),
  })
end

GRPC.exporters.group = function(group)
  return {
    id = tonumber(group:getID()),
    name = group:getName(),
    coalition = group:getCoalition() + 1, -- Increment for non zero-indexed gRPC enum
    category = group:getCategory() + 1, -- Increment for non zero-indexed gRPC enum
  }
end

GRPC.exporters.weapon = function(weapon)
  return exportCoalitionObject(weapon, {
    id = tonumber(weapon:getName()),
  })
end

GRPC.exporters.static = function(static)
  return exportCoalitionObject(static, {
    id = tonumber(static:getID()),
  })
end

GRPC.exporters.airbase = function(airbase)
  local a = {
    callsign = airbase:getCallsign(),
    displayName = airbase:getDesc()['displayName'],
  }

  if airbase:getUnit() then
    a.id = tonumber(airbase:getUnit():getID())
  end

  return exportCoalitionObject(airbase, a)
end

GRPC.exporters.scenery = function(scenery)
  return exportObject(scenery, {
    position = GRPC.exporters.position(scenery:getPoint()),
  })
end

GRPC.exporters.cargo = function(cargo)
  -- we know cargo is a static... will extend later
  return GRPC.exporters.static(cargo)
end

-- every object, even an unknown one, should at least have getName implemented as it is
-- in the base object of the hierarchy
-- https://wiki.hoggitworld.com/view/DCS_Class_Object
GRPC.exporters.unknown = function(object)
  return exportObject(object, {})
end

GRPC.exporters.markPanel = function(markPanel)
  local mp = {
    id = markPanel.idx,
    time = markPanel.time,
    initiator = GRPC.exporters.unit(markPanel.initiator),
    text = markPanel.text,
    position = GRPC.exporters.position(markPanel.pos)
  }

  if (markPanel.coalition >= 0 and markPanel.coalition <= 2) then
    mp["coalition"] = markPanel.coalition + 1; -- Increment for non zero-indexed gRPC enum
  end

  if (markPanel.groupID > 0) then
    mp["groupId"] = markPanel.groupID;
  end

  return mp
end
