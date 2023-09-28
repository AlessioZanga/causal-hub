use ndarray::prelude::*;

#[inline]
pub fn axis_chunks_size<A>(a: &Array2<A>) -> usize {
    usize::max(page_size::get() / a.ncols(), a.ncols())
}
