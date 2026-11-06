fn main() {
    println!("cargo:rerun-if-changed=app-icon.png");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        embed_windows_icon().expect("Failed to embed the Windows application icon");
    }
}

fn embed_windows_icon() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::fs::File;
    use std::path::PathBuf;

    const ICON_SIZES: [u32; 4] = [16, 32, 48, 256];
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let generated_ico = out_dir.join("app-icon.ico");

    let image = image::open("app-icon.png")?.into_rgba8();
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    for size in ICON_SIZES {
        let resized = image::imageops::resize(&image, size, size, image::imageops::Lanczos3);
        let frame = ico::IconImage::from_rgba_data(size, size, resized.into_raw());
        icon_dir.add_entry(ico::IconDirEntry::encode(&frame)?);
    }
    icon_dir.write(File::create(&generated_ico)?)?;

    winresource::WindowsResource::new()
        .set("ProductName", "DA Cartographer")
        .set("FileDescription", "DA Cartographer")
        .set("OriginalFilename", "cartographer.exe")
        .set_icon(generated_ico.to_string_lossy().as_ref())
        .compile()?;
    Ok(())
}
