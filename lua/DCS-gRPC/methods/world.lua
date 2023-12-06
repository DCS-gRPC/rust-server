--
-- RPC world actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_world
--

local world = world
local coalition = coalition
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

  for i, airbase in ipairs(data) do
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