# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Ability to output secrets in an `env` format with `bws` (#320)

## [0.3.1] - 2023-10-13

### Added

- Support for shell autocompletion with the `bws completions` command (#103)
- When running `bws` with no args, the help text is now printed to `stderr` instead of `stdout` to
  be consistent with `bws subcommand` behavior (#190)

## [0.3.0] - 2023-07-26

### Deprecated

- Switched command order from `action type` to `type action`, please re-read the help documentation
  (#76)

### Added

- Ability to create and edit projects (#53)
- Ability to create and edit secrets (#77)
- Support `NO_COLOR` environment variable to disable CLI colors (#61)
- Support for `CLICOLOR_FORCE` (#74)

### Fixed

- Improve login error handling (#109)
- Respect users color choice for errors (#61)

## [0.2.1] - 2023-03-22

### Fixed

- Add user agent to login requests (#11)
