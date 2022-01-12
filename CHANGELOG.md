# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Generated scaffolding for the `net.*` scope into `NetService`
- `SendChat` API
- `SendChatTo` API

### Changed
- Stream `PlayerSendChatEvent` to the `MissionService.StreamEvents` for clients to observe the chat as part of the event stream

### Removed
- `HookService.StreamChatMessages` has been removed in favor for `PlayerSendChatEvent`

## [0.2.0] - 2021-11-17
### Added
- `SetEmission` API
- `GetMissionStartTime` API
- `GetScenarioCurrentTime` API
- `GetBullseye` API
- `GetTransform` API
- `AddGroup` API (Initial version suitable for spawning fixed SAM sites)
- `Eval` API executable in the DCS hook environment along with command-line tool
- Ability to include DCS-gRPC on all server missions without needing to edit the mission
- Updated Rust version and many Rust dependencies

### Changed
- Split and reorganised APIs into versioned namespaces
- Switched to a different way of initialising the server that does not require sanitisation
- Changed Enum numbering to allow more idiomatic gRPC usage

## [0.1.0] - 2021-1-23
### Added
- Initial release with APIs documented in https://github.com/DCS-gRPC/rust-server/wiki/API-documentation-0.1.0
