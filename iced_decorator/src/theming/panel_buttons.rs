use iced::{
    color,
    widget::{button, svg},
    Color,
};

pub struct Theme {
    pub svg: Color,

    pub close: Color,
    pub close_active: Color,
    pub close_hover: Color,

    pub restore: Color,
    pub restore_active: Color,
    pub restore_hover: Color,

    pub minimize: Color,
    pub minimize_active: Color,
    pub minimize_hover: Color,
}

impl Theme {
    pub const LIGHT: Self = Self {
        svg: Color::BLACK,

        close: Color::TRANSPARENT,
        close_active: color!(255, 0, 0, 0.8),
        close_hover: color!(255, 0, 0, 0.5),

        restore: Color::TRANSPARENT,
        restore_hover: color!(0, 0, 0, 0.3),
        restore_active: color!(0, 0, 0, 0.5),

        minimize: Color::TRANSPARENT,
        minimize_hover: color!(0, 0, 0, 0.3),
        minimize_active: color!(0, 0, 0, 0.5),
    };

    pub const DARK: Self = Self {
        svg: Color::WHITE,

        close: Color::TRANSPARENT,
        close_active: color!(255, 0, 0, 0.8),
        close_hover: color!(255, 0, 0, 0.5),

        restore: Color::TRANSPARENT,
        restore_hover: color!(0, 0, 0, 0.3),
        restore_active: color!(0, 0, 0, 0.5),

        minimize: Color::TRANSPARENT,
        minimize_hover: color!(0, 0, 0, 0.3),
        minimize_active: color!(0, 0, 0, 0.5),
    };
}

impl Default for Theme {
    fn default() -> Self {
        Self::LIGHT
    }
}

/**
 * Svg
 */
#[derive(Default)]
pub enum Svg {
    /// No filtering to the rendered SVG.
    #[default]
    Default,
}

impl svg::StyleSheet for Theme {
    type Style = Svg;

    fn appearance(&self, style: &Self::Style) -> svg::Appearance {
        match style {
            Svg::Default => svg::Appearance {
                color: Some(self.svg),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Button {
    #[default]
    Restore,
    Minimize,
    Close,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Button) -> button::Appearance {
        match style {
            Button::Close => button::Appearance {
                background: Some(self.close.into()),
                ..button::Appearance::default()
            },
            Button::Restore => button::Appearance {
                background: Some(self.restore.into()),
                ..button::Appearance::default()
            },
            Button::Minimize => button::Appearance {
                background: Some(self.minimize.into()),
                ..button::Appearance::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        match style {
            Button::Close => button::Appearance {
                background: Some(self.close_hover.into()),
                ..active
            },
            Button::Restore => button::Appearance {
                background: Some(self.restore_hover.into()),
                ..active
            },
            Button::Minimize => button::Appearance {
                background: Some(self.minimize_hover.into()),
                ..active
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        match style {
            Button::Close => button::Appearance {
                background: Some(self.close_active.into()),
                ..active
            },
            Button::Restore => button::Appearance {
                background: Some(self.restore_active.into()),
                ..active
            },
            Button::Minimize => button::Appearance {
                background: Some(self.minimize_active.into()),
                ..active
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}
