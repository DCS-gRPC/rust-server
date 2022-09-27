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
    contact.name = Unit.getName({["id_"] = contact.object.id_})
    
    local contact_unit = Unit.getByName(contact.name)

    local descriptor
    if params.includeDescriptors then
      descriptor = GRPC.methods.getUnitDescriptor({ ["name"] = contact.name }).result.attributes
    end

    contact.id = tonumber(contact_unit:getID())
    contact.descriptor = descriptor
    contact.velocity = GRPC.exporters.vector(contact_unit:getVelocity())
    contact.position = GRPC.exporters.position(contact_unit:getPoint())

    results[i] = GRPC.exporters.detectedTarget(contact)
  end

  return GRPC.success({
    contacts = results
  })
end