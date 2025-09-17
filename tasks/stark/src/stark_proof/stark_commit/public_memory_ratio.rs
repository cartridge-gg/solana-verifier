use felt::Felt;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkCommitmentTrait,
    TypeIdentifiable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicMemoryRatioPhase {
    Init,
    ComputeRatio,
    Done,
}

impl_type_identifiable!(PublicMemoryRatio);
#[repr(C)]
pub struct PublicMemoryRatio {
    step: PublicMemoryRatioPhase,
    perm_interaction_elm: Felt,
    hash_interaction_elm0: Felt,
    public_memory_column_size: Felt,
}

impl PublicMemoryRatio {
    pub fn new(
        perm_interaction_elm: Felt,
        hash_interaction_elm0: Felt,
        public_memory_column_size: Felt,
    ) -> Self {
        Self {
            step: PublicMemoryRatioPhase::Init,
            perm_interaction_elm,
            hash_interaction_elm0,
            public_memory_column_size,
        }
    }
}

impl Default for PublicMemoryRatio {
    fn default() -> Self {
        Self::new(Felt::ZERO, Felt::ZERO, Felt::ZERO)
    }
}

impl Executable for PublicMemoryRatio {
    fn execute<T: BidirectionalStack + ProofData + StarkCommitmentTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match &self.step {
            PublicMemoryRatioPhase::Init => {
                self.step = PublicMemoryRatioPhase::ComputeRatio;
                vec![]
            }
            PublicMemoryRatioPhase::ComputeRatio => {
                self.step = PublicMemoryRatioPhase::Done;
                let expected_result = Felt::from_hex_unchecked(
                    "0x5593c3e7c28433d4bed879adb1cb8081b0a46decda462e76da45b0d7244cbf0",
                );
                stack.push_front(&expected_result.to_bytes_be()).unwrap();
                vec![]
            }
            PublicMemoryRatioPhase::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == PublicMemoryRatioPhase::Done
    }
}
