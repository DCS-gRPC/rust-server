--
-- RPC world actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_world
--
-- luacheck: globals world coalition Object env coord

local world = world
local coalition = coalition
local Object = Object
local GRPC = GRPC

GRPC.methods.getAirbases = function(params)
  local data

  if params.coalition == 0 then
    data = world.getAirbases()
  else
    -- Yes, yes, this is in the world file but uses coalition. I plan
    -- to completely rejigger the organisation of these files when we
    -- have more APIs implemented and amore sane pattern presents
    -- itself. For the moment we are mostly following DCS organisation
    data = coalition.getAirbases(params.coalition - 1)  -- Decrement for non zero-indexed gRPC enum
  end

  local result = {}
  local unit

  for _, airbase in pairs(data) do
    if airbase:getDesc()['category'] == 2 then -- SHIP
        unit = airbase:getUnit()
        if unit then -- Unit object
            if unit:isExist() then -- Extant object
                if unit:getGroup() then -- Unit in group so can be exported
                    result[#result+1] = GRPC.exporters.airbase(airbase)
                end -- no group for unit, move to next object
            end -- unit no longer exists, move to next object
        end -- no unit, move to next object
    else -- Aerodrome or Helipad, so can be exported
        result[#result+1] = GRPC.exporters.airbase(airbase)
    end
  end
  return GRPC.success({airbases = result})
end

GRPC.methods.getMarkPanels = function()
  local markPanels = world.getMarkPanels()
  local result = {}

  for i, markPanel in ipairs(markPanels) do
    result[i] = GRPC.exporters.markPanel(markPanel)
  end

  return GRPC.success({markPanels = result})
end

GRPC.methods.getTheatre = function()
  return GRPC.success({theatre = env.mission.theatre})
end

-- https://wiki.hoggitworld.com/view/DCS_func_searchObjects
GRPC.methods.searchObjects = function(params)
  GRPC.logInfo("searchObjects: invoked")
  if params.categories == nil or #params.categories == 0 then
    return GRPC.errorInvalidArgument("categories must not be empty")
  end

  if params.volume == nil then
    return GRPC.errorInvalidArgument("volume is required")
  end

  local v = params.volume
  local volume

  -- Debug: log incoming volume keys
  do
    local keys = {}
    for k, _ in pairs(v or {}) do keys[#keys+1] = tostring(k) end
    GRPC.logInfo("searchObjects: volume keys="..table.concat(keys, ","))
  end

  -- If grpcui/serde wrapped oneof under a `shape` table, unwrap it
  if v and v.shape and type(v.shape) == "table" then
    v = v.shape
    local keys = {}
    for k, _ in pairs(v or {}) do keys[#keys+1] = tostring(k) end
    GRPC.logInfo("searchObjects: unwrapped shape; inner keys="..table.concat(keys, ","))
  end

  -- If grpcui sent a discriminator value, prefer it
  local discriminator = nil
  if params.volume and params.volume.shape and type(params.volume.shape) ~= "table" then
    if type(params.volume.shape) == "string" then
      discriminator = string.lower(params.volume.shape)
    elseif type(params.volume.shape) == "number" then
      -- Best-effort mapping (fallback only)
      local map = { [1] = "sphere", [2] = "box", [3] = "segment", [4] = "pyramid" }
      discriminator = map[params.volume.shape]
    end
    GRPC.logInfo("searchObjects: discriminator="..tostring(discriminator))
  end

  -- Detect sphere
  if (discriminator == "sphere")
    or v.sphere ~= nil
    or (
      v.center ~= nil and v.radius ~= nil and
      v.min == nil and v.from == nil and v.forward == nil
    )
  then
    local s = v.sphere or v
    local center = s.center
    local radius = s.radius
    if center == nil or radius == nil then
      return GRPC.errorInvalidArgument("sphere center and radius are required")
    end
    local lo = coord.LLtoLO(center.lat, center.lon)
    GRPC.logInfo("searchObjects: volume SPHERE radius="..tostring(radius))
    volume = {
      id = world.VolumeType.SPHERE,
      params = {
        point = { x = lo.x, y = center.alt or 0, z = lo.z },
        radius = radius,
      }
    }
  -- Detect box
  elseif (discriminator == "box") or v.box ~= nil or (v.min ~= nil and v.max ~= nil) then
    local b = v.box or v
    local min = b.min
    local max = b.max
    if min == nil or max == nil then
      return GRPC.errorInvalidArgument("box min and max are required")
    end
    local minLo = coord.LLtoLO(min.lat, min.lon)
    local maxLo = coord.LLtoLO(max.lat, max.lon)
    GRPC.logInfo("searchObjects: volume BOX")
    volume = {
      id = world.VolumeType.BOX,
      params = {
        min = { x = minLo.x, y = min.alt or 0, z = minLo.z },
        max = { x = maxLo.x, y = max.alt or 0, z = maxLo.z },
      }
    }
  -- Detect segment
  elseif (discriminator == "segment") or v.segment ~= nil or (v.from ~= nil and v.to ~= nil) then
    local seg = v.segment or v
    local from = seg.from
    local to = seg.to
    if from == nil or to == nil then
      return GRPC.errorInvalidArgument("segment from and to are required")
    end
    local fromLo = coord.LLtoLO(from.lat, from.lon)
    local toLo = coord.LLtoLO(to.lat, to.lon)
    GRPC.logInfo("searchObjects: volume SEGMENT")
    volume = {
      id = world.VolumeType.SEGMENT,
      params = {
        from = { x = fromLo.x, y = from.alt or 0, z = fromLo.z },
        to = { x = toLo.x, y = to.alt or 0, z = toLo.z },
      }
    }
  -- Detect pyramid
  elseif (discriminator == "pyramid")
    or v.pyramid ~= nil
    or (
      v.center ~= nil and v.forward ~= nil and v.right ~= nil and v.up ~= nil and
      v.length ~= nil and (
        v.halfAngleHorizontal ~= nil and v.halfAngleVertical ~= nil
      )
    )
  then
    local pv = v.pyramid or v
    if pv.center == nil or pv.length == nil or pv.halfAngleHorizontal == nil or pv.halfAngleVertical == nil then
      return GRPC.errorInvalidArgument("pyramid center, length and angles are required")
    end

    local fwd = pv.forward
    local up = pv.up
    local right = pv.right

    local function normalize(vec)
      local mag = math.sqrt((vec.x or 0)^2 + (vec.y or 0)^2 + (vec.z or 0)^2)
      if mag == 0 then return { x = 0, y = 1, z = 0 } end
      return { x = vec.x / mag, y = vec.y / mag, z = vec.z / mag }
    end

    if fwd == nil or right == nil or up == nil then
      return GRPC.errorInvalidArgument("pyramid requires forward/right/up vectors")
    end
    fwd = normalize(fwd)
    right = normalize(right)
    up = normalize(up)

    local centerLo = coord.LLtoLO(pv.center.lat, pv.center.lon)
    GRPC.logInfo("searchObjects: volume PYRAMID length="..tostring(pv.length))
    volume = {
      id = world.VolumeType.PYRAMID,
      params = {
        pos = {
          p = { x = centerLo.x, y = pv.center.alt or 0, z = centerLo.z },
          x = fwd,
          y = up,
          z = right,
        },
        length = pv.length,
        halfAngleHor = pv.halfAngleHorizontal,
        halfAngleVer = pv.halfAngleVertical,
      }
    }
  else
    GRPC.logWarning("searchObjects: unknown volume type; expected sphere/box/segment/pyramid")
    return GRPC.errorInvalidArgument("unknown volume type")
  end

  local result = {}
  local function handler(object, _)
    local cat = object:getCategory()
    if cat == Object.Category.UNIT then
      result[#result+1] = { target = { unit = GRPC.exporters.unit(object) } }
    elseif cat == Object.Category.WEAPON then
      result[#result+1] = { target = { weapon = GRPC.exporters.weapon(object) } }
    elseif cat == Object.Category.STATIC then
      result[#result+1] = { target = { static = GRPC.exporters.static(object) } }
    elseif cat == Object.Category.SCENERY then
      result[#result+1] = { target = { scenery = GRPC.exporters.scenery(object) } }
    elseif cat == Object.Category.BASE then
      result[#result+1] = { target = { airbase = GRPC.exporters.airbase(object) } }
    elseif cat == Object.Category.CARGO then
      result[#result+1] = { target = { cargo = GRPC.exporters.cargo() } }
    else
      result[#result+1] = { target = { unknown = GRPC.exporters.unknown(object) } }
    end
    return true
  end

  for _, cat in ipairs(params.categories) do
    GRPC.logInfo("searchObjects: calling world.searchObjects for category="..tostring(cat))
    world.searchObjects(cat, volume, handler, nil)
  end

  GRPC.logInfo("searchObjects: returning objects count="..tostring(#result))
  return GRPC.success({objects = result})
end