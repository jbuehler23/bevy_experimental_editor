use crate::stdb::DbVector2;
use bevy::prelude::*;

pub const LERP_DURATION: f32 = 0.1;

// Color palettes
pub const CIRCLE_COLORS: &[Color] = &[
    // Yellow
    Color::srgb(175.0 / 255.0, 159.0 / 255.0, 49.0 / 255.0),
    Color::srgb(175.0 / 255.0, 116.0 / 255.0, 49.0 / 255.0),
    // Purple
    Color::srgb(112.0 / 255.0, 47.0 / 255.0, 252.0 / 255.0),
    Color::srgb(51.0 / 255.0, 91.0 / 255.0, 252.0 / 255.0),
    // Red
    Color::srgb(176.0 / 255.0, 54.0 / 255.0, 54.0 / 255.0),
    Color::srgb(176.0 / 255.0, 109.0 / 255.0, 54.0 / 255.0),
    Color::srgb(141.0 / 255.0, 43.0 / 255.0, 99.0 / 255.0),
    // Blue
    Color::srgb(2.0 / 255.0, 188.0 / 255.0, 250.0 / 255.0),
    Color::srgb(7.0 / 255.0, 50.0 / 255.0, 251.0 / 255.0),
    Color::srgb(2.0 / 255.0, 28.0 / 255.0, 146.0 / 255.0),
];

pub const FOOD_COLORS: &[Color] = &[
    Color::srgb(119.0 / 255.0, 252.0 / 255.0, 173.0 / 255.0),
    Color::srgb(76.0 / 255.0, 250.0 / 255.0, 146.0 / 255.0),
    Color::srgb(35.0 / 255.0, 246.0 / 255.0, 120.0 / 255.0),
    Color::srgb(119.0 / 255.0, 251.0 / 255.0, 201.0 / 255.0),
    Color::srgb(76.0 / 255.0, 249.0 / 255.0, 184.0 / 255.0),
    Color::srgb(35.0 / 255.0, 245.0 / 255.0, 165.0 / 255.0),
];

// Conversion functions
impl From<DbVector2> for Vec2 {
    fn from(v: DbVector2) -> Self {
        Vec2::new(v.x, v.y)
    }
}

impl From<Vec2> for DbVector2 {
    fn from(v: Vec2) -> Self {
        DbVector2 { x: v.x, y: v.y }
    }
}

// Mass calculations
pub fn mass_to_radius(mass: u32) -> f32 {
    (mass as f32).sqrt()
}

pub fn mass_to_diameter(mass: u32) -> f32 {
    mass_to_radius(mass) * 2.0
}
