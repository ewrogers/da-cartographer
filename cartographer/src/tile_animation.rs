use std::collections::HashMap;

#[derive(Clone, Debug)]
struct TileAnimationEntry {
    tile_sequence: Box<[u16]>,
    animation_interval_ms: u32,
}

#[derive(Clone, Debug, Default)]
pub struct TileAnimationTable {
    entries: Vec<TileAnimationEntry>,
    lookup: HashMap<u16, (usize, usize)>,
}

impl TileAnimationTable {
    /// Parses a Chaos tile animation table such as `gndani.tbl` or `stcani.tbl`.
    ///
    /// Tile IDs are kept in the raw asset ID space used by the reference tools.
    pub fn from_tbl_bytes(bytes: &[u8]) -> Result<Self, String> {
        let text = String::from_utf8_lossy(bytes);
        let mut table = Self::default();

        for line in text.lines() {
            let line = line.split("//").next().unwrap_or(line);
            let line = line.split(['#', ';']).next().unwrap_or(line);
            let Ok(values) = line
                .split_whitespace()
                .map(str::parse::<u16>)
                .collect::<Result<Vec<_>, _>>()
            else {
                continue;
            };
            let Some((&delay, tile_ids)) = values.split_last() else {
                continue;
            };
            if tile_ids.len() < 2 || delay == 0 {
                continue;
            }

            let animation_interval_ms = u32::from(delay) * 100;
            let tile_sequence: Vec<u16> = tile_ids.to_vec();
            let entry_index = table.entries.len();

            for (sequence_index, tile_id) in tile_sequence.iter().copied().enumerate() {
                table.lookup.insert(tile_id, (entry_index, sequence_index));
            }

            table.entries.push(TileAnimationEntry {
                tile_sequence: tile_sequence.into_boxed_slice(),
                animation_interval_ms,
            });
        }

        if table.entries.is_empty() {
            return Err("tile animation table contained no valid cycles".to_string());
        }

        Ok(table)
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn minimum_interval_ms(&self) -> Option<u32> {
        self.entries
            .iter()
            .map(|entry| entry.animation_interval_ms)
            .min()
    }

    pub fn animation_metadata(&self, tile_id: u16) -> Option<(usize, u32)> {
        let &(entry_index, _) = self.lookup.get(&tile_id)?;
        let entry = &self.entries[entry_index];
        Some((entry.tile_sequence.len(), entry.animation_interval_ms))
    }

    pub fn representative_tile_id(&self, tile_id: u16, available: impl Fn(u16) -> bool) -> u16 {
        self.tile_sequence(tile_id)
            .and_then(|sequence| sequence.iter().copied().find(|&frame| available(frame)))
            .unwrap_or(tile_id)
    }

    pub fn tile_sequence(&self, tile_id: u16) -> Option<&[u16]> {
        let &(entry_index, _) = self.lookup.get(&tile_id)?;
        Some(&self.entries[entry_index].tile_sequence)
    }

    pub fn animated_tile_id(&self, tile_id: u16, time_seconds: f64) -> u16 {
        let Some(&(entry_index, current_index)) = self.lookup.get(&tile_id) else {
            return tile_id;
        };
        let entry = &self.entries[entry_index];
        if entry.tile_sequence.len() <= 1 {
            return tile_id;
        }

        let elapsed_ms = (time_seconds.max(0.0) * 1000.0).floor() as u64;
        let frame_offset = (elapsed_ms / u64::from(entry.animation_interval_ms))
            % entry.tile_sequence.len() as u64;
        let sequence_index = (current_index + frame_offset as usize) % entry.tile_sequence.len();

        entry.tile_sequence[sequence_index]
    }
}

/// Ground map IDs are one-based, while the animation table and atlas are zero-based.
pub fn ground_tile(
    table: Option<&TileAnimationTable>,
    atlas: &render::TileAtlas,
    id: u16,
    time: f64,
) -> u16 {
    let Some(index) = id.checked_sub(1) else {
        return 0;
    };
    u16::try_from(ground_index(table, atlas, u32::from(index), time) + 1).unwrap_or(id)
}

pub fn ground_index(
    table: Option<&TileAnimationTable>,
    atlas: &render::TileAtlas,
    index: u32,
    time: f64,
) -> u32 {
    let Some((table, raw_id)) = table.zip(u16::try_from(index).ok()) else {
        return index;
    };
    let frame = u32::from(table.animated_tile_id(raw_id, time));
    if frame < atlas.tile_count() {
        frame
    } else {
        index
    }
}

/// Walls use the map ID directly, with zero reserved for an empty layer.
pub fn wall_tile(
    table: Option<&TileAnimationTable>,
    atlas: &render::SpriteAtlas,
    id: u16,
    time: f64,
) -> u16 {
    let Some(table) = table else { return id };
    if id == 0 {
        return 0;
    }
    let frame = table.animated_tile_id(id, time);
    if frame != 0 && atlas.sprite_height(u32::from(frame)) > 0 {
        frame
    } else {
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ground_frames_use_zero_based_ids_and_keep_base_art_when_missing() {
        let table = TileAnimationTable::from_tbl_bytes(b"0 1 2 5").unwrap();
        let palette = render::Palette::from_bytes(&[40; 768]).unwrap();
        let atlas = render::TileAtlas::from_raw(&[1; 2], &palette, 1, 1).unwrap();

        assert_eq!(ground_tile(Some(&table), &atlas, 0, 0.5), 0);
        assert_eq!(ground_tile(Some(&table), &atlas, 1, 0.5), 2);
        assert_eq!(ground_tile(Some(&table), &atlas, 1, 1.0), 1);
        assert_eq!(ground_tile(Some(&table), &atlas, 2, 1.0), 1);
    }

    #[test]
    fn wall_frames_keep_phase_and_fall_back_when_missing() {
        let table = TileAnimationTable::from_tbl_bytes(b"13 14 15 2").unwrap();
        let palette = render::Palette::from_bytes(&[40; 768]).unwrap();
        let mut sprites: Vec<_> = (0..15).map(|_| None).collect();
        for slot in &mut sprites[13..] {
            *slot = Some(render::HpfSprite {
                width: 28,
                height: 1,
                pixels: vec![1; 28],
            });
        }
        let atlas = render::SpriteAtlas::build(&sprites, &palette, 28).unwrap();

        assert_eq!(wall_tile(Some(&table), &atlas, 13, 0.2), 14);
        assert_eq!(wall_tile(Some(&table), &atlas, 14, 0.2), 14);
        assert_eq!(wall_tile(Some(&table), &atlas, 14, 0.4), 13);
    }

    #[test]
    fn skips_invalid_rows_and_ignores_comment_numbers() {
        let table = TileAnimationTable::from_tbl_bytes(
            b"0 1 5 # comment 99\n12 invalid 13 4\n20 21 0\n22 1",
        )
        .unwrap();
        assert_eq!(table.entry_count(), 1);
        assert_eq!(table.minimum_interval_ms(), Some(500));
    }

    #[test]
    fn parses_sequences_and_intervals() {
        let table = TileAnimationTable::from_tbl_bytes(b"0 1 2 5\n10 11 2\n").unwrap();

        assert_eq!(table.entry_count(), 2);
        assert_eq!(table.minimum_interval_ms(), Some(200));
    }

    #[test]
    fn rotates_from_current_tile() {
        let table = TileAnimationTable::from_tbl_bytes(b"0 1 2 5\n").unwrap();

        assert_eq!(table.animated_tile_id(1, 0.0), 1);
        assert_eq!(table.animated_tile_id(1, 0.5), 2);
        assert_eq!(table.animated_tile_id(1, 1.0), 0);
    }

    #[test]
    fn leaves_non_animated_tiles_unchanged() {
        let table = TileAnimationTable::from_tbl_bytes(b"0 1 2 5\n").unwrap();

        assert_eq!(table.animated_tile_id(99, 2.0), 99);
        assert_eq!(table.animated_tile_id(0, 0.5), 1);
    }

    #[test]
    fn representative_tile_is_first_sequence_frame() {
        let table = TileAnimationTable::from_tbl_bytes(b"10 11 12 2\n").unwrap();

        assert_eq!(table.representative_tile_id(11, |_| true), 10);
        assert_eq!(table.representative_tile_id(12, |id| id != 10), 11);
        assert_eq!(table.representative_tile_id(99, |_| true), 99);
    }
}
