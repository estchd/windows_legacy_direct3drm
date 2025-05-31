# Windows Legacy Direct3D API Bindings

Metadata and Rust bindings for Legacy Direct3DRM APIs that are not included in the [win32metadata](https://github.com/microsoft/win32metadata) metadata package.

Includes Types, Interfaces, Functions, etc. from the `d3drm.h`, `d3drmdef.h` and `d3drmobj.h` Headers. 

## Regenerating the Metadata

If you want to regenerate the metadata file yourself, see the `Readme.md` file under `.direct3drm/include` to see what headers are needed and run `dotnet build` in the `.metadata` directory.

## Regenerating the Bindings

The bindings are automatically regenerated the first time this crate is built and every time the metadata file in the `.windows` directory changes.