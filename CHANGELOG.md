# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

[Commits](https://github.com/Emilgardis/voicemeeter-sdk-rs/compare/v0.2.0...Unreleased)

### Breaking changes

* Made `FadeBy` and `FadeTo` into floats instead of integers #25
* Made all errors non-exhaustive #27
* Added VoicemeeterProgram::None for when there is no application running.
* Made `Strip::eq` return new error `ParameterError`.
* `OutputDevice` and `InputDevice` use an enum `DeviceType` for device type.

### Fixes

* Fixed initianting remote with application turned off.
* Fix wrong `EQ.AB` on bus and implement `EQ.on` and `EQ.AB` for strips
* Fixed `get_output_device` and `get_input_device` so that correct values are returned.

### Changes

* Fixed login and logout. #26

### Added

* Added `VoicemeeterRemote::update_program`
* Added `Strip::gate_detailed`, `Strip::comp_detailed` and  `Strip::denoiser`

## [v0.2.0] - 2023-09-11

[Commits](https://github.com/Emilgardis/voicemeeter-sdk-rs/compare/v0.1.1...v0.2.0)

### Breaking changes

* Made all gain values floats instead of integers #21

### Added

* Added a `param` function to most interfaces, allowing usage with `get_parameter*` and `set_parameter*` #23

### Changes

* Bus and strip parameters are now correctly named, previously some bus parameters used `Strip[i]`

## [End of Changelog]

Changelog starts on v0.1.1