# DCS Functional Parity

The DCS gRPC Server API implementation is still a work in progress, and therefore the following API functions have
been implemented.

The unimplemented functions is not committed to a roadmap. We do envision full DCS API equivalence at some point in the
future; pull requests are welcomed for expanding the API equivalency.

---

## Singletons

### Atmosphere Singleton
- [x] `getWind`
- [x] `getWindWithTurbulence`
- [x] `getTemperatureAndPressure`

### Coalitions Singleton
- [ ] `addGroup`
  - [ ] Sea
    - [ ] Group
    - [ ] Units
    - [ ] Waypoints
    - [ ] Tasks
  - [ ] Ground
    - [ ] Group
    - [ ] Units
    - [ ] Waypoints
    - [ ] Tasks
  - [ ] Plane
    - [ ] Group
    - [ ] Units
    - [ ] Waypoints
    - [ ] Tasks
    - [ ] Loadout
  - [ ] Helicopters
    - [ ] Group
    - [ ] Units
    - [ ] Waypoints
    - [ ] Tasks
    - [ ] Loadout
- [ ] `addStaticObject`
- [x] `getGroups`
- [ ] `getStaticObjects`
- [x] `getAirbases`
- [ ] `getPlayers`
- [ ] `getServiceProviders`
- [ ] `addRefPoint`
- [ ] `getRefPoints`
- [x] `getMainRefPoint` ("Bullseye")
- [ ] `getCountryCoalition`

### Coord Singleton
- [ ] `LLtoLO`
- [ ] `LOtoLL`
- [ ] `LLtoMGRS`
- [ ] `MGRStoLL`

### Env Singleton
- [ ] `info`
- [ ] `warning`
- [ ] `error`
- [ ] `setErrorMessageBoxEnabled`
- [ ] `getValueDictByKey`

### Land Singleton
- [ ] `getHeight`
- [ ] `getSurfaceHeightWithSeabed`
- [ ] `getSurfaceType`
- [ ] `isVisible`
- [ ] `getIP`
- [ ] `profile`
- [ ] `getClosestPointOnRoads`
- [ ] `findPathOnRoads`

### Mission Commands Singleton
- [ ] `addCommand`
- [ ] `addSubMenu`
- [ ] `removeItem`
- [ ] `addCommandForCoalition`
- [ ] `addSubMenuForCoalition`
- [ ] `removeItemForCoalition`
- [ ] `addCommandForGroup`
- [ ] `addSubMenuForGroup`
- [ ] `removeItemForGroup`

### Net Service
- [x] `net.send_chat`
- [x] `net.send_chat_to`
  - (not implemented `fromId` -- open to use cases)
- [ ] `recv_chat`
- [x] `get_player_list`
  - Implementation compounded with `get_player_info`
- [ ] `get_my_player_id`
- [ ] `get_server_id`
- [x] `get_player_info`
  - Returned as a detailed response with compounded `get_player_list`
- [ ] `kick`
- [ ] `get_stat`
- [x] <strike>`get_name`</strike> (will not be implemented -- returned part of `get_player_info`)
- [x] <strike>`get_slot`</strike> (will not be implemented -- returned part of `get_player_info`)
- [ ] `set_slot`
- [x] `force_player_slot`
- [ ] `lua2json`
- [ ] `json2lua`
- [ ] `dostring_in`
- [ ] `log`
- [ ] `trace`

#### GameGUI API
  - [ ] `load_mission`
  - [ ] `load_next_mission`


### Timer Service
- [x] `getTime`
- [x] `getAbsTime`
- [x] `getTime0`
- [ ] `scheduleFunction` (TODO -- should this be implemented?)
- [ ] `removeFunction`
- [ ] `setFunctionTime`

### Trigger Service
- [x] `getUserFlag`
- [x] `setUserFlag`
- [ ] `getZone`
- [x] `explosion`
- [x] `smoke`
- [ ] `effectSmokeBig`
- [x] `illuminationBomb`
- [x] `signalFlare`
- [ ] `radioTransmission`
- [ ] `stopRadioTransmission`
- [ ] `setUnitInternalCargo`


- [ ] `outSound`
- [ ] `outSoundForCoalition`
- [ ] `outSoundForCountry`
- [ ] `outSoundForGroup`
- [x] `outText`
- [x] `outTextForCoalition`
- [ ] `outTextForCountry`
- [x] `outTextForGroup`


