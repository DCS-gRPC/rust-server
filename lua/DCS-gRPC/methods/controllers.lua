local group_option_category = {}
group_option_category[1] = "Air"
group_option_category[2] = "Ground"
group_option_category[3] = "Naval"

GRPC.methods.setAlarmState = function(params)
  if params.alarmState == 0 then
    return GRPC.errorInvalidArgument("alarm_state cannot be unspecified (0)")
  end

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

  controller:setOption(state_id, params.alarmState - 1)

  return GRPC.success({})
end

GRPC.methods.getDetectedTargets = function(params)
  local unit = Unit.getByName(params.unitName)
  if unit == nil then
    return GRPC.errorNotFound("Could not find radar unit with name '" .. params.unitName .. "'")
  end

  local controller = Unit.getController(unit)
  local targets
  if params.detectionType == 0 or params.detectionType == nil then
    targets = controller:getDetectedTargets()
  else
    -- int value from https://wiki.hoggitworld.com/view/DCS_func_getDetectedTargets
    targets = controller:getDetectedTargets(params.detectionType)
  end

  if targets == nil then
    return GRPC.success({
      contacts = targets
    })
  end

  local results = {}

  for i, contact in ipairs(targets) do
    local category = Object.getCategory(contact.object)

    if category == nil then
      return GRPC.errorNotFound("Could not find target with id '" .. contact.object:getID() .. "'")
    end

    local result = {
      distance = contact.distance,
      id = contact.object.id_,
      visible = contact.visible,
      target = {}
    }

    --If target is a unit
    if category == 1 then
      if params.includeObject == true then
        result.target.unit = GRPC.exporters.unit( contact.object )
      else
        result.target.object = GRPC.exporters.unknown( contact.object )
      end
    end
    --If target is a weapon
    if category == 2 then
      if params.includeObject == true then
        result.target.weapon = GRPC.exporters.weapon( contact.object )
      else
        result.target.object = GRPC.exporters.unknown( contact.object )
      end
    end

    results[i] = result
  end

  return GRPC.success({
    contacts = results
  })
end