pub mod config;

use crate::swiftness::commitment::table;
use crate::swiftness::stark::types::{cast_slice_to_struct, cast_struct_to_slice};
use felt::Felt;
use utils::{BidirectionalStack, StarkVerifyTrait};
use crate::swiftness::commitment::vector::types::CommitmentTrait;
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct UnsentCommitment {
    pub original: Felt,
    pub interaction: Felt,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Decommitment {
    // Responses for queries to the original trace.
    pub original: table::types::Decommitment,
    // Responses for queries to the interaction trace.
    pub interaction: table::types::Decommitment,
}

impl CommitmentTrait<Decommitment, ()> for Decommitment {
    fn from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) {
        // Read original table decommitment
        table::types::Decommitment::from_stack(stack);
        
        // Read interaction table decommitment
        table::types::Decommitment::from_stack(stack);
    }

    fn from_stack_ref<T: BidirectionalStack + StarkVerifyTrait>(_stack: &T) -> &Self {
        // For Decommitment, data is stored in VerifyVariables, use from_stack instead
        unimplemented!("Decommitment data is stored in VerifyVariables, use from_stack instead")
    }

    fn push_to_stack<T: BidirectionalStack + StarkVerifyTrait>(&mut self, stack: &mut T) {
        // Push interaction table decommitment first (will be popped last)
        self.interaction.push_to_stack(stack);
        
        // Push original table decommitment second (will be popped first)
        self.original.push_to_stack(stack);
    }

    fn to_bytes_be(&self) -> Decommitment {
        *self
    }
}

// A witness for a decommitment of the AIR traces over queries.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Witness {
    pub original: table::types::Witness,
    pub interaction: table::types::Witness,
}

impl CommitmentTrait<Witness, ()> for Witness {
    fn from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) {
        // Read original table witness
        table::types::Witness::from_stack(stack);
        
        // Read interaction table witness
        table::types::Witness::from_stack(stack);
    }

    fn from_stack_ref<T: BidirectionalStack + StarkVerifyTrait>(_stack: &T) -> &Self {
        // For Witness, data is stored in VerifyVariables, use from_stack instead
        unimplemented!("Witness data is stored in VerifyVariables, use from_stack instead")
    }

    fn push_to_stack<T: BidirectionalStack + StarkVerifyTrait>(&mut self, stack: &mut T) {
        // Push interaction table witness first (will be popped last)
        self.interaction.push_to_stack(stack);
        
        // Push original table witness second (will be popped first)
        self.original.push_to_stack(stack);
    }

    fn to_bytes_be(&self) -> Witness {
        *self
    }
}

// Commitment for the Traces component.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Commitment<InteractionElements> {
    // Commitment to the first trace.
    pub original: table::types::Commitment,
    // The interaction elements that were sent to the prover after the first trace commitment (e.g.
    // memory interaction).
    pub interaction_elements: InteractionElements,
    // Commitment to the second (interaction) trace.
    pub interaction: table::types::Commitment,
}

// Bytes representation for stack operations
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TraceCommitmentBytes {
    pub original: table::types::TableCommitmentBytes,
    pub interaction: table::types::TableCommitmentBytes,
}

// Implement CommitmentTrait for Trace::Commitment
impl<InteractionElements> CommitmentTrait<TraceCommitmentBytes> for Commitment<InteractionElements>
where
    InteractionElements: Clone + Default,
{
    fn from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) -> Self {
        let data = stack.borrow_front();
        let commitment_ref = cast_slice_to_struct::<Self>(data);
        let commitment = commitment_ref.clone(); // Clone the reference
        stack.pop_front();
        commitment
    }

    fn from_stack_ref<T: BidirectionalStack + StarkVerifyTrait>(stack: &T) -> &Self {
        let data = stack.borrow_front();
        cast_slice_to_struct::<Self>(data)
    }

    fn push_to_stack<T: BidirectionalStack + StarkVerifyTrait>(&mut self, stack: &mut T) {
        let commitment_bytes = cast_struct_to_slice(self);
        stack.push_front(commitment_bytes).unwrap();
    }

    fn to_bytes_be(&self) -> TraceCommitmentBytes {
        TraceCommitmentBytes {
            original: self.original.to_bytes_be(),
            interaction: self.interaction.to_bytes_be(),
        }
    }
}

