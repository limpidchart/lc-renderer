## [0.2.0] - 2021-08-21

### Added

* Added `SIGINT` signal graceful shutdown handler
* Updated `lc-proto` to `v0.4.0` and protobuf usage
* Removed unused args from `Dockerfile`, added empty `CMD`, Rust version is set to `:1`

## [0.1.0] - 2021-05-27

### Added

* Added `lc-proto` with scripts to download and build protobufs
* Added `RendererError` enum that contains all application errors
* Added helpers to convert scales, colors, view values, points, bar label positions from protobuf enums
* Added helpers to create chart views from protobuf
* Added `RendererServer` that implements `ChartRenderer` lc-proto service
* Added workflows to lint and test code
* Added `Dockerfile` and workflows to build and push images

## [0.0.1] - 2021-05-18

Initial release of the app that provides gRPC API to render chart images via `lc-render` library
