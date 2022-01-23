#![allow(non_snake_case)]
use eframe::egui::{
    epaint::Shadow,
    style::{self, WidgetVisuals},
    Color32, Stroke, Visuals,
};

pub const BORDER: u32 = 0xFF373B42;
pub const BORDER_HOVER: u32 = 0xFF8B949E;
pub const HYPERLINK: u32 = 0xFF58A6FF;
pub const BACKGROUND: u32 = 0xFF0D1117;
pub const BACKGROUND_HOVER: u32 = 0xFF30363D;
pub const BACKGROUND_FOCUS: u32 = 0xFF21262D;
pub const FOREGROUND: u32 = 0xFFF0F6FC;
pub const BORDER_RADIUS: f32 = 6.0;

enum ColorByte {
    Red = 2,
    Green = 1,
    Blue = 0,
    Alpha = 3,
}

pub fn Color(color: u32) -> Color32 {
    fn get_byte(n: u32, byte: ColorByte) -> u8 {
        (n >> (byte as u8 * 8) & 0xff) as u8
    }

    let r = get_byte(color, ColorByte::Red);
    let g = get_byte(color, ColorByte::Green);
    let b = get_byte(color, ColorByte::Blue);
    let a = get_byte(color, ColorByte::Alpha);

    Color32::from_rgba_unmultiplied(r, g, b, a)
}

fn WidgetStyle(style: WidgetStyleType) -> WidgetVisuals {
    let stroke_color = match style {
        WidgetStyleType::Hovered => Color(BORDER_HOVER),
        WidgetStyleType::Active => Color(BORDER_HOVER),
        _ => Color(BORDER),
    };

    let bg_color = match style {
        WidgetStyleType::Hovered => Color(BACKGROUND_HOVER),
        WidgetStyleType::Active => Color(BACKGROUND_FOCUS),
        WidgetStyleType::Inactive => Color(BACKGROUND_FOCUS),
        _ => Color(BACKGROUND),
    };

    WidgetVisuals {
        bg_fill: bg_color,
        bg_stroke: Stroke::new(1.0, stroke_color),
        corner_radius: BORDER_RADIUS,
        fg_stroke: Stroke::new(1.0, Color(FOREGROUND)),
        expansion: 0.0,
    }
}

enum WidgetStyleType {
    NonInteractive,
    Inactive,
    Hovered,
    Active,
    Open,
}

pub fn dark() -> Visuals {
    Visuals {
        dark_mode: true,
        override_text_color: None,
        widgets: style::Widgets {
            noninteractive: WidgetStyle(WidgetStyleType::NonInteractive),
            inactive: WidgetStyle(WidgetStyleType::Inactive),
            hovered: WidgetStyle(WidgetStyleType::Hovered),
            active: WidgetStyle(WidgetStyleType::Active),
            open: WidgetStyle(WidgetStyleType::Open),
        },
        selection: style::Selection {
            bg_fill: Color(0x22FFFFFF),
            stroke: Stroke::new(1.0, Color(BORDER_HOVER)),
        },
        hyperlink_color: Color(HYPERLINK),
        faint_bg_color: Color32::from_rgb(255, 255, 0),
        extreme_bg_color: Color32::TRANSPARENT,
        code_bg_color: Color32::from_rgb(255, 255, 255),
        window_corner_radius: BORDER_RADIUS,
        window_shadow: Shadow::big_dark(),
        popup_shadow: Shadow::small_dark(),
        resize_corner_size: 12.0,
        text_cursor_width: 2.0,
        text_cursor_preview: false,
        clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
        button_frame: true,
        collapsing_header_frame: false,
    }
}