- [ ] `addOtherCommand`
- [ ] `removeOtherCommand`
- [ ] `addOtherCommandForCoalition`
- [ ] `removeOtherCommandForCoalition`
- [ ] `addOtherCommandForGroup`
- [ ] `removeOtherCommandForGroup`


- [x] `markToAll`
- [x] `markToCoalition`
- [x] `markToGroup`
- [x] `removeMark`
- [ ] `markupToAll`
- [ ] `lineToAll`
- [ ] `circleToAll`
- [ ] `rectToAll`
- [ ] `quadToAll`
- [ ] `textToAll`
- [ ] `arrowToAll`
- [ ] `setMarkupRadius`
- [ ] `setMarkupText`
- [ ] `setMarkupFontSize`
- [ ] `setMarkupColor`
- [ ] `setMarkupColorFill`
- [ ] `setMarkupTypeLine`
- [ ] `setMarkupPositionEnd`


- [ ] `setAITask`
- [ ] `pushAITask`
- [ ] `activateGroup`
- [ ] `deactivateGroup`
- [ ] `setGroupAIOn`
- [ ] `setGroupAIOff`
- [ ] `groupStopMoving`
- [ ] `groupContinueMoving`


### Voice Chat
- [ ] `createRoom`

### DCS Singleton
- [ ] `addEventHandler` (TODO: Should implement? Probably not)
- [ ] `removeEventHandler`
- [ ] `getPlayer`
- [x] `getAirbases`
- [ ] `searchObjects`
- [x] `getMarkPanels`

### DCS Events
- [x] `S_EVENT_INVALID`
- [x] `S_EVENT_SHOT`
- [x] `S_EVENT_HIT`
- [x] `S_EVENT_TAKEOFF`
- [x] `S_EVENT_LAND`
- [x] `S_EVENT_CRASH`
- [x] `S_EVENT_EJECTION`
- [x] `S_EVENT_REFUELING`
- [x] `S_EVENT_DEAD`
- [x] `S_EVENT_PILOT_DEAD`
- [x] `S_EVENT_BASE_CAPTURED`
- [x] `S_EVENT_MISSION_START`
- [x] `S_EVENT_MISSION_END`
- [ ] `S_EVENT_TOOK_CONTROL`
- [x] `S_EVENT_REFUELING_STOP`
- [x] `S_EVENT_BIRTH`
- [x] `S_EVENT_HUMAN_FAILURE`
- [x] `S_EVENT_DETAILED_FAILURE`
- [x] `S_EVENT_ENGINE_STARTUP`
- [x] `S_EVENT_ENGINE_SHUTDOWN`
- [x] `S_EVENT_PLAYER_ENTER_UNIT`
- [x] `S_EVENT_PLAYER_LEAVE_UNIT`
- [ ] `S_EVENT_PLAYER_COMMENT`
- [x] `S_EVENT_SHOOTING_START`
- [x] `S_EVENT_SHOOTING_END`
- [x] `S_EVENT_MARK_ADDED`
- [x] `S_EVENT_MARK_CHANGE`
- [x] `S_EVENT_MARK_REMOVED`
- [x] `S_EVENT_KILL`
- [x] `S_EVENT_SCORE`
- [x] `S_EVENT_UNIT_LOST`
- [x] `S_EVENT_LANDING_AFTER_EJECTION`
- [ ] `S_EVENT_PARATROOPER_LENDING`
- [x] `S_EVENT_DISCARD_CHAIR_AFTER_EJECTION`
- [x] `S_EVENT_WEAPON_ADD`
- [ ] `S_EVENT_TRIGGER_ZONE`
- [x] `S_EVENT_LANDING_QUALITY_MARK`
- [ ] `S_EVENT_BDA`
- [ ] <strike>`S_EVENT_MAX`</strike> Not a real event

