use eframe::egui;

fn draw_tabler(
    painter: &egui::Painter,
    rect: egui::Rect,
    color: egui::Color32,
    name: &'static str,
    png: &[u8],
) {
    let ctx = painter.ctx();
    let id = egui::Id::new(("toolbar_icon", name));
    let cached = ctx.data(|data| data.get_temp::<egui::TextureHandle>(id));
    let texture = match cached {
        Some(texture) => texture,
        None => {
            let Ok(icon) = eframe::icon_data::from_png_bytes(png) else {
                return;
            };
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [icon.width as usize, icon.height as usize],
                &icon.rgba,
            );
            let texture = ctx.load_texture(name, image, egui::TextureOptions::LINEAR);
            ctx.data_mut(|data| data.insert_temp(id, texture.clone()));
            texture
        }
    };
    let side = rect.width().min(rect.height()) * 0.72;
    let bounds = egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(side));
    let uv = egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0));
    painter.image(texture.id(), bounds, uv, color);
}

pub fn draw_icon_new(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "file-plus",
        include_bytes!("../../icons/file-plus.png"),
    );
}

pub fn draw_icon_open(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "folder-open",
        include_bytes!("../../icons/folder-open.png"),
    );
}

pub fn draw_icon_save(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "device-floppy",
        include_bytes!("../../icons/device-floppy.png"),
    );
}

pub fn draw_icon_export(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "external-link",
        include_bytes!("../../icons/external-link.png"),
    );
}

pub fn draw_icon_select(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "marquee",
        include_bytes!("../../icons/marquee.png"),
    );
}

pub fn draw_icon_pencil(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "brush",
        include_bytes!("../../icons/brush.png"),
    );
}

pub fn draw_icon_eraser(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "eraser",
        include_bytes!("../../icons/eraser.png"),
    );
}

pub fn draw_icon_fill(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "bucket-droplet",
        include_bytes!("../../icons/bucket-droplet.png"),
    );
}

pub fn draw_icon_eyedropper(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "color-picker",
        include_bytes!("../../icons/color-picker.png"),
    );
}

/// Draw a rectangle outline (Rectangle tool).
pub fn draw_icon_rectangle(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "rectangle",
        include_bytes!("../../icons/rectangle.png"),
    );
}

/// Draw a filled rectangle (Solid Rectangle tool).
pub fn draw_icon_rectangle_solid(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "rectangle-filled",
        include_bytes!("../../icons/rectangle-filled.png"),
    );
}

/// Draw a composite shapes icon (Shape tool).
pub fn draw_icon_shapes(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "shape",
        include_bytes!("../../icons/shape.png"),
    );
}

pub fn draw_icon_prefab(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "box",
        include_bytes!("../../icons/box.png"),
    );
}

/// Draw a square outline (Square shape tool).
pub fn draw_icon_square(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "square",
        include_bytes!("../../icons/square.png"),
    );
}

/// Draw a filled square (Solid Square tool).
pub fn draw_icon_square_solid(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "square-filled",
        include_bytes!("../../icons/square-filled.png"),
    );
}

/// Draw a circle outline (Circle shape tool).
pub fn draw_icon_circle(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "circle",
        include_bytes!("../../icons/circle.png"),
    );
}

/// Draw a filled circle (Solid Circle tool).
pub fn draw_icon_circle_solid(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "circle-filled",
        include_bytes!("../../icons/circle-filled.png"),
    );
}

/// Draw an upright triangle outline (Triangle shape tool).
pub fn draw_icon_triangle(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "triangle",
        include_bytes!("../../icons/triangle.png"),
    );
}

/// Draw a filled upright triangle (Solid Triangle tool).
pub fn draw_icon_triangle_solid(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "triangle-filled",
        include_bytes!("../../icons/triangle-filled.png"),
    );
}

pub fn draw_icon_line(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "line",
        include_bytes!("../../icons/line.png"),
    );
}

pub fn draw_icon_undo(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "arrow-back-up",
        include_bytes!("../../icons/arrow-back-up.png"),
    );
}

pub fn draw_icon_redo(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "arrow-forward-up",
        include_bytes!("../../icons/arrow-forward-up.png"),
    );
}

pub fn draw_icon_zoom(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "zoom",
        include_bytes!("../../icons/zoom.png"),
    );
}

pub fn draw_icon_ruler(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "ruler",
        include_bytes!("../../icons/ruler.png"),
    );
}

pub fn draw_icon_crosshair(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "crosshair",
        include_bytes!("../../icons/crosshair.png"),
    );
}

pub fn draw_icon_grid_3x3(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "grid-3x3",
        include_bytes!("../../icons/grid-3x3.png"),
    );
}

pub fn draw_icon_map_2(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "map-2",
        include_bytes!("../../icons/map-2.png"),
    );
}

pub fn draw_icon_alert_triangle(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "alert-triangle",
        include_bytes!("../../icons/alert-triangle.png"),
    );
}

pub type Icon = fn(&egui::Painter, egui::Rect, egui::Color32);

pub fn show_icon(ui: &mut egui::Ui, size: f32, color: egui::Color32, icon: Icon) {
    let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(size), egui::Sense::hover());
    icon(ui.painter(), rect, color);
}

pub fn dialog_title(ui: &mut egui::Ui, color: egui::Color32, icon: Icon, title: &str) {
    ui.horizontal(|ui| {
        show_icon(ui, 24.0, color, icon);
        ui.label(egui::RichText::new(title).size(18.0).strong().color(color));
    });
}

pub fn draw_icon_scissors(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "scissors",
        include_bytes!("../../icons/scissors.png"),
    );
}

pub fn draw_icon_copy(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "copy",
        include_bytes!("../../icons/copy.png"),
    );
}

pub fn draw_icon_clipboard(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "clipboard",
        include_bytes!("../../icons/clipboard.png"),
    );
}

pub fn draw_icon_trash(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "trash",
        include_bytes!("../../icons/trash.png"),
    );
}

pub fn draw_icon_edit(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "pencil",
        include_bytes!("../../icons/pencil.png"),
    );
}

pub fn draw_icon_search(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "search",
        include_bytes!("../../icons/search.png"),
    );
}

pub fn draw_icon_barrier_block(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "barrier-block",
        include_bytes!("../../icons/barrier-block.png"),
    );
}

pub fn draw_icon_x(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "x",
        include_bytes!("../../icons/x.png"),
    );
}

pub fn draw_icon_minus(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "minus",
        include_bytes!("../../icons/minus.png"),
    );
}

pub fn draw_icon_plus(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "plus",
        include_bytes!("../../icons/plus.png"),
    );
}

pub fn draw_icon_chevron_down(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "chevron-down",
        include_bytes!("../../icons/chevron-down.png"),
    );
}

pub fn draw_icon_chevron_left(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "chevron-left",
        include_bytes!("../../icons/chevron-left.png"),
    );
}

pub fn draw_icon_chevron_right(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "chevron-right",
        include_bytes!("../../icons/chevron-right.png"),
    );
}

pub fn draw_icon_eye(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "eye",
        include_bytes!("../../icons/eye.png"),
    );
}

pub fn draw_icon_layer_ground(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "layer-ground",
        include_bytes!("../../icons/layer-ground.png"),
    );
}

pub fn draw_icon_layer_left(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "layer-left",
        include_bytes!("../../icons/layer-left.png"),
    );
}

pub fn draw_icon_layer_right(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    draw_tabler(
        painter,
        rect,
        color,
        "layer-right",
        include_bytes!("../../icons/layer-right.png"),
    );
}
