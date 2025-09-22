use std::fmt::Debug;

pub const CAPACITY: usize = 65536;
pub const LENGTH_SIZE: usize = 2;
// pub const MASK_VALUES_ROWS_SIZE: usize = 4107;
// pub const MASK_VALUES_COLUMNS_SIZE: usize = 8;
pub const MASK_VALUES_SIZE: usize = 192;
pub const OODS_VALUES_SIZE: usize = 194;
pub const COLUMN_VALUES_SIZE: usize = 10;
pub const DOMAINS_SIZE: usize = 31;
pub const POWS_SIZE: usize = 134;
pub const BITS_SIZE: usize = 15;
pub const POSEIDON_BITS_SIZE: usize = 18;
//todo: make it dependent on Layout
pub const N_CONSTRAINTS: usize = 194;
pub const CONSTRAINT_DEGREE: usize = 2;
pub const NUM_COLUMNS_FIRST: u32 = 6;
pub const NUM_COLUMNS_SECOND: u32 = 2;

/// Trait for safely casting between account data and Rust types
pub trait AccountCast: Sized {
    /// Cast a slice to an immutable reference of Self
    fn cast(slice: &[u8]) -> &Self {
        assert_eq!(slice.len(), std::mem::size_of::<Self>());
        unsafe { &*(slice.as_ptr() as *const Self) }
    }

    /// Cast a mutable slice to a mutable reference of Self
    fn cast_mut(slice: &mut [u8]) -> &mut Self {
        assert_eq!(slice.len(), std::mem::size_of::<Self>());
        unsafe { &mut *(slice.as_mut_ptr() as *mut Self) }
    }
}

pub trait BidirectionalStack {
    type Error: std::error::Error + Debug;

    fn push_front(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    fn push_back(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    fn pop_front(&mut self);
    fn pop_back(&mut self);
    fn borrow_front(&self) -> &[u8];
    fn borrow_back(&self) -> &[u8];
    fn borrow_mut_front(&mut self) -> &mut [u8];
    fn borrow_mut_back(&mut self) -> &mut [u8];
    fn is_empty_front(&self) -> bool;
    fn is_empty_back(&self) -> bool;
}

pub trait Scheduler: BidirectionalStack {
    fn push_task<T: Executable>(&mut self, task: T) {
        self.push_back(&task.to_vec_with_type_tag()).unwrap();
    }
    fn push_data(&mut self, data: &[u8]) {
        self.push_front(data).unwrap();
    }
    fn pop_task(&mut self) {
        self.pop_back();
    }
    fn pop_data(&mut self) {
        self.pop_front();
    }
}

pub trait ProofData {
    /// Get a reference to the proof data as any type T
    fn get_proof_reference<T: Sized>(&self) -> &T {
        let bytes = self.get_proof_bytes();
        assert_eq!(bytes.len(), std::mem::size_of::<T>());
        unsafe { &*(bytes.as_ptr() as *const T) }
    }
    fn get_proof_bytes(&self) -> &[u8];
}
/// Trait for providing automatic type identification with cryptographic hashing
pub trait TypeIdentifiable {
    /// Returns a unique type ID based on the type name using a cryptographic hash
    const TYPE_ID: u32;
}

// For automatic implementation of TypeIdentifiable
#[macro_export]
macro_rules! impl_type_identifiable {
    ($type:ty) => {
        impl TypeIdentifiable for $type {
            // Generate a compile-time constant value based on type name using FNV-1a hash
            // This is a non-cryptographic but better hash function that can be used in const contexts
            const TYPE_ID: u32 = {
                // Create a hash from the fully qualified type name at compile time
                let full_type_name = concat!(module_path!(), "::", stringify!($type));
                let bytes = full_type_name.as_bytes();
                let len = bytes.len();

                // FNV-1a hash algorithm constants
                const FNV_PRIME: u32 = 16777619;
                const FNV_OFFSET_BASIS: u32 = 2166136261;

                // FNV-1a hash computation
                let mut hash = FNV_OFFSET_BASIS;
                let mut i = 0;
                while i < len {
                    hash ^= bytes[i] as u32;
                    hash = hash.wrapping_mul(FNV_PRIME);
                    i += 1;
                }

                hash
            };
        }
    };
}

pub trait Executable: Sized + TypeIdentifiable {
    /// The type tag is now automatically derived from TypeIdentifiable trait
    /// Using u32 instead of u8 for a much larger ID space
    const TYPE_TAG: u32 = Self::TYPE_ID;
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>>;
    fn is_finished(&mut self) -> bool {
        false
    }

    /// Cast a slice to an immutable reference of Self
    fn cast(slice: &[u8]) -> &Self {
        assert_eq!(slice.len(), std::mem::size_of::<Self>());
        unsafe { &*(slice.as_ptr() as *const Self) }
    }

    /// Cast a mutable slice to a mutable reference of Self
    fn cast_mut(slice: &mut [u8]) -> &mut Self {
        assert_eq!(slice.len(), std::mem::size_of::<Self>());
        unsafe { &mut *(slice.as_mut_ptr() as *mut Self) }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                (self as *mut Self) as *mut u8,
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn to_vec_with_type_tag(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend_from_slice(&Self::TYPE_TAG.to_be_bytes());
        vec.extend_from_slice(self.as_bytes());
        vec
    }
}
