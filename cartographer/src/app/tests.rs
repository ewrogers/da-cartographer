use super::*;

fn write_archive(path: &Path, entries: &[(&str, &[u8])]) {
    let count = entries.len() as u32 + 1;
    let mut data = count.to_le_bytes().to_vec();
    let mut offset = 4 + count * 17;
    for &(name, bytes) in entries {
        data.extend(offset.to_le_bytes());
        let mut name_field = [0; 13];
        name_field[..name.len()].copy_from_slice(name.as_bytes());
        data.extend(name_field);
        offset += bytes.len() as u32;
    }
    data.extend(offset.to_le_bytes());
    data.extend([0; 13]);
    for &(_, bytes) in entries {
        data.extend(bytes);
    }
    fs::write(path, data).unwrap();
}

#[test]
fn loads_sparse_wall_banks_and_mixed_case_overrides() {
    let dir = tempfile::tempdir().unwrap();
    let base = [vec![0; 8], vec![1; 28]].concat();
    let patch = [vec![0; 8], vec![1; 56]].concat();
    write_archive(
        &dir.path().join("base.DAT"),
        &[("stc00013.hpf", &base), ("STC10013.HPF", &base)],
    );
    write_archive(&dir.path().join("patch.dat"), &[("Stc10013.Hpf", &patch)]);
    let pool = archive::AssetPool::load(dir.path()).unwrap();
    let palette = render::Palette::from_bytes(&[33; 768]).unwrap();
    let atlas = EditorApp::load_wall_atlas(&pool, |_| &palette).unwrap();

    assert_eq!(atlas.sprite_height(13), 1);
    assert_eq!(atlas.sprite_height(10_013), 2);
}

#[test]
fn wall_animation_table_is_separate_from_palette_tables() {
    let dir = tempfile::tempdir().unwrap();
    write_archive(
        &dir.path().join("sample.dat"),
        &[
            ("stcpal.tbl", b"15 2\n"),
            ("stcani.tbl", b"15 16 5\n"),
            ("stc00000.pal", &[33; 768]),
            ("stc00002.pal", &[92; 768]),
            ("stc00005.pal", &[155; 768]),
        ],
    );
    let pool = archive::AssetPool::load(dir.path()).unwrap();
    let lookup = LoadedPaletteLookup::from_pool(&pool, "stc").unwrap();
    assert_eq!(
        lookup.palette_for_id(15).unwrap().color(1),
        [92, 92, 92, 255]
    );
    assert_eq!(
        lookup.palette_for_id(16).unwrap().color(1),
        [33, 33, 33, 255]
    );
}
