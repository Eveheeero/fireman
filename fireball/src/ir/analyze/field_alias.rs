//! Field-sensitive alias projection built on top of aggregate recovery.
//!
//! Combines recovered base+offset aggregate accesses with points-to targets so
//! downstream consumers can distinguish `base+4` from `base+8`, and can also
//! separate fields by synthetic heap allocation site when points-to has that
//! information.

use crate::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldProjectionKey {
    pub base: super::points_to::AbstractLocation,
    pub offset: i64,
}

#[derive(Debug, Clone)]
pub struct FieldProjection {
    pub key: FieldProjectionKey,
    pub access_count: usize,
    pub is_read: bool,
    pub is_write: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FieldAliasAnalysis {
    projections: Vec<FieldProjection>,
}

impl FieldAliasAnalysis {
    pub fn projections(&self) -> &[FieldProjection] {
        &self.projections
    }

    pub fn projection_count(&self) -> usize {
        self.projections.len()
    }

    pub fn distinct_base_count(&self) -> usize {
        self.projections
            .iter()
            .map(|projection| projection.key.base)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn heap_backed_projection_count(&self) -> usize {
        self.projections
            .iter()
            .filter(|projection| {
                matches!(
                    projection.key.base,
                    super::points_to::AbstractLocation::Heap(_)
                )
            })
            .count()
    }
}

pub fn analyze_field_alias(
    points_to: &super::points_to::PointsToSet,
    aggregates: &[super::struct_recovery::AggregateCandidate],
) -> FieldAliasAnalysis {
    let mut points_to = points_to.clone();
    let mut merged: HashMap<FieldProjectionKey, (usize, bool, bool)> = HashMap::new();

    for aggregate in aggregates {
        let mut bases = points_to.targets_of(&super::points_to::AbstractLocation::Register(
            aggregate.base,
        ));
        if bases.is_empty() {
            bases.insert(super::points_to::AbstractLocation::Register(aggregate.base));
        }

        for base in bases {
            for field in &aggregate.fields {
                let entry = merged
                    .entry(FieldProjectionKey {
                        base,
                        offset: field.offset,
                    })
                    .or_insert((0, false, false));
                entry.0 += field.access_count;
                entry.1 |= field.is_read;
                entry.2 |= field.is_write;
            }
        }
    }

    let projections = merged
        .into_iter()
        .map(|(key, (access_count, is_read, is_write))| FieldProjection {
            key,
            access_count,
            is_read,
            is_write,
        })
        .collect();

    FieldAliasAnalysis { projections }
}

pub fn log_field_alias_analysis(analysis: &FieldAliasAnalysis) {
    if analysis.projection_count() > 0 {
        debug!(
            "Field-sensitive alias analysis: {} field projections across {} bases ({} heap-backed)",
            analysis.projection_count(),
            analysis.distinct_base_count(),
            analysis.heap_backed_projection_count(),
        );
    }
}