### DCS Control Functions
- [x] `setUserCallbacks`
  - [ ] `onMissionLoadBegin`
  - [ ] `onMissionLoadProgress`
  - [x] `onMissionLoadEnd` (Used; not exposed to Clients)
  - [ ] `onSimulationStart`
  - [x] `onSimulationStop` (Used; not exposed to Clients)
  - [x] `onSimulationFrame` (Used; not exposed to Clients)
  - [ ] `onSimulationPause`
  - [ ] `onSimulationResume`
  - [ ] `onGameEvent`
  - [ ] `onNetConnect`
  - [ ] `onNetMissionChanged`
  - [ ] `onNetDisconnect`
  - [ ] `onPlayerConnect`
  - [x] `onPlayerDisconnect` (Emitted as a special GRPC event)
  - [ ] `onPlayerStart`
  - [ ] `onPlayerStop`
  - [ ] `onPlayerChangeSlot`
  - [x] `onPlayerTryConnect` (Emitted as a special GRPC event)
  - [x] `onPlayerTrySendChat` (Emitted as a special GRPC event)
  - [ ] `onPlayerTryChangeSlot`
- [ ] `setPause`
- [ ] `getPause`
- [ ] `stopMission`
- [ ] `exitProcess`
- [ ] `isServer`
- [ ] `getModelTime`
- [ ] `getRealTime`
- [ ] `getMissionOptions`
- [ ] `getAvailableCoalitions`
- [ ] `getAvailableSlots`
- [ ] `getCurrentMission`
- [x] `getMissionName`
- [ ] `getMissionFilename`
- [ ] `getMissionResult`
- [ ] `getUnitProperty`
- [ ] `getUnitType`
- [ ] `getUnitTypeAttribute`
- [ ] `writeDebriefing`
- [ ] `makeScreenShot`

---

## Classes

### Object
- [ ] Enums
  - [ ] Category
- [ ] `destroy`
- [ ] `isExists`
- [ ] `getCategory`
- [ ] `getTypeName`
- [ ] `getDesc`
- [ ] `hasAttribute`
- [ ] `getName`
- [ ] `getPoint`
- [ ] `getPosition`
- [ ] `getVelocity`
- [ ] `inAir`

### SceneryObject (inherits Object)
- [ ] `getLife`
- [ ] `getDescByName` (static)

### CoalitionObject (Object)
- [ ] `getCoalition`
- [ ] `getCountry`

### Weapon (inherits CoalitionObject)
- [ ] `getLauncher`
- [ ] `getTarget`
- [ ] `Desc.category`
- [ ] `Desc.warhead.type`
- [ ] `Desc.warhead.mass`
- [ ] `Desc.warhead.caliber`
- [ ] `Desc.warhead.explosiveMass`
- [ ] `Desc.warhead.shapedExplosiveMass`
- [ ] `Desc.warhead.shapedExplosiveArmorThickness`
- [ ] `DescMissile.guidance`
- [ ] `DescMissile.rangeMin`
- [ ] `DescMissile.rangeMaxAltMin`
- [ ] `DescMissile.rangeMaxAltMax`
- [ ] `DescMissile.altMin`
- [ ] `DescMissile.altMax`
- [ ] `DescMissile.Nmax` (max range?)
- [ ] `DescMissile.fuseDist`
- [ ] `DescRocket.distMin`
- [ ] `DescRocket.distMax`
- [ ] `DescBomb.guidance`
- [ ] `DescBomb.altMin`
- [ ] `DescBomb.altMax`

### Unit (inherits CoalitionObject)
- [ ] `isActive`
- [ ] `getPlayerName`
- [ ] `getID`
- [ ] `getNumber`
- [ ] `getController`
- [ ] `getGroup`
- [ ] `getCallsign`
- [ ] `getLife`
- [ ] `getLife0`
- [ ] `getFuel`
- [ ] `getAmmo`
- [ ] `getSensors`
- [ ] `hasSensors`
- [ ] `getRadar`
- [ ] `getDrawArgumentValue`
- [ ] `getNearestCargos`
- [ ] `enableEmission`
- [ ] `getDescentCateogry`
- [ ] (static) `getByName`
- [ ] (static) `getDescByName`

### Airbase (inherits CoalitionObject)
- [ ] `getDesc`
- [ ] `getCallsign`
- [ ] `getUnit`
- [ ] `getID`
- [ ] `getParking`
- [ ] `getRunways`
- [ ] `getTechObjectPos`
- [ ] `getRadioSilentMode`
- [ ] `setRadioSilentMode`

