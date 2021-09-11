local GRPC = GRPC
local unit = unit
local group = group
local AI = AI

local group_option_category = {}
group_option_category[1] = "Air"
group_option_category[2] = "Ground"
group_option_category[3] = "Naval"

GRPC.methods.setAlarmState = function(params)
  local obj

  if params.name.groupName then
    obj = Group.getByName(params.name.groupName)
  elseif  params.name.unitName then
    obj = Unit.getByName(params.name.unitName)
  else
    return GRPC.errorInvalidArgument("No Group or Unit name provided")
  end

  if obj == nil then
    return GRPC.errorNotFound("Could not find group or unit with provided name")
  end

  local controller = obj:getController()
  local category_id = obj:getCategory()

  local state_id = AI['Option'][group_option_category[category_id]]['id']['ALARM_STATE']

  controller:setOption(state_id, params.alarmState)

  return GRPC.success(nil)
end
