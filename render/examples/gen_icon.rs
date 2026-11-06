//! Draws the skeletal octagram etched into the slate release icon.

use std::f32::consts::TAU;

use std::fs::File;
use std::io::{self, BufWriter};

const SIZE: u32 = 512;
const ART_SIZE: u32 = 128;
const PIXEL: u32 = SIZE / ART_SIZE;
const OUTPUT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../cartographer/app-icon.png");

type Point = [f32; 2];
type Color = [u8; 3];

const SLATE: Color = [76, 85, 86];
const CUT: Color = [29, 36, 39];
const BEVEL: Color = [113, 128, 124];

const INLAY_SHADOW: Color = [61, 101, 96];
const TEAL: Color = [151, 185, 174];
const TEAL_LIGHT: Color = [194, 219, 208];

fn main() -> io::Result<()> {
    let outline: [Point; 17] = std::array::from_fn(|index| {
        let angle = TAU * (index % 16) as f32 / 16.0 - TAU / 4.0;
        let radius = if index % 2 == 0 { 193.0 } else { 67.0 };
        [256.0 + radius * angle.cos(), 256.0 + radius * angle.sin()]
    });
    let mut pixels = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for y in 0..SIZE {
        for x in 0..SIZE {
            pixels.extend(pixel(x, y, &outline));
        }
    }

    write_icon(OUTPUT, &pixels)?;
    let mut title = Vec::with_capacity(pixels.len());
    for y in 0..SIZE {
        for x in 0..SIZE {
            let point = [x as f32 + 0.5, y as f32 + 0.5];
            let alpha = if near_path(&outline, point, 13.0) {
                255
            } else {
                0
            };
            title.extend([TEAL[0], TEAL[1], TEAL[2], alpha]);
        }
    }
    write_icon(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../cartographer/title-icon.png"
        ),
        &title,
    )
}

fn write_icon(path: &str, pixels: &[u8]) -> io::Result<()> {
    let mut encoder = png::Encoder::new(BufWriter::new(File::create(path)?), SIZE, SIZE);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(pixels)?;
    writer.finish()?;
    Ok(())
}

fn pixel(x: u32, y: u32, outline: &[Point]) -> [u8; 4] {
    // Smooth the outer mask while sampling the artwork on the coarser grid.
    let covered = [[0.25, 0.25], [0.75, 0.25], [0.25, 0.75], [0.75, 0.75]]
        .into_iter()
        .filter(|&[dx, dy]| inside_icon(x as f32 + dx, y as f32 + dy))
        .count();
    if covered == 0 {
        return [0; 4];
    }

    let (col, row) = (x / PIXEL, y / PIXEL);
    let center = |cell| (cell * PIXEL) as f32 + PIXEL as f32 / 2.0;
    let [red, green, blue] = sample([center(col), center(row)], col, row, outline);
    [red, green, blue, (covered * 255 / 4) as u8]
}

fn inside_icon(x: f32, y: f32) -> bool {
    let corner = 106.0;
    let inset = SIZE as f32 - corner;
    (x.clamp(corner, inset) - x).hypot(y.clamp(corner, inset) - y) <= corner
}

fn sample(point: Point, col: u32, row: u32, outline: &[Point]) -> Color {
    let radius = (point[0] - 256.0).hypot(point[1] - 256.0);
    if near_path(outline, point, 7.5) {
        return if point[0] + point[1] < 512.0 {
            TEAL_LIGHT
        } else {
            TEAL
        };
    }
    if near_path(outline, point, 12.5) {
        return INLAY_SHADOW;
    }
    if near_path(outline, point, 18.0) {
        return CUT;
    }
    if (153.0..=163.0).contains(&radius) {
        return CUT;
    }
    if (163.0..=169.0).contains(&radius) && point[0] + point[1] > 512.0 {
        return BEVEL;
    }

    let noise = col.wrapping_mul(0x85EB_CA6B) ^ row.wrapping_mul(0xC2B2_AE35);
    let grain = (noise ^ (noise >> 13)) % 27;
    let light = match grain {
        0..=2 => 14,
        3..=6 => -13,
        _ => 0,
    };
    let slope = (12.0 - (point[0] + point[1]) / 42.0) as i16;
    SLATE.map(|channel| (i16::from(channel) + light + slope) as u8)
}

fn near_path(path: &[Point], point: Point, width: f32) -> bool {
    path.windows(2)
        .any(|segment| near_segment(segment[0], segment[1], point, width))
}

fn near_segment([ax, ay]: Point, [bx, by]: Point, [x, y]: Point, width: f32) -> bool {
    let (dx, dy) = (bx - ax, by - ay);
    let t = (((x - ax) * dx + (y - ay) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    (x - ax - t * dx).hypot(y - ay - t * dy) <= width
}
