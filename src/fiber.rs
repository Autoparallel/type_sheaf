use algebra::{
    ops::Op,
    structure::{Group, GroupAbelian},
};
use algebra::ops::Additive;
use num::Integer;

use num::complex::Complex;

struct GroupFiber<G, O>
where
    G: Group<O>,
    O: Op,
{
    elements: G,
    op: O,
}

impl<G, O> GroupFiber<G, O>
where
    G: Group<O>,
    O: Op,
{
    fn new(elements: G, op: O) -> Self {
        Self { elements, op }
    }
}

struct GroupAbelianFiber<G, O>
where
    G: GroupAbelian<O>,
    O: Op,
{
    elements: G,
    op: O,
}

impl<G, O> GroupAbelianFiber<G, O>
where
    G: GroupAbelian<O>,
    O: Op,
{
    fn new(elements: G, op: O) -> Self {
        Self { elements, op }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn complex_group() {
        let re1= num::BigRational::new(1.into(), 2.into());
        let im1 = num::BigRational::new(2.into(), 1.into());
        let z1 = Complex::new(re1, im1);
        println!("z1: {:#?}", z1);

        let re2= num::BigRational::new(3.into(), 2.into());
        let im2 = num::BigRational::new(5.into(), 1.into());
        let z2 = Complex::new(re2, im2);
        println!("z2: {:#?}", z2);

        let z3 = z1.clone() + z2.clone();
        println!("z3: {:#?}", z3);

        let z4 = z1 * z2;
        println!("z4: {:#?}", z4);
    }
}
