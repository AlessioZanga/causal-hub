pub trait ConditionalIndependenceTest {
    type LabelsIter<'a>: Iterator<Item = &'a str>
    where
        Self: 'a;

    fn labels_iter(&self) -> Self::LabelsIter<'_>;

    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool;
}

pub trait GeneralizedConditionalIndependenceTest {
    type LabelsIter<'a>: Iterator<Item = &'a str>
    where
        Self: 'a;

    fn labels_iter(&self) -> Self::LabelsIter<'_>;

    fn call<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>;
}
