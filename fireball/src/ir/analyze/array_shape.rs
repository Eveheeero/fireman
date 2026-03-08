use super::struct_recovery::AggregateCandidate;
use crate::{ir::Register, prelude::*};

/// Conservative array-shape hint derived from constant-stride aggregate access patterns.
#[derive(Debug, Clone)]
pub struct ArrayShapeCandidate {
    pub base: Register,
    pub stride: i64,
    pub min_offset: i64,
    pub max_offset: i64,
    pub observed_elements: usize,
    pub contiguous: bool,
    pub dimensions: u8,
}

pub fn analyze_array_shapes(aggregates: &[AggregateCandidate]) -> Vec<ArrayShapeCandidate> {
    aggregates
        .iter()
        .filter_map(|candidate| {
            if !candidate.likely_array {
                return None;
            }

            let stride = candidate.stride?;
            if stride <= 0 || candidate.fields.is_empty() {
                return None;
            }

            let min_offset = candidate.fields.first()?.offset;
            let max_offset = candidate.fields.last()?.offset;
            let contiguous = candidate
                .fields
                .windows(2)
                .all(|window| window[1].offset - window[0].offset == stride);

            Some(ArrayShapeCandidate {
                base: candidate.base.clone(),
                stride,
                min_offset,
                max_offset,
                observed_elements: candidate.fields.len(),
                contiguous,
                dimensions: 1,
            })
        })
        .collect()
}

pub fn log_array_shape_analysis(shapes: &[ArrayShapeCandidate]) {
    if shapes.is_empty() {
        return;
    }

    let contiguous_shapes = shapes.iter().filter(|shape| shape.contiguous).count();
    debug!(
        "Array shape inference: {} candidates ({} contiguous, max_dimensions={})",
        shapes.len(),
        contiguous_shapes,
        shapes
            .iter()
            .map(|shape| shape.dimensions)
            .max()
            .unwrap_or(0),
    );
}
