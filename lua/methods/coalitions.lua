--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_coalition
--

local GRPC = GRPC
local coalition = coalition

GRPC.methods.getPlayers = function(params)
  local units = coalition.getPlayers(params.coalition)

  result = {}

  for i, unit in ipairs(units) do
    result[i] = GRPC.exporters.unit(unit)
  end
  return GRPC.success({units = result})

end