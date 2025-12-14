#[macro_export]
macro_rules! gruvbox {
    ($mode:ident) => {{
        use egui::{
            CornerRadius, Shadow, Stroke, Visuals,
            style::{Selection, TextCursorStyle, WidgetVisuals, Widgets},
        };
        use $crate::gui::data::constants::colors::gruvbox::$mode::*;
        Visuals {
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    weak_bg_fill: BG_0,
                    bg_fill: BG_0,
                    bg_stroke: Stroke::new(1.0, BG_3),
                    fg_stroke: Stroke::new(1.0, DARK_GRAY),
                    corner_radius: CornerRadius::same(2),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    weak_bg_fill: BG_3,
                    bg_fill: BG_3,
                    bg_stroke: Default::default(),
                    fg_stroke: Stroke::new(1.0, FG_3),
                    corner_radius: CornerRadius::same(2),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    weak_bg_fill: BG_2,
                    bg_fill: BG_2,
                    bg_stroke: Stroke::new(1.0, DARK_GRAY),
                    fg_stroke: Stroke::new(1.5, FG_0),
                    corner_radius: CornerRadius::same(3),
                    expansion: 1.0,
                },
                active: WidgetVisuals {
                    weak_bg_fill: BG_2,
                    bg_fill: BG_2,
                    bg_stroke: Stroke::new(1.0, FG_0),
                    fg_stroke: Stroke::new(2.0, FG_0),
                    corner_radius: CornerRadius::same(2),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    weak_bg_fill: BG_1,
                    bg_fill: BG_0,
                    bg_stroke: Stroke::new(1.0, BG_3),
                    fg_stroke: Stroke::new(1.0, FG_2),
                    corner_radius: CornerRadius::same(2),
                    expansion: 0.0,
                },
            },
            selection: Selection {
                bg_fill: BLUE_ALT,
                stroke: Stroke::new(1.0, BG_0_SOFT),
            },
            hyperlink_color: BLUE,
            faint_bg_color: BG_0_SOFT,
            extreme_bg_color: BG_0_HARD,
            code_bg_color: BG_4,
            warn_fg_color: ORANGE,
            error_fg_color: RED,
            window_shadow: Shadow {
                offset: [10, 20],
                blur: 15,
                spread: 0,
                color: BG_1,
            },
            window_fill: BG,
            window_stroke: Stroke::new(1.0, BG_3),
            panel_fill: BG,
            popup_shadow: Shadow {
                offset: [6, 10],
                blur: 8,
                spread: 0,
                color: BG_1,
            },
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, FG_4),
                preview: false,
                blink: true,
                on_duration: 0.5,
                off_duration: 0.5,
            },
            striped: true,
            ..Visuals::$mode()
        }
    }};
}

pub(crate) use gruvbox;
