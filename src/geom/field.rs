/// Semantic field definitions.
/// Represents the field structure that encodes meaning and relationships.

use crate::geom::space::Coordinate3;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldPoint {
    pub position: Coordinate3,
    pub intensity: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConceptCluster {
    pub anchor: String,
    pub members: Vec<String>,
    pub total_intensity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticField {
    concept_map: BTreeMap<String, FieldPoint>,
}

impl SemanticField {
    pub fn new() -> Self {
        Self {
            concept_map: BTreeMap::new(),
        }
    }

    pub fn upsert_concept(&mut self, concept: impl Into<String>, point: FieldPoint) {
        self.concept_map.insert(concept.into(), point);
    }

    pub fn concept(&self, concept: &str) -> Option<&FieldPoint> {
        self.concept_map.get(concept)
    }

    pub fn concept_count(&self) -> usize {
        self.concept_map.len()
    }

    pub fn ordered_concepts(&self) -> impl Iterator<Item = (&String, &FieldPoint)> {
        self.concept_map.iter()
    }

    pub fn apply_uniform_delta(&mut self, delta: i64) {
        for point in self.concept_map.values_mut() {
            point.intensity += delta;
        }
    }

    pub fn map_intensity<F>(&mut self, mut mapper: F)
    where
        F: FnMut(i64) -> i64,
    {
        for point in self.concept_map.values_mut() {
            point.intensity = mapper(point.intensity);
        }
    }

    pub fn total_energy(&self) -> i64 {
        self.concept_map.values().map(|p| p.intensity.abs()).sum()
    }

    pub fn normalize_energy(&mut self, target_energy: i64) {
        let current = self.total_energy();
        let target = target_energy.max(0);
        if current == 0 || target == current {
            return;
        }

        for point in self.concept_map.values_mut() {
            let num = point.intensity * target;
            let half = current / 2;
            point.intensity = if num >= 0 {
                (num + half) / current
            } else {
                (num - half) / current
            };
        }
    }

    pub fn compress_by_intensity(&mut self, min_abs_intensity: i64) {
        let threshold = min_abs_intensity.max(0);
        self.concept_map.retain(|_, point| point.intensity.abs() >= threshold);
    }

    pub fn canonical_concepts(&self) -> Vec<String> {
        self.concept_map.keys().cloned().collect()
    }

    pub fn canonical_snapshot(&self) -> Vec<(String, i64, i64, i64, i64)> {
        self
            .ordered_concepts()
            .map(|(concept, point)| {
                (
                    concept.clone(),
                    point.intensity,
                    point.position.x,
                    point.position.y,
                    point.position.z,
                )
            })
            .collect()
    }

    pub fn canonical_hash(&self) -> Result<String, serde_json::Error> {
        let bytes = serde_json::to_vec(&self.canonical_snapshot())?;
        let mut h = Sha256::new();
        h.update(bytes);
        Ok(format!("{:x}", h.finalize()))
    }

    pub fn merge_from(&mut self, other: &SemanticField) {
        for (concept, point) in other.ordered_concepts() {
            if let Some(existing) = self.concept_map.get_mut(concept) {
                existing.intensity += point.intensity;
            } else {
                self.concept_map.insert(concept.clone(), point.clone());
            }
        }
    }

    pub fn clusters_by_subject(&self) -> Vec<ConceptCluster> {
        let mut grouped: BTreeMap<String, Vec<(String, i64)>> = BTreeMap::new();
        for (concept, point) in self.ordered_concepts() {
            let anchor = concept
                .split_once(':')
                .map(|(head, _)| head.to_string())
                .unwrap_or_else(|| concept.clone());
            grouped
                .entry(anchor)
                .or_default()
                .push((concept.clone(), point.intensity));
        }

        grouped
            .into_iter()
            .map(|(anchor, mut members)| {
                members.sort_by(|a, b| a.0.cmp(&b.0));
                let total_intensity = members.iter().map(|(_, intensity)| *intensity).sum();
                ConceptCluster {
                    anchor,
                    members: members.into_iter().map(|(concept, _)| concept).collect(),
                    total_intensity,
                }
            })
            .collect()
    }
}

impl Default for SemanticField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_has_deterministic_order() {
        let mut field = SemanticField::new();
        field.upsert_concept(
            "zeta",
            FieldPoint {
                position: Coordinate3::new(0, 0, 0),
                intensity: 1,
            },
        );
        field.upsert_concept(
            "alpha",
            FieldPoint {
                position: Coordinate3::new(1, 0, 0),
                intensity: 2,
            },
        );

        let concepts: Vec<&str> = field.ordered_concepts().map(|(k, _)| k.as_str()).collect();
        assert_eq!(concepts, vec!["alpha", "zeta"]);
    }

    #[test]
    fn normalization_and_compression_are_deterministic() {
        let mut field = SemanticField::new();
        field.upsert_concept(
            "light:wave",
            FieldPoint {
                position: Coordinate3::new(0, 0, 0),
                intensity: 100,
            },
        );
        field.upsert_concept(
            "light:particle",
            FieldPoint {
                position: Coordinate3::new(1, 0, 0),
                intensity: 20,
            },
        );
        field.normalize_energy(60);
        field.compress_by_intensity(9);

        let concepts = field.canonical_concepts();
        assert_eq!(concepts, vec!["light:particle", "light:wave"]);
        assert_eq!(field.total_energy(), 60);
    }

    #[test]
    fn subject_clusters_are_stable() {
        let mut field = SemanticField::new();
        field.upsert_concept(
            "light:wave",
            FieldPoint {
                position: Coordinate3::new(0, 0, 0),
                intensity: 10,
            },
        );
        field.upsert_concept(
            "light:particle",
            FieldPoint {
                position: Coordinate3::new(0, 1, 0),
                intensity: 8,
            },
        );
        field.upsert_concept(
            "vacuum:medium",
            FieldPoint {
                position: Coordinate3::new(2, 0, 0),
                intensity: -7,
            },
        );

        let clusters = field.clusters_by_subject();
        assert_eq!(clusters.len(), 2);
        assert_eq!(clusters[0].anchor, "light");
        assert_eq!(clusters[0].members, vec!["light:particle", "light:wave"]);
        assert_eq!(clusters[0].total_intensity, 18);
    }

    #[test]
    fn canonical_hash_is_stable_for_equivalent_fields() {
        let mut a = SemanticField::new();
        let mut b = SemanticField::new();

        a.upsert_concept(
            "light:wave",
            FieldPoint {
                position: Coordinate3::new(0, 0, 0),
                intensity: 12,
            },
        );
        a.upsert_concept(
            "light:particle",
            FieldPoint {
                position: Coordinate3::new(1, 0, 0),
                intensity: 10,
            },
        );

        b.upsert_concept(
            "light:particle",
            FieldPoint {
                position: Coordinate3::new(1, 0, 0),
                intensity: 10,
            },
        );
        b.upsert_concept(
            "light:wave",
            FieldPoint {
                position: Coordinate3::new(0, 0, 0),
                intensity: 12,
            },
        );

        let ha = a.canonical_hash().unwrap_or_default();
        let hb = b.canonical_hash().unwrap_or_default();
        assert_eq!(ha, hb);
    }
}
