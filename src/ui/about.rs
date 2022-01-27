use std::{fs::File, io::Read};

use eframe::egui::{self, Align, Layout, RichText, TextureId, Ui, Vec2, Widget};

#[derive(Default)]
pub struct AboutPage {}

impl AboutPage {
    pub(crate) fn ui(&self, ui: &mut Ui) {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.allocate_space(ui.available_size() / 2.0 - Vec2::new(0.0, 64.0));
            Image::asset(ui.ctx(), "kodo_icon_rounded.png").ui(ui);
            ui.add_space(12.0);
            ui.label(RichText::new("Kodo Editor").heading());
            ui.add_space(4.0);
            ui.label(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).monospace());
        });
    }
}

struct Image {
    texture_id: TextureId,
    size: Vec2,
}

impl Image {
    pub fn asset(ctx: &egui::Context, path: &str) -> Self {
        let texture: &egui::TextureHandle = {
            let mut image_data = Vec::new();
            File::open(format!("assets/{}", path))
                .unwrap()
                .read_to_end(&mut image_data)
                .expect("could not load asset");
            let image = Self::load_image(&image_data).unwrap();
            &ctx.load_texture(path, image)
        };

        Self {
            texture_id: texture.id(),
            size: texture.size_vec2(),
        }
    }

    pub fn ui(&self, ui: &mut Ui) {
        egui::Image::new(self.texture_id, self.size).ui(ui);
    }

    fn load_image(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
        use image::GenericImageView as _;
        let image = image::load_from_memory(image_data)?;
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        Ok(egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        ))
    }
}
