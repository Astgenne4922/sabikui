use std::fs;

use egui::{
    CornerRadius, Shadow, Stroke, Visuals,
    ecolor::ParseHexColorError,
    style::{Selection, TextCursorStyle, WidgetVisuals, Widgets},
};
use serde::{Deserialize, Serialize, de::Visitor};

use crate::gui::constants;

struct Color32(egui::Color32);

pub fn load_theme(mode: Mode) -> Option<Theme> {
    let file = dirs::config_dir()
        .expect("There should be a config directory")
        .join(constants::CONFIG_DIRECTORY)
        .join(constants::THEME_DIRECTORY);
    let file = match mode {
        Mode::Dark => file.join(constants::DARK_THEME_FILE),
        Mode::Light => file.join(constants::LIGHT_THEME_FILE),
    };
    fs::read(file)
        .ok()
        .and_then(|file| toml::from_slice(&file).ok())
        .map(|t| Theme { mode, ..t })
}

pub fn save_theme(theme: &Theme, mode: Mode) {
    let file = dirs::config_dir()
        .expect("There should be a config directory")
        .join(constants::CONFIG_DIRECTORY)
        .join(constants::THEME_DIRECTORY);
    if !file.exists() {
        fs::create_dir(&file).unwrap(); // TODO handle errors
    }

    let file = match mode {
        Mode::Dark => file.join(constants::DARK_THEME_FILE),
        Mode::Light => file.join(constants::LIGHT_THEME_FILE),
    };

    if !file.exists() {
        fs::write(
            file,
            toml::to_string_pretty(&theme).expect("The serialization should not fail"),
        )
        .unwrap();
    }
}

#[derive(Default, Deserialize, Serialize, Clone, Copy)]
pub enum Mode {
    #[default]
    Dark,
    Light,
}

#[derive(Deserialize, Serialize)]
pub struct Theme {
    widgets: WidgetsTheme,
    selection: SelectionTheme,
    hyperlink_color: Color32,
    faint_bg_color: Color32,
    extreme_bg_color: Color32,
    code_bg_color: Color32,
    warn_fg_color: Color32,
    error_fg_color: Color32,
    window_shadow: Color32,
    window_fill: Color32,
    window_stroke: Color32,
    panel_fill: Color32,
    popup_shadow: Color32,
    text_cursor: Color32,
    #[serde(skip)]
    mode: Mode,
}

