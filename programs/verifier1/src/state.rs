use crate::error::VerifierError;
use types::swiftness::stark::types::{cast_struct_to_slice, StarkProof};
use utils_1::{AccountCast, BidirectionalStack, ProofData};
use utils_1::{CAPACITY, LENGTH_SIZE};

/// Define the type of state stored in accounts
#[repr(C)]
#[derive(Debug)]
pub struct BidirectionalStackAccount {
    pub front_index: usize,
    pub back_index: usize,
    pub buffer: [u8; CAPACITY],
    pub proof: StarkProof,
}
impl Default for BidirectionalStackAccount {
    fn default() -> Self {
        Self {
            front_index: 0,
            back_index: CAPACITY,
            buffer: [0; CAPACITY],
            proof: StarkProof::default(),
        }
    }
}

impl BidirectionalStackAccount {
    pub fn simulate(&mut self) -> u128 {
        let mut simulation_steps = 0;
        while !self.is_empty_back() {
            self.execute();
            simulation_steps += 1;
        }
        simulation_steps
    }
}

impl AccountCast for BidirectionalStackAccount {}

impl BidirectionalStack for BidirectionalStackAccount {
    type Error = VerifierError;

    fn push_front(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        for byte in data {
            self.buffer[self.front_index] = *byte;
            self.front_index = self.front_index.saturating_add(1);
        }

        let data_length = data.len();
        for i in 0..LENGTH_SIZE {
            self.buffer[self.front_index] = ((data_length >> (i * 8)) & 0xFF).try_into()?;
            self.front_index = self.front_index.saturating_add(1);
        }

        Ok(())
    }

    fn push_back(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        for byte in data.iter().rev() {
            self.back_index = self.back_index.saturating_sub(1);
            self.buffer[self.back_index] = *byte;
        }

        let data_length = data.len();
        for i in 0..LENGTH_SIZE {
            self.back_index = self.back_index.saturating_sub(1);
            self.buffer[self.back_index] = ((data_length >> (i * 8)) & 0xFF).try_into()?;
        }

        Ok(())
    }

    fn pop_front(&mut self) {
        let mut data_length = 0_usize;
        for _ in 0..LENGTH_SIZE {
            self.front_index = self.front_index.saturating_sub(1);
            let x: usize = self.buffer[self.front_index].into();
            data_length = (data_length << 8) | x;
        }

        self.front_index = self.front_index.saturating_sub(data_length);
    }

    fn pop_back(&mut self) {
        let mut data_length = 0_usize;
        for _ in 0..LENGTH_SIZE {
            let x: usize = self.buffer[self.back_index].into();
            data_length = (data_length << 8) | x;
            self.back_index = self.back_index.saturating_add(1);
        }

        self.back_index = self.back_index.saturating_add(data_length);
    }

    fn borrow_front(&self) -> &[u8] {
        let mut data_length = 0_usize;
        for i in 1..=LENGTH_SIZE {
            let x: usize = self.buffer[self.front_index.saturating_sub(i)].into();
            data_length = (data_length << 8) | x;
        }

        &self.buffer[self.front_index.saturating_sub(data_length + LENGTH_SIZE)
            ..self.front_index.saturating_sub(LENGTH_SIZE)]
    }

    fn borrow_back(&self) -> &[u8] {
        let mut data_length = 0_usize;
        for i in 0..LENGTH_SIZE {
            let x: usize = self.buffer[self.back_index.saturating_add(i)].into();
            data_length = (data_length << 8) | x;
        }

        &self.buffer[self.back_index.saturating_add(LENGTH_SIZE)
            ..self.back_index.saturating_add(LENGTH_SIZE + data_length)]
    }

    fn borrow_mut_front(&mut self) -> &mut [u8] {
        let mut data_length = 0_usize;
        for i in 1..=LENGTH_SIZE {
            let x: usize = self.buffer[self.front_index.saturating_sub(i)].into();
            data_length = (data_length << 8) | x;
        }

        &mut self.buffer[self.front_index.saturating_sub(data_length + LENGTH_SIZE)
            ..self.front_index.saturating_sub(LENGTH_SIZE)]
    }

    fn borrow_mut_back(&mut self) -> &mut [u8] {
        let mut data_length = 0_usize;
        for i in 0..LENGTH_SIZE {
            let x: usize = self.buffer[self.back_index.saturating_add(i)].into();
            data_length = (data_length << 8) | x;
        }

        &mut self.buffer[self.back_index.saturating_add(LENGTH_SIZE)
            ..self.back_index.saturating_add(LENGTH_SIZE + data_length)]
    }

    fn is_empty_front(&self) -> bool {
        self.front_index == 0
    }

    fn is_empty_back(&self) -> bool {
        self.back_index == CAPACITY
    }
}

impl ProofData for BidirectionalStackAccount {
    fn get_proof_bytes(&self) -> &[u8] {
        cast_struct_to_slice(&self.proof)
    }
}
#[cfg(test)]
mod tests {
    use crate::state::{BidirectionalStackAccount, CAPACITY};
    use utils_1::BidirectionalStack;

