pub use borsh_size_derive::BorshSize;
pub trait BorshSize {

    fn calculate_borsh_size(&self) -> usize;
}

const ARRAY_SIZE_BYTES_SIZE: usize  = 4;


impl<T> BorshSize for [T] where T:BorshSize {

    fn calculate_borsh_size(&self) -> usize {
        ARRAY_SIZE_BYTES_SIZE + (&self[..]).iter()
        .map(|a| { a.calculate_borsh_size() } )
        .sum::<usize>()
    }
    
}


impl <T> BorshSize for Vec<T> where T:BorshSize {

    fn calculate_borsh_size(&self) -> usize {
        ARRAY_SIZE_BYTES_SIZE + 
        &self.iter().map(BorshSize::calculate_borsh_size).sum()
    }

}

impl BorshSize for str {

    fn calculate_borsh_size(&self) -> usize {
        ARRAY_SIZE_BYTES_SIZE + self.len()
    }

}

impl BorshSize for &str {

    fn calculate_borsh_size(&self) -> usize {
        ARRAY_SIZE_BYTES_SIZE + self.len()
    }

}

impl BorshSize for String {

    fn calculate_borsh_size(&self) -> usize {
        ARRAY_SIZE_BYTES_SIZE + self.capacity()
    }

}

impl BorshSize for &String {

    fn calculate_borsh_size(&self) -> usize {
        ARRAY_SIZE_BYTES_SIZE + self.capacity()
    }

}


macro_rules! array_heap_size {
    ($($n:tt)+) => {
        $(
        impl<T> BorshSize for [T; $n]
        where
            T: BorshSize,
        {

            #[inline]
            fn calculate_borsh_size(&self) -> usize {
                ARRAY_SIZE_BYTES_SIZE + (&self[..]).iter()
                .map(|a| {
                    a.calculate_borsh_size()
                })
                .sum::<usize>()
            }
        }
        )*
    };
}

array_heap_size!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 128 192 256 384 512 1024 2048 4096 8192 16384 1048576 2097152 3145728 4194304);

#[macro_export]
macro_rules! non_dynamic_const_heap_size {
    ($($ty:ty)*) => {
        $(impl BorshSize for $ty {
    
            #[inline]
            fn calculate_borsh_size(&self) -> usize {
                core::mem::size_of::<$ty>()
            }
        })*
    };
}
non_dynamic_const_heap_size!(() u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize bool char f32 f64);


#[cfg(test)]
mod tests {
    use crate::BorshSize;

    #[test]
    fn test_array_u16_size() {
        let s = 33u16; // 2
        let serialized_size = s.calculate_borsh_size();
        assert_eq!(serialized_size, 2);
    }

    #[test]
    fn test_array_u32_array_size() {
        let s = [33u32, 67u32]; // 4 + 4 + 4
        let serialized_size = s.calculate_borsh_size();
        assert_eq!(serialized_size, 12);
    }

    #[test]
    fn test_vec_u16_size() {
        let s = vec![33u16, 67u16]; // 4 + 2 + 2
        let serialized_size = s.calculate_borsh_size();
        assert_eq!(serialized_size, 8);
    }

    #[test]
    fn test_str_size() {
        let s = "test123"; // 4 + 7
        let serialized_size = s.calculate_borsh_size();
        assert_eq!(serialized_size, 11);
    }

    #[test]
    fn test_string_size() {
        let s = "test123".to_string(); // 4 + 7
        let serialized_size = s.calculate_borsh_size();
        assert_eq!(serialized_size, 11);
    }

    #[test]
    fn test_vec_str_size() {
        let s = vec!["aa", "abcd"]; // 4 + ( (4 + 2) + (4 + 4) )
        let serialized_size = s.calculate_borsh_size();
        assert_eq!(serialized_size, 18);
    }

    #[test]
    fn test_vec_string_size() {
        let s = vec!["aa".to_string(), "abcd".to_string()]; // 4 + ( (4 + 2) + (4 + 4) )
        let serialized_size = s.calculate_borsh_size();
        assert_eq!(serialized_size, 18);
    }

    #[test]
    fn test_array_str_size() {
        let s = &["aa", "abc"]; // 4 + ( (4 + 2) + (4 + 3) )
        let serialized_size = s.calculate_borsh_size();
        assert_eq!(serialized_size, 17);
    }

    #[test]
    fn test_array_string_size() {
        let s = &["aa".to_string(), "abc".to_string()]; // 4 + ( (4 + 2) + (4 + 3) )
        let serialized_size = s.calculate_borsh_size();
        assert_eq!(serialized_size, 17);
    }

    #[test]
    fn test_slice_string_size() {
        let s = &["aa".to_string(), "abc".to_string(), "a".to_string()]; // 4 + ( (4 + 2) + (4 + 3) + (4 + 1) )
        let serialized_size = s[..].calculate_borsh_size();
        assert_eq!(serialized_size, 22);
    }

    #[test]
    fn test_slice_str_size() {
        let s = &["aa", "abc", "a"]; // 4 + ( (4 + 2) + (4 + 3) + (4 + 1) )
        let serialized_size = s[..].calculate_borsh_size();
        assert_eq!(serialized_size, 22);
    }


    #[test]
    fn test_slice_u64_size() {
        let s = &[1u64, 2u64]; // 4 + 8 + 8
        let serialized_size = s[..].calculate_borsh_size();
        assert_eq!(serialized_size, 20);
    }


}