# Current context

DA Cartographer is the standalone community editor for Dark Ages map files.
It supports editing ground and walls, prefab placement, and map and collision
PNG export. Keep community asset compatibility independent of any server's
content model.

## Code map

- [archive](archive/) opens and indexes the user's `.dat` archives.
- [map](map/) owns raw tile bytes and dimension inference.
- [render](render/) decodes indexed art and builds texture atlases.
- [cartographer](cartographer/) owns the egui UI, editing, and PNG export.
- [packaging](packaging/) and the [release workflow](.github/workflows/release.yml)
  build platform packages from the shared icon. CI tests and packages all six
  platform and architecture combinations on GitHub-hosted runners. Pushing a
  version tag publishes those packages as a DA Cartographer release.

## Compatibility and handoff

Preserve legacy single-palette and newer palette-table rendering. Ground and
wall animations use optional tables. Project data is local to the user's
installation. [The editor guide](docs/EditorGuide.md) explains those paths.

The [code audit](docs/Audit.md) records the verified save, wall loading,
animation, export, dependency, and viewport fixes. Follow-up work covers
visible animation deadlines, atlas paging, preview caching, and native
release profiling. The maintainer owns the repository transfer.
