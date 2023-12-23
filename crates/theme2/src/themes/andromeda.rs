// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, PlayerColor, PlayerColors, StatusColorsRefinement, ThemeColorsRefinement,
    UserFontStyle, UserFontWeight, UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily,
    UserThemeStylesRefinement,
};

pub fn andromeda() -> UserThemeFamily {
    UserThemeFamily {
        name: "Andromeda".into(),
        author: "Zed Industries".into(),
        themes: vec![UserTheme {
            name: "Andromeda".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    border: Some(rgba(0x252931ff).into()),
                    border_variant: Some(rgba(0x21232aff).into()),
                    elevated_surface_background: Some(rgba(0x262a33ff).into()),
                    background: Some(rgba(0x262a33ff).into()),
                    panel_background: Some(rgba(0x21242bff).into()),
                    element_hover: Some(rgba(0x2b2f3980).into()),
                    element_selected: Some(rgba(0x383b4580).into()),
                    text: Some(rgba(0xf7f7f8ff).into()),
                    text_muted: Some(rgba(0xaca8aeff).into()),
                    text_placeholder: Some(rgba(0x474a53ff).into()),
                    text_disabled: Some(rgba(0xf7f7f8ff).into()),
                    text_accent: Some(rgba(0x11a793ff).into()),
                    status_bar_background: Some(rgba(0x262a33ff).into()),
                    title_bar_background: Some(rgba(0x262a33ff).into()),
                    toolbar_background: Some(rgba(0x1e2025ff).into()),
                    tab_bar_background: Some(rgba(0x21242bff).into()),
                    tab_inactive_background: Some(rgba(0x21242bff).into()),
                    tab_active_background: Some(rgba(0x1e2025ff).into()),
                    scrollbar_thumb_background: Some(rgba(0xf7f7f84d).into()),
                    scrollbar_thumb_hover_background: Some(rgba(0xf7f7f84d).into()),
                    scrollbar_thumb_border: Some(rgba(0x21232aff).into()),
                    scrollbar_track_border: Some(rgba(0x21232aff).into()),
                    editor_foreground: Some(rgba(0xf7f7f8ff).into()),
                    editor_background: Some(rgba(0x1e2025ff).into()),
                    editor_gutter_background: Some(rgba(0x1e2025ff).into()),
                    editor_line_number: Some(rgba(0xf7f7f859).into()),
                    editor_active_line_number: Some(rgba(0xf7f7f8ff).into()),
                    editor_wrap_guide: Some(rgba(0xf7f7f80d).into()),
                    editor_active_wrap_guide: Some(rgba(0xf7f7f81a).into()),
                    terminal_background: Some(rgba(0x1e2025ff).into()),
                    terminal_ansi_bright_black: Some(rgba(0x40434cff).into()),
                    terminal_ansi_bright_red: Some(rgba(0x8e103aff).into()),
                    terminal_ansi_bright_green: Some(rgba(0x457c38ff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0x958435ff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x1b5148ff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0x682781ff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x018169ff).into()),
                    terminal_ansi_bright_white: Some(rgba(0xf7f7f8ff).into()),
                    terminal_ansi_black: Some(rgba(0x1e2025ff).into()),
                    terminal_ansi_red: Some(rgba(0xf82872ff).into()),
                    terminal_ansi_green: Some(rgba(0x96df72ff).into()),
                    terminal_ansi_yellow: Some(rgba(0xfee56dff).into()),
                    terminal_ansi_blue: Some(rgba(0x11a793ff).into()),
                    terminal_ansi_magenta: Some(rgba(0xc74decff).into()),
                    terminal_ansi_cyan: Some(rgba(0x09e7c6ff).into()),
                    terminal_ansi_white: Some(rgba(0xf7f7f8ff).into()),
                    ..Default::default()
                },
                status: StatusColorsRefinement {
                    created: Some(rgba(0x96df72ff).into()),
                    deleted: Some(rgba(0xcd1d5aff).into()),
                    error: Some(rgba(0xf82872ff).into()),
                    hint: Some(rgba(0x618399ff).into()),
                    modified: Some(rgba(0xfee56dff).into()),
                    success: Some(rgba(0xf7f7f8ff).into()),
                    warning: Some(rgba(0xfee56dff).into()),
                    ..Default::default()
                },
                player: Some(PlayerColors(vec![
                    PlayerColor {
                        cursor: rgba(0x11a793ff).into(),
                        background: rgba(0x11a793ff).into(),
                        selection: rgba(0x11a7933d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xc74decff).into(),
                        background: rgba(0xc74decff).into(),
                        selection: rgba(0xc74dec3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xf29c14ff).into(),
                        background: rgba(0xf29c14ff).into(),
                        selection: rgba(0xf29c143d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x8a3fa6ff).into(),
                        background: rgba(0x8a3fa6ff).into(),
                        selection: rgba(0x8a3fa63d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x09e7c6ff).into(),
                        background: rgba(0x09e7c6ff).into(),
                        selection: rgba(0x09e7c63d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xf82872ff).into(),
                        background: rgba(0xf82872ff).into(),
                        selection: rgba(0xf828723d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xfee56dff).into(),
                        background: rgba(0xfee56dff).into(),
                        selection: rgba(0xfee56d3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x96df72ff).into(),
                        background: rgba(0x96df72ff).into(),
                        selection: rgba(0x96df723d).into(),
                    },
                ])),
                syntax: Some(UserSyntaxTheme {
                    highlights: vec![
                        (
                            "attribute".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "boolean".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x96df72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xafabb1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment.doc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xafabb1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x96df72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constructor".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "embedded".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis.strong".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "enum".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "function".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfee56dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "hint".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x618399ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "keyword".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "label".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_text".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_uri".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x96df72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "number".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x96df72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "operator".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "predictive".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x315f70ff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "preproc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "primary".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "property".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.bracket".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.delimiter".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.list_marker".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.escape".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xafabb1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.regex".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special.symbol".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "tag".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "text.literal".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "title".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "type".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x09e7c6ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variable".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                    ],
                }),
            },
        }],
    }
}
