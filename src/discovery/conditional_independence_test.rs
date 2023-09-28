use crate::models::ConditionalIndependence;

pub trait ConditionalIndependenceTest {
    type LabelsIter<'a>: Iterator<Item = &'a str>
    where
        Self: 'a;

    fn labels(&self) -> Self::LabelsIter<'_>;

    fn eval(&self, x: usize, y: usize, z: &[usize]) -> (usize, f64, f64);

    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool;

    fn with_significance_level(self, alpha: f64) -> Self;
}

impl<T> ConditionalIndependence for T
where
    T: ConditionalIndependenceTest,
{
    type LabelsIter<'a> = T::LabelsIter<'a> where T: 'a, Self: 'a;

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        ConditionalIndependenceTest::labels(self)
    }

    #[inline]
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool {
        ConditionalIndependenceTest::call(self, x, y, z)
    }
}
