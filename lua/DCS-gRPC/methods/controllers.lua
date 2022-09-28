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
    return GRPC.errorNotFound("Could not find unit with name '" .. params.unitName .. "'")
  end

  local controller = Unit.getController(unit)
  local targets = controller:getDetectedTargets()

  if targets == nil then
    return GRPC.success({
      contacts = targets
    })
  end

  local results = {}

  for i, contact in ipairs(targets) do
    local category = Object.getCategory(contact.object)

    contact.id = contact.object.id_
    contact.target = {}

    if category == 1 then
      contact.name = Unit.getName(contact.object)
      contact.type = 1
      if params.includeObject == true then
        contact.unit = GRPC.exporters.unit( contact.object )
      end
    end
    if category == 2 then
      contact.name = Weapon.getName(contact.object)
      contact.type = 2
      if params.includeObject == true then
        contact.weapon = GRPC.exporters.weapon( contact.object )
      end
    end

    if contact.type == nil then
      return GRPC.errorNotFound("Could not find target with id '" .. contact.object.id_ .. "' as a Unit or Weapon")
    end

    local result = {
      distance = contact.distance,
      name = tostring(contact.name),
      id = contact.id,
      type = contact.type,
      visible = contact.visible,
      unit = contact.unit,
      weapon = contact.weapon,
    }

    results[i] = result
  end

  return GRPC.success({
    contacts = results
  })
end