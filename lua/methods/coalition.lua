--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_coalition
--

local coalition = coalition
local GRPC = GRPC

GRPC.methods.getGroups = function(params)
  -- https://wiki.hoggitworld.com/view/DCS_func_getGroups
  local groups = coalition.getGroups(params.coalition, params.category)

  local result = {}
  for i, group in ipairs(groups) do
    result[i] = GRPC.exporters.group(group)
  end

  return GRPC.success({groups = result})
end
