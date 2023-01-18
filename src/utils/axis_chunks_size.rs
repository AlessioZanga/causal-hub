use ndarray::prelude::*;

/// Compute the optimal axis chunks size.
#[inline]
pub fn axis_chunks_size<A>(a: &Array2<A>) -> usize {
    usize::max(page_size::get() / a.ncols(), a.ncols())
}
