/// Geometric space primitives.
/// Defines coordinate systems, transformations, and spatial relationships.

use serde::{Deserialize, Serialize};

/// Fixed-point coordinate for deterministic geometry.
pub type Scalar = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coordinate3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Coordinate3 {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    pub fn manhattan_distance(&self, other: &Self) -> Scalar {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn translated(&self, dx: Scalar, dy: Scalar, dz: Scalar) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Metric {
    Manhattan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricSpace {
    pub metric: Metric,
    pub bounds_min: Coordinate3,
    pub bounds_max: Coordinate3,
}

impl GeometricSpace {
    pub fn new(bounds_min: Coordinate3, bounds_max: Coordinate3) -> Self {
        Self {
            metric: Metric::Manhattan,
            bounds_min,
            bounds_max,
        }
    }

    pub fn contains(&self, point: Coordinate3) -> bool {
        point.x >= self.bounds_min.x
            && point.y >= self.bounds_min.y
            && point.z >= self.bounds_min.z
            && point.x <= self.bounds_max.x
            && point.y <= self.bounds_max.y
            && point.z <= self.bounds_max.z
    }

    pub fn distance(&self, a: Coordinate3, b: Coordinate3) -> Scalar {
        match self.metric {
            Metric::Manhattan => a.manhattan_distance(&b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_distance_is_stable() {
        let space = GeometricSpace::new(Coordinate3::new(0, 0, 0), Coordinate3::new(10, 10, 10));
        let a = Coordinate3::new(2, 3, 5);
        let b = Coordinate3::new(7, 8, 5);
        assert_eq!(space.distance(a, b), 10);
        assert_eq!(space.distance(a, b), 10);
    }

    #[test]
    fn bounds_check_works() {
        let space = GeometricSpace::new(Coordinate3::new(-5, -5, -5), Coordinate3::new(5, 5, 5));
        assert!(space.contains(Coordinate3::new(0, 0, 0)));
        assert!(!space.contains(Coordinate3::new(6, 0, 0)));
    }
}
