--
-- RPC coalition actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_coalition
--

local GRPC = GRPC
local coalition = coalition

local skill = {
  [0] = "Random",
  [1] = "Average",
  [2] = "Good",
  [3] = "High",
  [4] = "Excellent",
  [5] = "Player",
  Random = 0,
  Average = 1,
  Good = 2,
  High = 3,
  Excellent = 4,
  Player = 5
}

local altitudeType = {
  [1] = "BARO",
  [2] = "RADIO",
  BARO = 1,
  RADIO = 2
}

local buildAirplanePoints = function(points)
  local builtPoints = {}
  for _, pointData in pairs(points) do
    local pointVec3
    if type(pointData.place) == "string" then
      if Airbase.getByName(pointData.place) then
        pointVec3 = Airbase.getByName(pointData.place):getPoint()
      elseif trigger.misc.getZone(pointData.place) then
        pointVec3 = trigger.misc.getZone(pointData.place).point
      end
    elseif type(pointData.place) == "table" then
      pointVec3 = coord.LLtoLO(pointData.place.lat, pointData.place.lon)
    end
    builtPoints[#builtPoints+1] = {
      alt = pointData.alt,
      x = pointVec3.x,
      y = pointVec3.z,
      type = pointData.type,
      eta = 0,
      eta_locked = true,
      alt_type = altitudeType[altitudeType.alt_type],
      formation_template = "",
      speed = pointData.speed,
      action = pointData.action,
      task = {
        id = "ComboTask",
        params = {
          tasks = {}
        }
      }
    }
  end
  -- here we ammend the first point to allow for spawns from airbases if it isn't an airspawn
  if Airbase.getByName(points[1].place) then
    if points[1].type ~= "Turning Point" and points[1].action ~= "Turning Point" then
      local ab = Airbase.getByName(points[1].place)
      local abId = Airbase.getID(ab)
      local abCat = Airbase.getDesc(ab).category
      if abCat == 0 then -- Airbase.Category.AIRDROME
        builtPoints[1].airdromeId = abId
      elseif abCat == 2 then -- Airbase.Category.HELIPAD
        builtPoints[1].linkUnit = abId
        builtPoints[1].helipadId = abId -- why its named helipad i dont know
      end
    end
  end
  return builtPoints
end

local createPlaneGroupUnitsTemplate = function(unitListTemplate)
    local units = {}
    for i, unitTemplate in pairs(unitListTemplate) do
      local pointVec3
      if type(unitListTemplate.place) == "string" then
        if Airbase.getByName(unitListTemplate.place) then
          pointVec3 = Airbase.getByName(unitListTemplate.place):getPoint()
        elseif trigger.misc.getZone(unitListTemplate.place) then
          pointVec3 = trigger.misc.getZone(unitListTemplate.place).point
        end
      elseif type(unitListTemplate.place) == "table" then
        pointVec3 = coord.LLtoLO(unitListTemplate.place.lat, unitListTemplate.place.lon)
      end
      local fuel = Unit.getDescByName(unitListTemplate.type).fuelMassMax -- needed incase no payload table is applied
      units[i] = {
        name = unitTemplate.unitName, -- or unitTemplate.name.."-"..i
        type = unitListTemplate.type,
        x = pointVec3.x,
        y = pointVec3.z,
        alt = unitListTemplate.alt,
        alt_type = altitudeType[unitTemplate.alt_type],
        speed = unitListTemplate.speed,
        payload = unitTemplate.payload or {
        ["pylons"] = {},
        ["fuel"] = fuel,
        ["flare"] = 0,
        ["chaff"] = 0,
        ["gun"] = 0,
        },
        parking = unitTemplate.parking or nil,
        parking_id = unitTemplate.parking_id or nil,
        callsign = unitTemplate.callsign or nil,
        skill = skill[unitListTemplate.skill],
        livery_id = unitTemplate.livery_id or nil,
      }
    end
    return units
  end

local createPlaneGroupTemplate = function(planeGroupTemplate)
  local groupTable = {
    name = planeGroupTemplate.groupName,
    task = planeGroupTemplate.task,
    route = {
      points = buildAirplanePoints(planeGroupTemplate.points)
    }
  }
  groupTable.units = createPlaneGroupUnitsTemplate(planeGroupTemplate.units)
  if planeGroupTemplate.group_id ~= nil then
    groupTable['groupId'] = planeGroupTemplate.group_id
  end
  if planeGroupTemplate.hidden ~= nil then
    groupTable['hidden'] = planeGroupTemplate.hidden
  end
  if planeGroupTemplate.late_activation ~= nil then
    groupTable['lateActivation'] = planeGroupTemplate.late_activation
  end
  if planeGroupTemplate.start_time ~= nil and planeGroupTemplate.start_time > 0 then
    groupTable['start_time'] = planeGroupTemplate.start_time
  end
  if planeGroupTemplate.visible ~= nil then
    groupTable['visible'] = planeGroupTemplate.visible
  end
  return groupTable
