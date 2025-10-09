#[cfg(feature = "bevy")]
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use spacetimedb::SpacetimeType;

/// 2D Vector type that can be used both in Bevy and SpacetimeDB
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SpacetimeType))]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            Self::zero()
        }
    }

    pub fn distance_to(&self, other: &Vector2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(feature = "bevy")]
impl From<Vec2> for Vector2 {
    fn from(v: Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

#[cfg(feature = "bevy")]
impl From<Vector2> for Vec2 {
    fn from(v: Vector2) -> Self {
        Vec2::new(v.x, v.y)
    }
}

#[cfg(feature = "bevy")]
impl From<Vec3> for Vector2 {
    fn from(v: Vec3) -> Self {
        Self { x: v.x, y: v.y }
    }
}

#[cfg(feature = "bevy")]
impl From<Vector2> for Vec3 {
    fn from(v: Vector2) -> Self {
        Vec3::new(v.x, v.y, 0.0)
    }
}

impl std::ops::Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
