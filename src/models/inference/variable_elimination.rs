use crate::models::{
    BayesianNetwork,
    DiscreteFactor as DiscreteJPD, /* FIXME: Implement a separate class for DiscreteJPD. */
    Factor,
};

#[derive(Clone, Debug)]
pub struct VariableElimination<'m, M> {
    m: &'m M,
}

impl<'m, M> VariableElimination<'m, M> {
    fn sum_product<'a, P, Z>(p: P, z: Z) -> P::Item
    where
        P: IntoIterator,
        P::Item: Factor,
        Z: IntoIterator<Item = &'a str>,
    {
        todo!() // FIXME:
    }

    fn variable_elimination<'a, P>(p: P, z: &'a str) -> P
    where
        P: IntoIterator,
        P::Item: Factor,
    {
        todo!() // FIXME:
    }
}