end

local createGroundUnitsTemplate = function(unitListTemplate)
  local units = {}

  for _, unitTemplate in pairs(unitListTemplate) do
    local pos = coord.LLtoLO(unitTemplate.position.lat, unitTemplate.position.lon)
    local unit = {
      name = unitTemplate.name,
      type = unitTemplate.type,
      x = pos.x,
      y = pos.z,
      transportable = { randomTransportable = false },
      skill = skill[unitTemplate.skill],
      heading = unitTemplate.heading,
      playerCanDrive = true
    }
    table.insert(units, unit)
  end

  return units
end

local createGroundGroupTemplate = function(groupTemplate)
  local pos = coord.LLtoLO(groupTemplate.position.lat, groupTemplate.position.lon)

  local groupTable = {
    name = groupTemplate.name,
    route = {
      spans = {},
      points = {
        {
          x = pos.x,
          y = pos.z,
          type = "Turning Point",
          eta = 0,
          eta_locked = true,
          alt_type = "BARO",
          formation_template = "",
          speed = 0,
          action = "Off Road",
          task = {
            id = "ComboTask",
            params = {
                tasks = {}
            }
          }
        }
      }
    },
    task = "Ground Nothing",
    taskSelected = true,
    tasks = {},
    uncontrollable = false,
    units = createGroundUnitsTemplate(groupTemplate.units),
    visible = false,
    x = pos.x,
    y = pos.z
  }

  if groupTemplate.group_id ~= nil then
    groupTable['groupId'] = groupTemplate.group_id
  end
  if groupTemplate.hidden ~= nil then
    groupTable['hidden'] = groupTemplate.hidden
  end
  if groupTemplate.late_activation ~= nil then
    groupTable['lateActivation'] = groupTemplate.late_activation
  end
  if groupTemplate.start_time ~= nil and groupTemplate.start_time > 0 then
    groupTable['start_time'] = groupTemplate.start_time
  end
  if groupTemplate.visible ~= nil then
    groupTable['visible'] = groupTemplate.visible
  end

  return groupTable
end

GRPC.methods.addGroup = function(params)
  if params.country_id == 0 or params.country_id == 15 then
    return GRPC.errorInvalidArgument("invalid country code")
  end
  local template
  if params.template.type == "Airplane" then
    template = createPlaneGroupTemplate(params.template.airplaneTemplate)
  elseif params.template.type == "Ground" then
    template = createGroundGroupTemplate(params.template.groundTemplate)
  end
  coalition.addGroup(params.country - 1, params.groupCategory, template) -- Decrement for non zero-indexed gRPC enum

  return GRPC.success({group = GRPC.exporters.group(Group.getByName(template.name))})
end

GRPC.methods.getGroups = function(params)
  local result = {}
  for _, c in pairs(coalition.side) do
    if params.coalition == 0 or params.coalition - 1 == c then -- Decrement for non zero-indexed gRPC enum
      -- https://wiki.hoggitworld.com/view/DCS_func_getGroups
      local groups = coalition.getGroups(c, params.category)

      for _, group in ipairs(groups) do
        table.insert(result, GRPC.exporters.group(group))
      end
    end
  end

  return GRPC.success({groups = result})
end

-- This method should be called once per coalition per mission so using COALITION_ALL to save 2
-- API calls is not worth the extra code.
GRPC.methods.getBullseye = function(params)
  if params.coalition == 0 then
    return GRPC.errorInvalidArgument("a specific coalition must be chosen")
  end

  local referencePoint = coalition.getMainRefPoint(params.coalition - 1) -- Decrement for non zero-indexed gRPC enum

  return GRPC.success({
    position = GRPC.exporters.position(referencePoint)
  })
end

GRPC.methods.getPlayers = function(params)
  local units = coalition.getPlayers(params.coalition)
  local result = {}
  for i, unit in ipairs(units) do
    result[i] = GRPC.exporters.unit(unit)
  end
  return GRPC.success({units = result})
end