### StaticObject (inherits CoalitionObject)
- [ ] `getID`
- [ ] `getLife`
- [ ] `getCargoDisplayName`
- [ ] `getCargoWeight`
- [ ] `getDrawArgumentValue`
- [ ] (static) `getByName`
- [ ] (static) `getDescByName`

### Group
- [ ] (static) `getByName`
- [ ] `isExist`
- [ ] `activate`
- [ ] `destroy`
- [ ] `getCategory`
- [ ] `getCoalition`
- [ ] `getName`
- [ ] `getID`
- [ ] `getUnit`
- [ ] `getUnits`
- [ ] `getSize`
- [ ] `getController`
- [ ] `enableEmission`

### Controller
- [ ] `setTask`
- [ ] `resetTask`
- [ ] `pushTask`
- [ ] `popTask`
- [ ] `hasTask`
- [ ] `setCommand`
- [ ] `setOption`
- [ ] `setOnOff`
- [ ] `knowTarget`
- [ ] `isTargetDetected`
- [ ] `getDetectedTargets`
- [Main Tasks](https://wiki.hoggitworld.com/view/DCS_task_mission)
  - [ ] `mission`
  - [ ] `AttackGroup`
  - [ ] `AttackUnit`
  - [ ] `Bombing`
  - [ ] `CarpetBombing`
  - [ ] `AttackMapObject`
  - [ ] `BombingRunway`
  - [ ] `orbit`
  - [ ] `refueling`
  - [ ] `land`
  - [ ] `follow`
  - [ ] `followBigFormation`
  - [ ] `escort`
  - [ ] `Embarking`
  - [ ] `fireAtPoint`
  - [ ] `hold`
  - [ ] `FAC_AttackGroup`
  - [ ] `EmbarkToTransport`
  - [ ] `DisembarkFromTransport`
  - [ ] `CargoTransportation`
  - [ ] `goToWaypoint`
  - [ ] `groundEscort`
- Enroute Tasks
  - [ ] `engageTargets`
  - [ ] `engageTargetsInZone`
  - [ ] `engageGroup`
  - [ ] `engageUnit`
  - [ ] `awacs`
  - [ ] `tanker`
  - [ ] `ewr`
  - [ ] `FAC_engageGroup`
  - [ ] `FAC`
- Commands
  - [ ] `script`
  - [ ] `setCallsign`
  - [ ] `setFrequency`
  - [ ] `switchWaypoint`
  - [ ] `stopRoute`
  - [ ] `switchAction`
  - [ ] `setInvisible`
  - [ ] `setImmortal`
  - [ ] `activateBeacon`
  - [ ] `deactivateBeacon`
  - [ ] `eplrs`
  - [ ] `start`
  - [ ] `transmitMessage`
  - [ ] `stopTransmission`
  - [ ] `smoke_on_off`
- [Options](https://wiki.hoggitworld.com/view/DCS_func_setOption)
  - [ ] `ROE`
  - [ ] `Reaction To Threat`
  - [ ] `Radar Using`
  - [ ] `Flare Using`
  - [ ] `Formation`
  - [ ] `RTB On Bingo`
  - [ ] `silence`
  - [ ] `Disperse on Attack`
  - [ ] `Alarm State`
  - [ ] `ECM Using`
  - [ ] `Prohibit AA`
  - [ ] `Prohibit Jettison`
  - [ ] `Prohibit Afterburner`
  - [ ] `Prohibit AG`
  - [ ] `Missile Attack Range`
  - [ ] `Prohibit WP Pass Report`
  - [ ] `Engage Air Weapons`
  - [ ] `Option Radio Usage Contact`
  - [ ] `Option Radio Usage Kill`
  - [ ] `AC Engagement Range`
  - [ ] `jett tanks if empty`
  - [ ] `forced attack`
  - [ ] `Altitude Restriction for AAA Min`
  - [ ] `restrict targets`
  - [ ] `Altitude Restriction for AAA Max`

### Detection
TODO

### Spot
- [ ] (static) `createInfraRed`
- [ ] `createLaser`
- [ ] `createInfraRed`
- [ ] `destroy`
- [ ] `getCategory`
- [ ] `getPoint`
- [ ] `setPoint`
- [ ] `getCode`
- [ ] `setCode`
