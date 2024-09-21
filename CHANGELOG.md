# Changelog

## [2.0.0](https://github.com/salpland/haze/compare/v1.4.0...v2.0.0)

### Changes

- Allow importing and exporting multiple worlds at once
- Fix target worlds not being cleaned up during exporting with the `--overwrite`
  flag ([#14](https://github.com/salpland/haze/issues/14))
- Add a pretty printer for `haze list`
- Rename `--path` flag to `--minecraft-version` and restrict setting its value
  to `stable`, `preview`, or `education`
- Allow setting `COM_MOJANG` environment variable to define an arbitrary
  `com.mojang` path
- Merge `haze_core` and `haze` into a single crate
- Add colored help message for `haze help`
- Support using comments in `config.json`
- Provide binaries for Linux and MacOS

## [1.4.0](https://github.com/salpland/haze/compare/v1.3.0...v1.4.0)

### Changes

- Add the `--path` flag to allow using predefined export/import paths to
  `com.mojang` or custom ones
  ([#12](https://github.com/salpland/haze/issues/12))

## [1.3.0](https://github.com/salpland/haze/compare/v1.2.0...v1.3.0)

### Changes

- Add `haze list` subcommand to list the available worlds in the project. By
  [@solvedDev](https://github.com/solvedDev)
  ([#10](https://github.com/salpland/haze/issues/10))
- BREAKING: Rename `test` and `save` subcommands to `export` and `import`
  respectively. By [@solvedDev](https://github.com/solvedDev)
  ([#8](https://github.com/salpland/haze/issues/8))
- Extract the core of Haze into a library. By
  [@solvedDev](https://github.com/solvedDev)
  ([#9](https://github.com/salpland/haze/issues/9))

## [1.2.0](https://github.com/salpland/haze/compare/v1.1.0...v1.2.0)

### Changes

- BREAKING: Use `worlds` property in the configuration
  ([#4](https://github.com/salpland/haze/issues/4))
- Improve error and logging messages
- Use less bold text for errors
- Update the descriptions for some commands
- Remove diagnostic codes

## [1.1.0](https://github.com/salpland/haze/compare/v1.0.1...v1.1.0)

### Changes

- Disable overwriting in `haze test` by default
  ([#1](https://github.com/salpland/haze/issues/1))
- Add `--overwrite` flag to `haze test` to enable overwriting
  ([#2](https://github.com/salpland/haze/issues/2))
- Improve error/info messages ([#3](https://github.com/salpland/haze/issues/2))

## [1.0.1](https://github.com/salpland/haze/compare/v1.0.0...v1.0.1)

### Changes

- Update the project's description for more clarity