impl Theme {
    pub const fn dark() -> Self {
        use crate::gui::constants::colors::gruvbox::dark::{
            BG, BG_0, BG_0_HARD, BG_0_SOFT, BG_1, BG_2, BG_3, BG_4, BLUE, BLUE_ALT, DARK_GRAY, FG_0, FG_2, FG_3, FG_4,
            ORANGE, RED,
        };

        Self {
            widgets: WidgetsTheme {
                noninteractive: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_0),
                    bg_fill: Color32(BG_0),
                    bg_stroke: Color32(BG_3),
                    fg_stroke: Color32(DARK_GRAY),
                },
                inactive: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_3),
                    bg_fill: Color32(BG_3),
                    bg_stroke: Color32(egui::Color32::TRANSPARENT),
                    fg_stroke: Color32(FG_3),
                },
                hovered: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_2),
                    bg_fill: Color32(BG_2),
                    bg_stroke: Color32(DARK_GRAY),
                    fg_stroke: Color32(FG_0),
                },
                active: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_2),
                    bg_fill: Color32(BG_2),
                    bg_stroke: Color32(FG_0),
                    fg_stroke: Color32(FG_0),
                },
                open: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_1),
                    bg_fill: Color32(BG_0),
                    bg_stroke: Color32(BG_3),
                    fg_stroke: Color32(FG_2),
                },
            },
            selection: SelectionTheme {
                bg_fill: Color32(BLUE_ALT),
                stroke: Color32(BG_0_SOFT),
            },
            hyperlink_color: Color32(BLUE),
            faint_bg_color: Color32(BG_0_SOFT),
            extreme_bg_color: Color32(BG_0_HARD),
            code_bg_color: Color32(BG_4),
            warn_fg_color: Color32(ORANGE),
            error_fg_color: Color32(RED),
            window_shadow: Color32(BG_1),
            window_fill: Color32(BG),
            window_stroke: Color32(BG_3),
            panel_fill: Color32(BG),
            popup_shadow: Color32(BG_1),
            text_cursor: Color32(FG_4),
            mode: Mode::Dark,
        }
    }

    pub const fn light() -> Self {
        use crate::gui::constants::colors::gruvbox::light::{
            BG, BG_0, BG_0_HARD, BG_0_SOFT, BG_1, BG_2, BG_3, BG_4, BLUE, BLUE_ALT, DARK_GRAY, FG_0, FG_2, FG_3, FG_4,
            ORANGE, RED,
        };

        Self {
            widgets: WidgetsTheme {
                noninteractive: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_0),
                    bg_fill: Color32(BG_0),
                    bg_stroke: Color32(BG_3),
                    fg_stroke: Color32(DARK_GRAY),
                },
                inactive: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_3),
                    bg_fill: Color32(BG_3),
                    bg_stroke: Color32(egui::Color32::TRANSPARENT),
                    fg_stroke: Color32(FG_3),
                },
                hovered: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_2),
                    bg_fill: Color32(BG_2),
                    bg_stroke: Color32(DARK_GRAY),
                    fg_stroke: Color32(FG_0),
                },
                active: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_2),
                    bg_fill: Color32(BG_2),
                    bg_stroke: Color32(FG_0),
                    fg_stroke: Color32(FG_0),
                },
                open: WidgetPropertyTheme {
                    weak_bg_fill: Color32(BG_1),
                    bg_fill: Color32(BG_0),
                    bg_stroke: Color32(BG_3),
                    fg_stroke: Color32(FG_2),
                },
            },
            selection: SelectionTheme {
                bg_fill: Color32(BLUE_ALT),
                stroke: Color32(BG_0_SOFT),
            },
            hyperlink_color: Color32(BLUE),
            faint_bg_color: Color32(BG_0_SOFT),
            extreme_bg_color: Color32(BG_0_HARD),
            code_bg_color: Color32(BG_4),
            warn_fg_color: Color32(ORANGE),
            error_fg_color: Color32(RED),
            window_shadow: Color32(BG_1),
            window_fill: Color32(BG),
            window_stroke: Color32(BG_3),
            panel_fill: Color32(BG),
            popup_shadow: Color32(BG_1),
            text_cursor: Color32(FG_4),
            mode: Mode::Light,
        }
    }

    pub fn visuals(&self) -> Visuals {
        let default = match self.mode {
            Mode::Dark => Visuals::dark(),
            Mode::Light => Visuals::light(),
        };

        Visuals {
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    weak_bg_fill: self.widgets.noninteractive.weak_bg_fill.0,
                    bg_fill: self.widgets.noninteractive.bg_fill.0,
                    bg_stroke: Stroke::new(1.0, self.widgets.noninteractive.bg_stroke.0),
                    fg_stroke: Stroke::new(1.0, self.widgets.noninteractive.fg_stroke.0),
                    corner_radius: CornerRadius::same(2),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    weak_bg_fill: self.widgets.inactive.weak_bg_fill.0,
                    bg_fill: self.widgets.inactive.bg_fill.0,
                    bg_stroke: Stroke::new(0.0, self.widgets.inactive.bg_stroke.0),
                    fg_stroke: Stroke::new(1.0, self.widgets.inactive.fg_stroke.0),
                    corner_radius: CornerRadius::same(2),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    weak_bg_fill: self.widgets.hovered.weak_bg_fill.0,
                    bg_fill: self.widgets.hovered.bg_fill.0,
                    bg_stroke: Stroke::new(1.0, self.widgets.hovered.bg_stroke.0),
                    fg_stroke: Stroke::new(1.5, self.widgets.hovered.fg_stroke.0),
                    corner_radius: CornerRadius::same(3),
                    expansion: 1.0,
                },
                active: WidgetVisuals {
                    weak_bg_fill: self.widgets.active.weak_bg_fill.0,
                    bg_fill: self.widgets.active.bg_fill.0,
                    bg_stroke: Stroke::new(1.0, self.widgets.active.bg_stroke.0),
                    fg_stroke: Stroke::new(2.0, self.widgets.active.fg_stroke.0),
                    corner_radius: CornerRadius::same(2),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    weak_bg_fill: self.widgets.open.weak_bg_fill.0,
                    bg_fill: self.widgets.open.bg_fill.0,
                    bg_stroke: Stroke::new(1.0, self.widgets.open.bg_stroke.0),
                    fg_stroke: Stroke::new(1.0, self.widgets.open.fg_stroke.0),
                    corner_radius: CornerRadius::same(2),
                    expansion: 0.0,
                },
            },
            selection: Selection {
                bg_fill: self.selection.bg_fill.0,
                stroke: Stroke::new(1.0, self.selection.stroke.0),
            },
            hyperlink_color: self.hyperlink_color.0,
            faint_bg_color: self.faint_bg_color.0,
            extreme_bg_color: self.extreme_bg_color.0,
            code_bg_color: self.code_bg_color.0,
            warn_fg_color: self.warn_fg_color.0,
            error_fg_color: self.error_fg_color.0,
            window_shadow: Shadow {
                offset: [10, 20],
                blur: 15,
                spread: 0,
                color: self.window_shadow.0,
            },
            window_fill: self.window_fill.0,
            window_stroke: Stroke::new(1.0, self.window_stroke.0),
            panel_fill: self.panel_fill.0,
            popup_shadow: Shadow {
                offset: [6, 10],
                blur: 8,
                spread: 0,
                color: self.popup_shadow.0,
            },
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, self.text_cursor.0),
                preview: false,
                blink: true,
                on_duration: 0.5,
                off_duration: 0.5,
            },
            striped: true,
            ..default
        }
    }
}

#[derive(Deserialize, Serialize)]
struct WidgetsTheme {
    noninteractive: WidgetPropertyTheme,
    inactive: WidgetPropertyTheme,
    hovered: WidgetPropertyTheme,
    active: WidgetPropertyTheme,
    open: WidgetPropertyTheme,
}

#[derive(Deserialize, Serialize)]
struct WidgetPropertyTheme {
    weak_bg_fill: Color32,
    bg_fill: Color32,
    bg_stroke: Color32,
    fg_stroke: Color32,
}

#[derive(Deserialize, Serialize)]
struct SelectionTheme {
    bg_fill: Color32,
    stroke: Color32,
}

impl Serialize for Color32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_hex())
    }
}

impl<'de> Deserialize<'de> for Color32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Color32Visitor)
    }
}

struct Color32Visitor;

impl Visitor<'_> for Color32Visitor {
    type Value = Color32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A string with a valid color hex representation (#RRGGBB)")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match egui::Color32::from_hex(v) {
            Ok(color) => Ok(Color32(color)),
            Err(error) => match error {
                ParseHexColorError::MissingHash => Err(E::custom(format!("Missing hash sign: {v}"))),
                ParseHexColorError::InvalidLength => Err(E::custom(format!("Invalid length: {v}"))),
                ParseHexColorError::InvalidInt(_) => Err(E::custom(format!("Invalid integer: {v}"))),
            },
        }
    }
}
