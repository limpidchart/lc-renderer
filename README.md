# lc-renderer

[![crates.io](https://img.shields.io/crates/v/lc-renderer.svg)](https://crates.io/crates/lc-renderer)
[![docs](https://docs.rs/lc-renderer/badge.svg)](https://docs.rs/lc-renderer)
[![inspection](https://github.com/limpidchart/lc-renderer/actions/workflows/inspection.yml/badge.svg?branch=main)](https://github.com/limpidchart/lc-renderer/actions/workflows/inspection.yml)

Server to render chart images

## API

The server implements [GRPC](https://www.grpc.io) API of [lc-proto](https://github.com/limpidchart/lc-proto) `ChartRenderer` service.  
It performs only a very basic validation of input parameters.  
Please use lc-api for complex validation and REST API.
