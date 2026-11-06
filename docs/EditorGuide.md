# Using DA Cartographer

## Set up your artwork

Choose your Dark Ages installation folder on first launch, or put your
`.dat` archives in the app's `assets` folder. The editor reads archives in
alphabetical order. When several contain the same asset, the last archive
wins, regardless of capitalization.

The editor uses `TILEA.BMP` for ground tiles and `stcNNNNN.hpf` for walls
and objects. It can use the older `legend.pal` palette, the newer `mpt*` and
`stc*` palette tables, or a mix. `gndani.tbl` and `stcani.tbl` enable
ground and wall animation. `SOTP.DAT` enables the collision overlay,
the inspector's tab map, and collision PNG export.

## Open and edit a map

Open a `.map` file with the folder button or drop it onto the window. Each
file contains tiles without dimensions, so the editor looks in `maps.ron`
for a matching map number, such as `LOD185.MAP` for map 185. Check the
[map reference](MapList.md) for known dimensions. If the map still looks
wrong, click its dimensions in the status bar and choose the correct width
and height. Shrinking the tile count can remove tiles. Maps and prefabs
support up to 1,048,576 tiles.

| Action | Shortcut |
| --- | --- |
| Open, create, save, export | Ctrl or Cmd + O, N, S, E |
| Undo, redo | Ctrl or Cmd + Z, Shift + Ctrl or Cmd + Z |
| Copy, cut, paste | Ctrl or Cmd + C, X, V |
| Select, brush, line, shape | V, B, L, U |
| Erase, fill, sample a tile, place a prefab | E, G, I, P |
| Switch ground or wall paint target | T |
| Switch left or right wall target | Q |
| Toggle collision overlay | Tab |

Drag with **Select** to mark a rectangle. Drag inside the selection to
move it or hold **Shift** to copy it. The selection respects visible
layers and uses the visible wall layers when any are shown. Hide both
wall layers to work on ground alone.

## Reuse a piece of a map

Right-click a selection and choose **Create Prefab** to save a building,
tree, or other arrangement for reuse. Use the **Prefab** tool to browse,
preview, and place saved pieces. The library also lets you import a prefab
`.ron` file or edit it in its own tab. Empty layers in a prefab let the
underlying map show through when you place it. Prefabs live in the
app's `prefabs` folder.

## Export an image

Choose **Export** to save a PNG. Pick the scale and leave the background
transparent or choose a color. If collision data is available, you can
also export the tab map as a separate PNG. You can keep editing during
export. If the image is too large, pick a smaller scale.
