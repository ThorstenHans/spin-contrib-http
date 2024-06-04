# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.8](https://github.com/ThorstenHans/spin-contrib-http/compare/v0.0.7...v0.0.8) - 2024-06-04

### Other
- check allowed origins on every request if ORIGIN is present
- implement Debug trait for CorsConfig

## [0.0.7](https://github.com/ThorstenHans/spin-contrib-http/compare/v0.0.6...v0.0.7) - 2024-05-08

### Added
- Add get_header_value_as_string to Request (Contrib trait)

### Other
- Add sample to illustrate how one could use CORS
- change signature of CorsRouter::register_options_handler to take a ref of CorsConfig
- implement Clone trait for CorsConfig

## [0.0.6](https://github.com/ThorstenHans/spin-contrib-http/compare/v0.0.5...v0.0.6) - 2024-05-08

### Added
- reactor CORS API (BREAKING)

## [0.0.5](https://github.com/ThorstenHans/spin-contrib-http/compare/v0.0.4...v0.0.5) - 2024-04-29

### Added
- [**breaking**] Rewrite CORS according to spec

## [0.0.4](https://github.com/ThorstenHans/spin-contrib-http/compare/v0.0.3...v0.0.4) - 2024-04-26

### Other
- remove deps and println statements

## [0.0.3](https://github.com/ThorstenHans/spin-contrib-http/compare/v0.0.2...v0.0.3) - 2024-04-26

### Other
- Update documentation for crates.io and docs.rs
