# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- `ForcePlayerSlot` API

### Added
- `PlayerChangeSlotEvent` emitted when player changes slot

### Fixed
- Corrected `proto` files from camel-casing to snake-casing; not a runtime breaking change but some code generators
  may generate different casing by convention, creating a compiler only issue.
  - `net.proto` - `GetPlayerInfo.remote_address`
  - `mission.proto` - `PlayerSendChatEvent.player_id`

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
