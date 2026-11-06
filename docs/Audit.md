# DA Cartographer code audit

Reviewed November 5, 2026. The numbered findings record behavior before the
fixes. See [implementation and verification](#implementation-and-verification)
for the current status.

## Fix before the next release

1. **Track which revision was saved.** Edit a Map, save, then undo the only
   edit. The document differs from disk, but the undo stack is empty and
   closing skips the save prompt. The opposite case can show a prompt for a
   document that matches disk. The toolbar also cleared undo history after
   saving, hiding the faulty state in that workflow. Track a saved revision
   across undo, redo, and divergent edits. Save Maps and prefabs through a same-directory temporary
   file so a failed write preserves the prior file. The missing prompt was
   reproduced with a scratch test.
   [Document history](../cartographer/src/document.rs#L1060) ·
   [Map save](../map/src/lib.rs#L95) ·
   [Prefab save](../cartographer/src/prefab.rs#L244)

2. **Load sparse wall banks.** The wall loader stops at a missing ID more
   than 100 past the last decoded sprite.
   The local legacy archive jumps from 8694 to 10000. Its `stc10013.hpf`
   decodes independently but never reaches the editor atlas. Of the 1,029
   files in `simkin_maps`, 271 reference visible wall art present after the
   cutoff. `lod4721.map` references 10013. Enumerate archive entries, parse
   wall IDs case-insensitively, and preserve archive precedence. Test sparse
   and mixed-case names and look up every frame in a wall cycle.
   [Wall loader](../cartographer/src/app/mod.rs#L1062) ·
   [Archive index](../archive/src/pool.rs#L24)

3. **Resolve animation frames and palettes consistently.** Both palette modes
   share the playback code. Ground Map ID 1 converts to atlas index 0, which
   the shared animator treats as empty. Newer wall palette loading also
   accepts `stcani.tbl` as a palette table. A synthetic two-frame wall cycle
   selected palette 5 from the delay field. Separate Map IDs from atlas
   indices, match palette table names explicitly, and use one frame resolver
   for the viewport and all previews. Reject invalid cycles and keep the base
   sprite visible if the selected frame has no artwork.
   [Ground conversion](../cartographer/src/tile_animation.rs#L108) ·
   [Frame lookup](../cartographer/src/tile_animation.rs#L89) ·
   [Palette loading](../cartographer/src/palette_lookup.rs#L140) ·
   [7.41 animation table evidence](https://ewrogers.github.io/darkages-741-re/file-formats/tile-animation-tables.html)

4. **Bound Map input and PNG export.** A payload with 65,537 cells opens as
   1×1 due to a narrowing cast. Reject dimensions that cannot represent the
   payload. Set explicit allocation limits for Maps, prefabs, and table
   ranges. Oren Island City's 133×200 Map at 400% needs about 2.50 GiB for
   the RGBA export buffer before wall headroom and PNG encoding. Validate
   output size first, then run rendering and encoding on a worker. The
   dimension bug was reproduced. The export memory figure comes from the
   production sizing formula.
   [Map dimensions](../map/src/lib.rs#L151) ·
   [Palette ranges](../cartographer/src/palette_lookup.rs#L16) ·
   [Export allocation](../cartographer/src/export.rs#L19) ·
   [UI export call](../cartographer/src/app/mod.rs#L738)

5. **Refresh the lockfile.** The dry run resolved `memmap2` 0.9.12,
   `webbrowser` 1.2.6, `quick-xml` 0.41.0, and `event-listener` 5.4.2
   within the current manifest. It also picked up compatible updates to
   `image`, `serde`, `tracing-subscriber`, and `winresource`. These are
   lockfile findings. Advisory reachability in editor workflows still needs
   its own check. Build all desktop release targets after refreshing.
   [Lockfile](../Cargo.lock) ·
   [memmap2 advisory](https://rustsec.org/advisories/RUSTSEC-2026-0186.html) ·
   [webbrowser advisory](https://rustsec.org/advisories/RUSTSEC-2026-0257.html) ·
   [quick-xml advisory](https://rustsec.org/advisories/RUSTSEC-2026-0194.html) ·
   [event-listener advisory](https://rustsec.org/advisories/RUSTSEC-2026-0221.html)

## Next performance and compatibility work

- **Cull before looking up sprites.** The viewport visits every Map cell
  and builds meshes during each repaint. With the same 1,000×650 viewport
  and 104 submitted shapes, a headless debug-build probe measured median
  CPU scene construction of 1.0 ms at 64×64, 6.4 ms at 256×256, and 94.0 ms
  at 1,024×1,024. This isolates CPU scene construction. Measure release frame
  times on dense Maps when implementing visible-cell culling. Preserve wall
  draw order and include enough margin for tall and animated sprites. Cache
  the collision grid, palette ID lists, and silhouette work by their inputs.
  [Scene traversal](../cartographer/src/panels/viewport.rs#L1058) ·
  [Collision grid](../cartographer/src/panels/viewport.rs#L1678) ·
  [Palette rows](../cartographer/src/panels/inspector.rs#L823)

- **Schedule visible animation transitions and page atlases.** The shortest
  interval anywhere in an archive wakes the whole editor, even on a static
  Map. Wake at the next frame boundary among visible sprites and previews.
  A single oversized texture also blanks a whole layer. The local ground
  atlas is 3,584×5,238, so a device limited to 4,096 pixels cannot upload it.
  Pack pages within the device limit and preserve depth order across pages.
  [Repaint scheduling](../cartographer/src/app/mod.rs#L2447) ·
  [Texture upload](../cartographer/src/app/mod.rs#L1126)

- **Define the compatibility fixtures before broadening support.** The local
  4.51 archive supplies the legacy palette and base terrain, with neither
  animation table. The newer checks use synthetic archive entries and the
  7.41 format reference. Add fixtures for both palette modes covering ground
  and wall frame timing, starting phase, missing frames, and static PNG
  export policy. Alternate art in `tileas.bmp` and `sts*.hpf` deserves its
  own follow-up if seasonal preview is in scope.
  [PNG rendering](../cartographer/src/export.rs#L86) ·
  [7.41 alternate banks](https://ewrogers.github.io/darkages-741-re/file-formats/map-tile-banks.html)

- **Keep egui and stage the GUI upgrade separately.** The locked eframe
  0.33.3 build uses Glow. The 0.37 line is available and changes the app
  entry points and default renderer. Upgrade with an explicit backend choice
  after collecting a baseline. Update `rfd` 0.15.4 → 0.17.2 separately and
  smoke-test Linux file dialogs. RON 0.11.0 → 0.12.3 and ICO 0.5.0 → 0.6.0
  can follow with prefab round trips and Windows icon checks. Direct PNG
  0.18.1 and tracing 0.1.44 already match the available releases.
  [eframe changelog](https://github.com/emilk/egui/blob/main/crates/eframe/CHANGELOG.md) ·
  [rfd changelog](https://docs.rs/crate/rfd/latest/source/CHANGELOG.md)

## Implementation and verification

- Saves now retain history and track the saved checkpoint. Map, prefab,
  and PNG writes replace the destination after the new file is complete.
- The archive index matches asset names without regard to capitalization.
  The 4.51 archive now loads the late wall bank used by `lod4721.map`,
  including its 56-pixel-high `stc10013.hpf`.
- Both palette modes share the frame resolver. Regression tests cover
  ground index zero, wall phase, missing art, and the distinction between
  animation and palette tables.
- Maps and prefabs accept up to 1,048,576 cells. PNG buffers are capped at
  256 MiB each. The worker renders and encodes from a snapshot of the map.
- The lockfile now resolves the patched dependencies called out above.
  The GUI and native file-dialog upgrades remain separate.
- In the same headless CPU probe, 256×256 and 1,024×1,024 both took about
  0.66 ms after culling and submitted the same 104 shapes.

The audit used the local 4.51 archives and 1,435 maps from `simkin_maps` and
`mimi_maps`. The current checks cover synthetic animation data and CPU scene
construction. Visible animation deadlines, atlas paging, preview caching,
live 7.41 playback, and release-mode GPU timing remain follow-up work.
