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
    - [x] Units
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
- [x] `addStaticObject`
- [x] `getGroups`
- [x] `getStaticObjects`
- [x] `getAirbases`
- [x] `getPlayers`
- [ ] `getServiceProviders`
- [ ] `addRefPoint`
- [ ] `getRefPoints`
- [x] `getMainRefPoint` ("Bullseye")
- [ ] `getCountryCoalition`

### Coord Singleton
- [ ] <del>`LLtoLO`</del> (internally, used; but LAT/LNG is used in API definitions)
- [ ] <del>`LOtoLL`</del> (internally, used; but LAT/LNG is used in API definitions)
- [ ] <del>`LLtoMGRS`</del> Rather implement using conversion: https://en.wikipedia.org/wiki/Military_Grid_Reference_System
- [ ] <del>`MGRStoLL`</del> (internally, used; but LAT/LNG is used in API definitions)

### Env Singleton
The following API's are not planned to be exposed via gRPC; client applications
should use their independent logging and tracing functions.
- [ ] <del>`info`</del>
- [ ] <del>`warning`</del>
- [ ] <del>`error`</del>
- [ ] <del>`setErrorMessageBoxEnabled`</del>
- [ ] <del>`getValueDictByKey`</del>

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
- [x] `addCommand`
- [x] `addSubMenu`
- [x] `removeItem`
- [x] `addCommandForCoalition`
- [x] `addSubMenuForCoalition`
- [x] `removeItemForCoalition`
- [x] `addCommandForGroup`
- [x] `addSubMenuForGroup`
- [x] `removeItemForGroup`

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
- [x] `kick`
- [ ] `get_stat`
- [x] <strike>`get_name`</strike> (will not be implemented -- returned part of `get_player_info`)
- [x] <strike>`get_slot`</strike> (will not be implemented -- returned part of `get_player_info`)
- [ ] `set_slot`
- [x] `force_player_slot`
- [ ] <del>`lua2json`</del> (DCS-gRPC bridge preferred)
- [ ] <del>`json2lua`</del> (DCS-gRPC bridge preferred)
- [ ] <del>`dostring_in`</del> (DCS-gRPC bridge preferred)
- [ ] <del>`log`</del> (client applications should use their own logging)
- [ ] <del>`trace`</del> (client applications should use their own logging)

#### GameGUI API
- [x] `reload_current_mission` Custom API
- [x] `load_mission`
- [x] `load_next_mission`

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
- [x] `outTextForUnit`

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
- [x] `markupToAll`
- [x] `markupToCoalition`
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

### DCS Singleton (DCS-gRPC WorldService)
- [ ] `addEventHandler` (TODO: Should implement? Probably not)
- [ ] `removeEventHandler`
- [ ] `getPlayer`
- [x] `getAirbases`
- [ ] `searchObjects`
- [x] `getMarkPanels`
- [x] `getTheatre` (DCS-gRPC method. Calls env.mission.theatre internally)

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
- [x] `setPause` - API name changed to `setPaused` as this is more accurate
- [x] `getPause` - API name changed to `getPaused` as this is more accurate
- [x] `stopMission`
- [x] `exitProcess`
- [x] `isMultiplayer`
- [x] `isServer`
- [ ] `getModelTime`
- [ ] `getRealTime`
- [ ] `getMissionOptions`
- [ ] `getAvailableCoalitions`
- [ ] `getAvailableSlots`
- [ ] `getCurrentMission`
- [x] `getMissionName`
- [x] `getMissionDescription`
- [x] `getMissionFilename`
- [ ] `getMissionResult`
- [ ] `getUnitProperty`
- [x] `getUnitType`
- [ ] `getUnitTypeAttribute`
- [ ] `writeDebriefing`
- [ ] `makeScreenShot`

---

## Classes

### SceneryObject
- [ ] `Object` class members
  - [ ] Category
  - [ ] `destroy`
  - [ ] `isExists`
  - [ ] `getCategory`
  - [x] `getTypeName`
  - [ ] `getDesc`
  - [ ] `hasAttribute`
  - [x] `getName`
  - [x] `getPoint`
  - [ ] `getPosition`
  - [ ] `getVelocity`
  - [ ] `inAir`
- [ ] `SceneryObject` class members
  - [ ] `getLife`
  - [ ] `getDescByName` (static)

### Weapon

- [ ] `Object` class members
  - [ ] Category
  - [ ] `destroy`
  - [ ] `isExists`
  - [ ] `getCategory`
  - [x] `getTypeName`
  - [ ] `getDesc`
  - [ ] `hasAttribute`
  - [x] `getName`
  - [x] `getPoint`
  - [ ] `getPosition`
  - [ ] `getVelocity`
  - [ ] `inAir`
- [ ] `CoalitionObject` class members
  - [ ] `getCoalition`
  - [ ] `getCountry`
- [ ] `Weapon` class members
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

### Unit
Primarily enhanced with `GRPC.exporters.unit`

- [ ] `Object` class members
  - [ ] Category
  - [ ] `destroy`
  - [ ] `isExists`
  - [x] `getCategory`
  - [x] `getTypeName`
  - [ ] `getDesc`
  - [ ] `hasAttribute`
  - [x] `getName`
  - [x] `getPoint`
  - [ ] `getPosition`
  - [x] `getVelocity`
  - [ ] `inAir`
- [ ] `CoalitionObject` class members
  - [x] `getCoalition`
  - [ ] `getCountry`
- [ ] `Unit` Class members
  - [ ] `isActive`
  - [ ] `getPlayerName`
  - [x] `getID`
  - [ ] `getNumber`
  - [ ] `getController`
  - [ ] `getGroup`
  - [x] `getCallsign`
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

### Airbase

- [ ] `Object` class members
  - [ ] Category
  - [ ] `destroy`
  - [ ] `isExists`
  - [x] `getCategory`
  - [ ] `getTypeName`
  - [ ] `getDesc`
  - [ ] `hasAttribute`
  - [x] `getName`
  - [x] `getPoint`
  - [ ] `getPosition`
  - [ ] `getVelocity`
  - [ ] `inAir`
- [ ] `CoalitionObject` class members
  - [x] `getCoalition`
  - [ ] `getCountry`
- [ ] `Airbase` class members
  - [ ] `getDesc`
  - [x] `getCallsign`
  - [x] `getUnit`
  - [ ] `getID`
  - [ ] `getParking`
  - [ ] `getRunways`
  - [ ] `getTechObjectPos`
  - [ ] `getRadioSilentMode`
  - [ ] `setRadioSilentMode`

### StaticObject

- [ ] `Object` class members
  - [ ] Category
  - [ ] `destroy`
  - [ ] `isExists`
  - [ ] `getCategory`
  - [x] `getTypeName`
  - [ ] `getDesc`
  - [ ] `hasAttribute`
  - [x] `getName`
  - [x] `getPoint`
  - [ ] `getPosition`
  - [ ] `getVelocity`
  - [ ] `inAir`
- [ ] `CoalitionObject` class members
  - [x] `getCoalition`
  - [ ] `getCountry`
- [ ] `StaticObject` Class members
  - [x] `getID`
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
- [x] `getCategory`
- [x] `getCoalition`
- [x] `getName`
- [x] `getID`
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
  - [x] `Alarm State`
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
