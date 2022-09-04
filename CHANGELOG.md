# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Fixed error when retrieving mark panels (`WorldService.GetMarkPanels`) when the mark panel was created by a game master / JTAC, or when the player who created the mark panel left. `MarkPanel.initiator` is now optional. ([#156](https://github.com/DCS-gRPC/rust-server/issues/156))
- Fixed scale of blocking time percentage in stats logs.

### Added
- Added `SimulationFps` event that is fired every second and contains simulation fps information since the last event (i.e. for the past ~1sec).
- Added `GetSessionId` API

### Changed
- Unit objects now return the full group object in the `group` field to make event processing easier. This replaces the `group_name` and `group_category` fields and is a backwards incompatible change.

## [0.6.0] - 2022-05-30

### Added
- `OutTextForUnit` API
- `GetStaticObjects` API
- `AddStaticObject` API (for standard static objects)
- `AddLinkedStatic` API (for statics linked to units such as ships)
- `MarkupToAll` API
- `MarkupToCoalition` API
- `GetTheatre` API
- `GetUnitType` API
- `ReloadCurrentMission` API
- `LoadNextMission` API
- `LoadMission` API

### Fixed
- Fixed event handler error log missing actual error message (contained `nil` instead of the message).

## [0.5.0] - 2022-04-19
### Added
- `GetMissionFilename` API
- `GetPaused` API
- `SetPaused` API
- `StopMission` API
- `ExitProcess` API
- `KickPlayer` API
- `IsMultiplayer` API
- `IsServer` API
- `GetMissionDescription` API
- `BanPlayer` API
- `GetBannedPlayers` API
- `UnbanPlayer`  API

### Changed
- Replaced `groupName` field in the `GroupCommand` event with all the group details as exposed by the group exporter
  (currently id, name, coalition, category). This change was made based on experience writing a client that processes these events
  where only having the groupName was a limitation. This change breaks backwards compatibility with 0.4.0 where the `GroupCommand`
  event was first added.

## [0.4.0] - 2022-03-07
### Added
- `ForcePlayerSlot` API
- `PlayerChangeSlotEvent` emitted when player changes slot
- `StreamUnits` can optionally specify the `category` of the units which may be monitored.
- APIs for creating the F10 radio menus and letting players run them. These will emit events to DCS-gRPC clients when run.

### Fixed
- `MarkToCoalition` was sending the mark to the incorrect coalition.
- `NetService.GetPlayers` overwrote `CoalitionService.GetPlayers` (see Breaking Changes for details)
- Corrected `proto` files from camel-casing to snake-casing; not a runtime breaking change but some code generators
  may generate different casing by convention, creating a compiler only issue.
  - `net.proto` - `GetPlayerInfo.remote_address`
  - `mission.proto` - `PlayerSendChatEvent.player_id`
- Corrected `proto` files with enumerations to be named correct; compiler-only breaking change, not runtime.
  - `coalition.proto` - `AddGroupRequest.Point` - enum `Type` has been renamed to `PointType`
  - `coalition.proto` - `AddGroupRequest` - enum members of `Skill` has been prefixed with `SKILL_`
- `CoalitionService.GetPlayers` did not filter correctly on specified coalition
- `StreamUnits` would only monitor the `Plane` groups; now monitors all groups with the default option of `GROUP_CATEGORY_UNSPECIFIED`

### Breaking Changes
- Added `GROUP_CATEGORY_UNSPECIFIED` to `dcs.v0.common.GroupCategory`; breaking change as all indexes have changed.
- `CoalitionService.GetPlayers` was renamed to `CoalitionService.GetPlayerUnits`; fixes conflict with `NetService.GetPlayers`



## [0.3.0] - 2022-01-14
### Added
- Generated scaffolding for the `net.*` scope into `NetService`
- `SendChat` API
- `SendChatTo` API
- `GetPlayers` API
- Optional config file at `Saved Games\DCS\Config\dcs-grpc.lua`
- `Connect` and `Disconnect` events
- INFO log entry for the the host and port he server listens on
- DEBUG log entry for all current settings
- `place` to `LandingQualityMark` event

### Changed
- Stream `PlayerSendChatEvent` to the `MissionService.StreamEvents` for clients to observe the chat as part of the event stream
- Fixed an issue where units updates were not being stream after initial load

### Removed
- `HookService.StreamChatMessages` has been removed in favor for `PlayerSendChatEvent`
- Option to specify settings inside of the `MissionScripting.lua`

### Fixed
- Speed and heading in units stream

## [0.2.0] - 2021-11-17
### Added
- `SetEmission` API
- `GetScenarioStartTime` API
- `GetScenarioCurrentTime` API
- `GetBullseye` API
- `GetTransform` API
- `AddGroup` API (Initial version suitable for spawning fixed SAM sites)
- `Eval` API executable in the DCS hook environment along with command-line tool
- `day`, `month` and `year` fields to `GetTimeZero` API
- `coalition` and `category` to `Group`s
- `category` to `Unit`s
- `GetUnit` API
- `GetMagneticDeclination` API
- `GetTransform` API (get all information for a unit in 3D space)
- Ability to include DCS-gRPC on all server missions without needing to edit the mission
- Updated Rust version and many Rust dependencies
- `autostart` option

### Changed
- Split and reorganised APIs into versioned namespaces
- Switched to a different way of initialising the server that does not require sanitisation
- Changed Enum numbering to allow more idiomatic gRPC usage

## [0.1.0] - 2021-1-23
### Added
- Initial release with APIs documented in https://github.com/DCS-gRPC/rust-server/wiki/API-documentation-0.1.0
