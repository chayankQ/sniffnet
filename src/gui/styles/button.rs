//! Buttons style

#![allow(clippy::module_name_repetitions)]

use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Background, Color, Vector};

use crate::gui::styles::style_constants::{BORDER_BUTTON_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::gradient_type::{
    get_gradient_buttons, get_gradient_hovered_buttons, GradientType,
};
use crate::gui::styles::types::palette::mix_colors;
use crate::{StyleType};

#[derive(Clone, Copy, Default)]
pub enum ButtonType {
    #[default]
    Standard,
    BorderedRound,
    BorderedRoundSelected,
    TabActive,
    TabInactive,
    Starred,
    NotStarred,
    Neutral,
    Alert,
    Badge,
    Gradient(GradientType),
}

impl button::StyleSheet for StyleType {
    type Style = ButtonType;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let colors = get_colors(*self);
        let color_buttons = self.get_buttons_color();
        button::Appearance {
            background: Some(match style {
                ButtonType::TabActive | ButtonType::BorderedRoundSelected => {
                    Background::Color(mix_colors(colors.primary, color_buttons))
                }
                ButtonType::Starred => Background::Color(colors.starred),
                ButtonType::BorderedRound => Background::Color(Color {
                    a: self.get_alpha_round_containers(),
                    ..color_buttons
                }),
                ButtonType::Neutral | ButtonType::NotStarred => {
                    Background::Color(Color::TRANSPARENT)
                }
                ButtonType::Gradient(GradientType::None) | ButtonType::Badge => {
                    Background::Color(colors.secondary)
                }
                ButtonType::Gradient(gradient_type) => Background::Gradient(get_gradient_buttons(
                    &colors,
                    *gradient_type,
                    self.is_nightly(),
                    1.0,
                )),
                _ => Background::Color(color_buttons),
            }),
            border_radius: match style {
                ButtonType::Neutral => 0.0.into(),
                ButtonType::TabActive | ButtonType::TabInactive => [0.0, 0.0, 30.0, 30.0].into(),
                ButtonType::BorderedRound | ButtonType::BorderedRoundSelected => 12.0.into(),
                ButtonType::Starred | ButtonType::NotStarred => 100.0.into(),
                _ => BORDER_BUTTON_RADIUS.into(),
            },
            border_width: match style {
                ButtonType::TabActive
                | ButtonType::TabInactive
                | ButtonType::Starred
                | ButtonType::NotStarred
                | ButtonType::Neutral
                | ButtonType::Badge => 0.0,
                ButtonType::BorderedRound => BORDER_WIDTH * 2.0,
                _ => BORDER_WIDTH,
            },
            shadow_offset: match style {
                ButtonType::TabActive | ButtonType::TabInactive => Vector::new(3.0, 2.0),
                _ => Vector::new(0.0, 0.0),
            },
            text_color: match style {
                ButtonType::Starred => Color::BLACK,
                ButtonType::Badge | ButtonType::Gradient(_) => colors.text_headers,
                _ => colors.text_body,
            },
            border_color: match style {
                ButtonType::Alert => Color::new(0.8, 0.15, 0.15, 1.0),
                ButtonType::BorderedRound => Color {
                    a: self.get_alpha_round_borders(),
                    ..color_buttons
                },
                _ => colors.secondary,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let colors = get_colors(*self);
        let color_buttons = self.get_buttons_color();
        button::Appearance {
            shadow_offset: match style {
                ButtonType::Neutral => Vector::default(),
                ButtonType::TabActive | ButtonType::TabInactive => Vector::new(3.0, 3.0),
                _ => Vector::new(0.0, 2.0),
            },
            background: Some(match style {
                ButtonType::Starred => Background::Color(colors.starred),
                ButtonType::Neutral => Background::Color(Color{a: get_alpha_round_borders(*self), ..color_buttons}),
                ButtonType::Gradient(GradientType::None) => {
                    Background::Color(mix_colors(colors.primary, colors.secondary))
                }
                ButtonType::Gradient(gradient_type) => Background::Gradient(
                    get_gradient_hovered_buttons(&colors, *gradient_type, self.is_nightly()),
                ),
                _ => Background::Color(mix_colors(colors.primary, color_buttons)),
            }),
            border_radius: match style {
                ButtonType::Neutral => 0.0.into(),
                ButtonType::TabActive | ButtonType::TabInactive => [0.0, 0.0, 30.0, 30.0].into(),
                ButtonType::BorderedRound | ButtonType::BorderedRoundSelected => 12.0.into(),
                ButtonType::Starred | ButtonType::NotStarred => 100.0.into(),
                _ => BORDER_BUTTON_RADIUS.into(),
            },
            border_width: match style {
                ButtonType::Starred
                | ButtonType::TabActive
                | ButtonType::TabInactive
                | ButtonType::BorderedRound => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: match style {
                ButtonType::Alert => Color::new(0.8, 0.15, 0.15, 1.0),
                ButtonType::BorderedRound | ButtonType::NotStarred => Color {
                    a: get_alpha_round_borders(*self),
                    ..color_buttons
                },
                ButtonType::Neutral => color_buttons,
                _ => colors.secondary,
            },
            text_color: match style {
                ButtonType::Starred => Color::BLACK,
                ButtonType::Gradient(_) => colors.text_headers,
                _ => colors.text_body,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        match style {
            ButtonType::Gradient(_) => {
                let colors = get_colors(*self);
                let color_buttons = get_buttons_color(*self);
                button::Appearance {
                    background: Some(match style {
                        ButtonType::Gradient(GradientType::None) => Background::Color(Color {
                            a: get_alpha_round_containers(*self),
                            ..colors.secondary
                        }),
                        ButtonType::Gradient(gradient_type) => {
                            Background::Gradient(get_gradient_buttons(
                                &colors,
                                *gradient_type,
                                self.is_nightly(),
                                get_alpha_round_containers(*self),
                            ))
                        }
                        _ => Background::Color(color_buttons),
                    }),
                    border_radius: BORDER_BUTTON_RADIUS.into(),
                    border_width: BORDER_WIDTH,
                    shadow_offset: Vector::new(0.0, 0.0),
                    text_color: colors.text_headers,
                    border_color: Color {
                        a: get_alpha_round_borders(*self),
                        ..colors.secondary
                    },
                }
            }
            _ => button::StyleSheet::active(self, style),
        }
    }
}
