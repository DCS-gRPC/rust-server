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

--local altitudeType = {
--  [1] = "BARO",
--  [2] = "RADIO",
--  BARO = 1,
--  RADIO = 2
--}

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
  if params.groupCategory == 0 then
    return GRPC.errorInvalidArgument("group category must be specified")
  end
  if params.country_id == 0 or params.country_id == 15 then
    return GRPC.errorInvalidArgument("invalid country code")
  end

  local template = createGroundGroupTemplate(params.template.groundTemplate)

  coalition.addGroup(params.country - 1, params.groupCategory - 1, template) -- Decrement for non zero-indexed gRPC enum

  return GRPC.success({group = GRPC.exporters.group(Group.getByName(template.name))})
end

GRPC.methods.getStaticObjects = function(params)
  local result = {}
  for _, coalitionId in pairs(coalition.side) do
    if params.coalition == 0 or params.coalition - 1 == coalitionId then -- Decrement for non zero-indexed gRPC enum
      local staticObjects = coalition.getStaticObjects(coalitionId)

      for _, staticObject in ipairs(staticObjects) do
        table.insert(result, GRPC.exporters.static(staticObject))
      end
    end
  end

  return GRPC.success({statics = result})
end

GRPC.methods.addStaticObject = function(params)
  if params.name == nil or params.name == "" then
    return GRPC.errorInvalidArgument("name not supplied")
  end
  if params.type == nil or params.type == "" then
    return GRPC.errorInvalidArgument("type not supplied")
  end
  if params.country_id == 0 or params.country_id == 15 then
    return GRPC.errorInvalidArgument("invalid country code")
  end

  if params.position == nil then
    return GRPC.errorInvalidArgument("provide position")
  end

  local pos = coord.LLtoLO(params.position.lat, params.position.lon)
  local staticTemplate = {
    name = params.name,
    type = params.type,
    heading = math.rad(params.heading),
    dead = params.dead,
    x = pos.x,
    y = pos.z,
  }

  if params.score ~= nil then
    staticTemplate.rate = params.score
  end

  if params.cargoMass > 0 then
    staticTemplate.canCargo = true
    staticTemplate.mass = params.cargoMass
  end

  coalition.addStaticObject(params.country - 1, staticTemplate)

  return GRPC.success({ name = params.name })
end

GRPC.methods.addLinkedStatic = function(params)
  if params.name == nil or params.name == "" then
    return GRPC.errorInvalidArgument("name not supplied")
  end
  if params.type == nil or params.type == "" then
    return GRPC.errorInvalidArgument("type not supplied")
  end
  if params.country_id == 0 or params.country_id == 15 then
    return GRPC.errorInvalidArgument("invalid country code")
  end
  if params.unit == nil or params.unit == "" then
    return GRPC.errorInvalidArgument("provide the unit to ")
  end

  local unit = Unit.getByName(params.unit)
  if unit == nil then
    return GRPC.errorNotFound("offset unit name not found")
  end

  -- we still need to supply a position to ED API, so current linked unit is good enough
  local pos = unit:getPoint()

  local staticTemplate = {
    name = params.name,
    type = params.type,
    x = pos.x,
    y = pos.z,
    dead = params.dead,
    linkOffset = true,
    linkUnit = unit:getID(),
    offsets = {
      x = params.x,
      y = params.y,
      angle = math.rad(params.angle),
    }
  }

  if params.score ~= nil then
    staticTemplate.rate = params.score
  end

  coalition.addStaticObject(params.country - 1, staticTemplate)

  return GRPC.success({ name = params.name })
end

GRPC.methods.getGroups = function(params)
  local result = {}
  for _, c in pairs(coalition.side) do
    if params.coalition == 0 or params.coalition - 1 == c then -- Decrement for non zero-indexed gRPC enum
      -- https://wiki.hoggitworld.com/view/DCS_func_getGroups
      local getFilteredGroups = function()
        if params.category == 0 then
          return coalition.getGroups(c)
        end
        return coalition.getGroups(c, params.category - 1)
      end
      local groups = getFilteredGroups()

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

GRPC.methods.getPlayerUnits = function(params)
  local units = coalition.getPlayers(params.coalition - 1) -- Decrement for non zero-indexed gRPC enum
  local result = {}
  for i, unit in ipairs(units) do
    result[i] = GRPC.exporters.unit(unit)
  end
  return GRPC.success({units = result})
end
