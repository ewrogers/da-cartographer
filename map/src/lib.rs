use std::io::{Read, Write};
use std::path::Path;
use std::{cmp::Reverse, fs, io};

pub const TILE_WIDTH: f32 = 56.0;
pub const TILE_HEIGHT: f32 = 27.0;
pub const TILE_BYTES: usize = 6;
pub const MAX_TILES: usize = 1024 * 1024;
const MAX_MAP_BYTES: usize = MAX_TILES * TILE_BYTES;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tile {
    pub ground: u16,
    pub left_wall: u16,
    pub right_wall: u16,
}

#[derive(Clone, Debug)]
pub struct Map {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        let count = width as usize * height as usize;
        Self {
            width,
            height,
            tiles: vec![Tile::default(); count],
        }
    }

    /// Parse a map from raw binary tile data. Dimensions are guessed
    /// to be the most square factorization with width >= height.
    pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
        if !data.len().is_multiple_of(TILE_BYTES) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "map data length {} is not a multiple of {}",
                    data.len(),
                    TILE_BYTES
                ),
            ));
        }

        let tile_count = data.len() / TILE_BYTES;
        let (width, height) = guess_dimensions(tile_count).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "unsupported map tile count")
        })?;

        let mut tiles = Vec::with_capacity(tile_count);
        for i in 0..tile_count {
            let base = i * TILE_BYTES;
            let ground = u16::from_le_bytes([data[base], data[base + 1]]);
            let left_wall = u16::from_le_bytes([data[base + 2], data[base + 3]]);
            let right_wall = u16::from_le_bytes([data[base + 4], data[base + 5]]);
            tiles.push(Tile {
                ground,
                left_wall,
                right_wall,
            });
        }

        Ok(Self {
            width,
            height,
            tiles,
        })
    }

    /// Serialize the map back to raw binary tile data.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.tiles.len() * TILE_BYTES);
        for tile in &self.tiles {
            data.extend_from_slice(&tile.ground.to_le_bytes());
            data.extend_from_slice(&tile.left_wall.to_le_bytes());
            data.extend_from_slice(&tile.right_wall.to_le_bytes());
        }
        data
    }

    /// Load a map from a `.map` file.
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut data = Vec::new();
        fs::File::open(path)?
            .take(MAX_MAP_BYTES as u64 + 1)
            .read_to_end(&mut data)?;
        Self::from_bytes(&data)
    }

    /// Save the map to a `.map` file.
    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        if validate_dimensions(self.width, self.height)? != self.tiles.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "map tile count mismatch",
            ));
        }
        write_atomic(path.as_ref(), |file| file.write_all(&self.to_bytes()))
    }

    /// Returns the tile at (col, row), or None if out of bounds.
    pub fn get(&self, col: u16, row: u16) -> Option<&Tile> {
        if col < self.width && row < self.height {
            self.tiles
                .get(row as usize * self.width as usize + col as usize)
        } else {
            None
        }
    }
}

/// Replace a file only after the complete new contents have been written.
pub fn write_atomic(
    path: &Path,
    write: impl FnOnce(&mut fs::File) -> io::Result<()>,
) -> io::Result<()> {
    let resolved;
    let path = match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_symlink() => {
            resolved = fs::canonicalize(path)?;
            &resolved
        }
        _ => path,
    };
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty());
    let mut temporary = tempfile::NamedTempFile::new_in(parent.unwrap_or(Path::new(".")))?;
    if let Ok(metadata) = fs::metadata(path) {
        if !metadata.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "destination is not a file",
            ));
        }
        fs::OpenOptions::new().write(true).open(path)?;
        temporary
            .as_file()
            .set_permissions(metadata.permissions())?;
    }
    write(temporary.as_file_mut())?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    Ok(())
}

pub fn validate_dimensions(width: u16, height: u16) -> io::Result<usize> {
    let count = usize::from(width) * usize::from(height);
    if count == 0 || count > MAX_TILES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Map dimensions must contain between 1 and {MAX_TILES} tiles."),
        ));
    }
    Ok(count)
}

/// Returns representable factor pairs, from most square to most elongated.
pub fn all_dimensions(n: usize) -> Vec<(u16, u16)> {
    if n == 0 || n > MAX_TILES {
        return Vec::new();
    }
    let mut pairs = Vec::new();
    for divisor in 1..=n.isqrt() {
        if n.is_multiple_of(divisor)
            && let (Ok(a), Ok(b)) = (u16::try_from(divisor), u16::try_from(n / divisor))
        {
            pairs.push((a, b));
            if a != b {
                pairs.push((b, a));
            }
        }
    }
    pairs.sort_by_key(|&(w, h)| (w.abs_diff(h), Reverse(w >= h), w.max(h), w.min(h)));
    pairs
}

fn guess_dimensions(n: usize) -> Option<(u16, u16)> {
    all_dimensions(n).into_iter().find(|(w, h)| w >= h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unrepresentable_dimensions() {
        let bytes = vec![0; 65_537 * TILE_BYTES];
        assert!(Map::from_bytes(&bytes).is_err());
        assert!(Map::from_bytes(&[]).is_err());
        assert!(validate_dimensions(1025, 1024).is_err());
    }

    #[test]
    fn failed_write_preserves_original_contents() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("map");
        fs::write(&path, b"original").unwrap();
        let result = write_atomic(&path, |file| {
            file.write_all(b"partial")?;
            Err(io::Error::other("test write failure"))
        });
        assert!(result.is_err());
        assert_eq!(fs::read(&path).unwrap(), b"original");
        assert_eq!(directory.path().read_dir().unwrap().count(), 1);
    }

    #[test]
    fn test_guess_dimensions_perfect_square() {
        assert_eq!(guess_dimensions(2500), Some((50, 50)));
        assert_eq!(guess_dimensions(10000), Some((100, 100)));
        assert_eq!(guess_dimensions(400), Some((20, 20)));
    }

    #[test]
    fn test_guess_dimensions_rectangle() {
        // 2250 = 50*45, which is closer to square than 75*30 etc.
        assert_eq!(guess_dimensions(2250), Some((50, 45)));
        // 75 = 15*5
        assert_eq!(guess_dimensions(75), Some((15, 5)));
    }

    #[test]
    fn test_all_dimensions() {
        // 2500 = 50*50, many factors
        let dims = all_dimensions(2500);
        assert_eq!(dims[0], (50, 50)); // most square first
        assert!(dims.contains(&(25, 100)));
        assert!(dims.contains(&(100, 25)));
        assert!(dims.contains(&(2500, 1)));
        assert!(dims.contains(&(1, 2500)));
        // all pairs valid
        for &(w, h) in &dims {
            assert_eq!(w as usize * h as usize, 2500);
        }
    }

    #[test]
    fn test_all_dimensions_prime() {
        // 7 is prime → both orientations, wider first
        assert_eq!(all_dimensions(7), vec![(7, 1), (1, 7)]);
    }

    #[test]
    fn test_roundtrip() {
        let mut map = Map::new(10, 8);
        map.tiles[0].ground = 42;
        map.tiles[79].right_wall = 99;
        let data = map.to_bytes();
        assert_eq!(data.len(), 10 * 8 * 6);
        let loaded = Map::from_bytes(&data).unwrap();
        assert_eq!(loaded.tiles[0].ground, 42);
        assert_eq!(loaded.tiles[79].right_wall, 99);
    }
}
