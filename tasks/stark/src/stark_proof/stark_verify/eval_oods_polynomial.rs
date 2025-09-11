use crate::stark_proof::stark_commit::eval_oods_polynomial_inner::EvalOodsPolynomialInner;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

// EvalOodsPolynomial task - evaluates OODS polynomial for a single point
#[derive(Debug, Clone)]
#[repr(C)]
pub struct EvalOodsPolynomial {
    processed: bool,
}

impl_type_identifiable!(EvalOodsPolynomial);

impl EvalOodsPolynomial {
    pub fn new() -> Self {
        Self { processed: false }
    }
}

impl Default for EvalOodsPolynomial {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for EvalOodsPolynomial {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        _stack: &mut T,
    ) -> Vec<Vec<u8>> {
        self.processed = true;
        vec![EvalOodsPolynomialInner::new().to_vec_with_type_tag()]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}
