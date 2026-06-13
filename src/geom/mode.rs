/// Resonance modes for semantic propagation.
/// Defines how meaning resonates through geometric structures.

use crate::geom::field::SemanticField;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResonanceMode {
    Amplify,
    Dampen,
    Balance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResonanceTransform {
    pub mode: ResonanceMode,
    pub magnitude: i64,
}

impl ResonanceTransform {
    pub fn apply(&self, field: &mut SemanticField) {
        let delta = match self.mode {
            ResonanceMode::Amplify => self.magnitude,
            ResonanceMode::Dampen => -self.magnitude,
            ResonanceMode::Balance => 0,
        };

        field.apply_uniform_delta(delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::field::FieldPoint;
    use crate::geom::space::Coordinate3;

    #[test]
    fn amplify_increases_intensity() {
        let mut field = SemanticField::new();
        field.upsert_concept(
            "light",
            FieldPoint {
                position: Coordinate3::new(0, 0, 0),
                intensity: 5,
            },
        );

        ResonanceTransform {
            mode: ResonanceMode::Amplify,
            magnitude: 3,
        }
        .apply(&mut field);

        assert_eq!(field.concept("light").map(|p| p.intensity), Some(8));
    }
}