    #[test]
    fn test_default() {
        let stack = BidirectionalStackAccount::default();
        assert_eq!(stack.front_index, 0);
        assert_eq!(stack.back_index, CAPACITY);
        assert_eq!(stack.buffer, [0; CAPACITY]);
    }

    #[test]
    fn test_push_front_and_borrow_front() {
        let mut stack = BidirectionalStackAccount::default();

        // Push data to front
        let data = [1, 2, 3, 4];
        stack.push_front(&data).unwrap();

        // Borrow and verify
        let borrowed = stack.borrow_front();
        assert_eq!(borrowed, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_push_back_and_borrow_back() {
        let mut stack = BidirectionalStackAccount::default();

        // Push data to back
        let data = [1, 2, 3, 4];
        stack.push_back(&data).unwrap();

        // Borrow and verify
        let borrowed = stack.borrow_back();
        assert_eq!(borrowed, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_push_pop_front() {
        let mut stack = BidirectionalStackAccount::default();

        // Initial state
        assert_eq!(stack.front_index, 0);

        // Push data
        let data = [5, 6, 7, 8];
        stack.push_front(&data).unwrap();

        // Verify front_index moved
        assert_eq!(stack.front_index, data.len() + crate::state::LENGTH_SIZE);

        // Pop data
        stack.pop_front();

        // Verify front_index returned to initial state
        assert_eq!(stack.front_index, 0);
    }

    #[test]
    fn test_push_pop_back() {
        let mut stack = BidirectionalStackAccount::default();

        // Initial state
        assert_eq!(stack.back_index, CAPACITY);

        // Push data
        let data = [5, 6, 7, 8];
        stack.push_back(&data).unwrap();

        // Verify back_index moved
        assert_eq!(
            stack.back_index,
            CAPACITY - data.len() - crate::state::LENGTH_SIZE
        );

        // Pop data
        stack.pop_back();

        // Verify back_index returned to initial state
        assert_eq!(stack.back_index, CAPACITY);
    }

    #[test]
    fn test_multiple_push_front() {
        let mut stack = BidirectionalStackAccount::default();

        // Push first data
        let data1 = [1, 2, 3];
        stack.push_front(&data1).unwrap();

        // Push second data
        let data2 = [4, 5, 6, 7];
        stack.push_front(&data2).unwrap();

        // Borrow and verify most recent data
        let borrowed = stack.borrow_front();
        assert_eq!(borrowed, &[4, 5, 6, 7]);

        // Pop most recent data
        stack.pop_front();

        // Verify we can access the first data
        let borrowed = stack.borrow_front();
        assert_eq!(borrowed, &[1, 2, 3]);
    }

    #[test]
    fn test_multiple_push_back() {
        let mut stack = BidirectionalStackAccount::default();

        // Push first data
        let data1 = [1, 2, 3];
        stack.push_back(&data1).unwrap();

        // Push second data
        let data2 = [4, 5, 6, 7];
        stack.push_back(&data2).unwrap();

        // Borrow and verify most recent data
        let borrowed = stack.borrow_back();
        assert_eq!(borrowed, &[4, 5, 6, 7]);

        // Pop most recent data
        stack.pop_back();

        // Verify we can access the first data
        let borrowed = stack.borrow_back();
        assert_eq!(borrowed, &[1, 2, 3]);
    }

    #[test]
    fn test_bidirectional_operations() {
        let mut stack = BidirectionalStackAccount::default();

        // Push data to both ends
        stack.push_front(&[1, 2, 3]).unwrap();
        stack.push_back(&[7, 8, 9]).unwrap();

        // Verify data at both ends
        assert_eq!(stack.borrow_front(), &[1, 2, 3]);
        assert_eq!(stack.borrow_back(), &[7, 8, 9]);

        // Push more data to both ends
        stack.push_front(&[4, 5, 6]).unwrap();
        stack.push_back(&[10, 11, 12]).unwrap();

        // Verify most recent data
        assert_eq!(stack.borrow_front(), &[4, 5, 6]);
        assert_eq!(stack.borrow_back(), &[10, 11, 12]);

        // Pop from both ends
        stack.pop_front();
        stack.pop_back();

        // Verify earlier data
        assert_eq!(stack.borrow_front(), &[1, 2, 3]);
        assert_eq!(stack.borrow_back(), &[7, 8, 9]);
    }

    #[test]
    fn test_borrow_mut_front() {
        let mut stack = BidirectionalStackAccount::default();

        // Push data
        stack.push_front(&[1, 2, 3, 4]).unwrap();

        // Get mutable reference and modify
        {
            let data = stack.borrow_mut_front();
            data[0] = 5;
            data[3] = 8;
        }

        // Verify modifications
        assert_eq!(stack.borrow_front(), &[5, 2, 3, 8]);
    }

    #[test]
    fn test_borrow_mut_back() {
        let mut stack = BidirectionalStackAccount::default();

        // Push data
        stack.push_back(&[1, 2, 3, 4]).unwrap();

        // Get mutable reference and modify
        {
            let data = stack.borrow_mut_back();
            data[0] = 5;
            data[3] = 8;
        }

        // Verify modifications
        assert_eq!(stack.borrow_back(), &[5, 2, 3, 8]);
    }

    #[test]
    fn test_empty_data() {
        let mut stack = BidirectionalStackAccount::default();

        // Push empty data
        let empty: [u8; 0] = [];
        stack.push_front(&empty).unwrap();
        stack.push_back(&empty).unwrap();

        // Verify empty data
        assert_eq!(stack.borrow_front(), &[]);
        assert_eq!(stack.borrow_back(), &[]);

        // Pop empty data
        stack.pop_front();
        stack.pop_back();

        // Verify state after popping
        assert_eq!(stack.front_index, 0);
        assert_eq!(stack.back_index, CAPACITY);
    }

    #[test]
    fn test_large_data() {
        let mut stack = BidirectionalStackAccount::default();

        // Create larger data (1KB)
        let large_data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();

        // Push to front and back
        stack.push_front(&large_data).unwrap();
        stack.push_back(&large_data).unwrap();

        // Verify data
        assert_eq!(stack.borrow_front(), large_data.as_slice());
        assert_eq!(stack.borrow_back(), large_data.as_slice());
    }

    #[test]
    fn test_alternating_operations() {
        let mut stack = BidirectionalStackAccount::default();

        // Alternating push operations
        stack.push_front(&[1, 2]).unwrap();
        stack.push_back(&[3, 4]).unwrap();
        stack.push_front(&[5, 6]).unwrap();
        stack.push_back(&[7, 8]).unwrap();

        // Verify most recent data
        assert_eq!(stack.borrow_front(), &[5, 6]);
        assert_eq!(stack.borrow_back(), &[7, 8]);

        // Alternating pop operations
        stack.pop_front();
        assert_eq!(stack.borrow_front(), &[1, 2]);

        stack.pop_back();
        assert_eq!(stack.borrow_back(), &[3, 4]);
    }

    #[test]
    fn test_multiple_operations_sequence() {
        let mut stack = BidirectionalStackAccount::default();

        // Push multiple items
        for i in 0..5 {
            let data = [i, i + 1, i + 2];
            stack.push_front(&data).unwrap();
        }

        // Verify last item
        assert_eq!(stack.borrow_front(), &[4, 5, 6]);

        // Pop and verify each item in reverse order
        for i in (0..5).rev() {
            let expected = [i, i + 1, i + 2];
            assert_eq!(stack.borrow_front(), &expected);
            stack.pop_front();
        }

        // Stack should be empty now (front_index back to 0)
        assert_eq!(stack.front_index, 0);
    }

    #[test]
    fn test_mixed_data_types() {
        let mut stack = BidirectionalStackAccount::default();

        // Push different types of data (converted to bytes)
        let string_data = "Hello, world!".as_bytes();
        let numeric_data = &[0, 1, 2, 3, 4, 5];
        let binary_data = &[0xFF, 0xAA, 0x55, 0x00];

        stack.push_front(string_data).unwrap();
        stack.push_back(numeric_data).unwrap();
        stack.push_front(binary_data).unwrap();

        // Verify data
        assert_eq!(stack.borrow_front(), binary_data);
        assert_eq!(stack.borrow_back(), numeric_data);

        // Pop and verify
        stack.pop_front();
        assert_eq!(stack.borrow_front(), string_data);
    }

    #[test]
    fn test_boundary_conditions() {
        let mut stack = BidirectionalStackAccount::default();

        // Test with single byte
        stack.push_front(&[42]).unwrap();
        assert_eq!(stack.borrow_front(), &[42]);
        stack.pop_front();

        // Test with boundary values
        let boundary_data = [0, 255, 1, 254];
        stack.push_front(&boundary_data).unwrap();
        assert_eq!(stack.borrow_front(), &boundary_data);
    }

    #[test]
    fn test_capacity_management() {
        let mut stack = BidirectionalStackAccount::default();

        // Calculate how much data we can safely push (leaving some margin)
        // This is a simple test to verify we can use a significant portion of capacity
        let safe_capacity = CAPACITY / 4; // Using 1/4 of capacity for safety
        let data = vec![1u8; safe_capacity];

        // We should be able to push this data both to front and back
        stack.push_front(&data).unwrap();
        stack.push_back(&data).unwrap();

        // Verify the data
        assert_eq!(stack.borrow_front().len(), safe_capacity);
        assert_eq!(stack.borrow_back().len(), safe_capacity);

        // The front_index and back_index should reflect the data size + length bytes
        assert_eq!(stack.front_index, safe_capacity + crate::state::LENGTH_SIZE);
        assert_eq!(
            stack.back_index,
            CAPACITY - safe_capacity - crate::state::LENGTH_SIZE
        );
    }
}
