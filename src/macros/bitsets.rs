trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
}

impl BitSet for u64 {
    fn clear(&mut self, index: usize) {
        *self &= !(1 << index)
    }

    fn is_set(&self, index: usize) -> bool {
        (*self >> index) & 1 == 1
    }

    fn set(&mut self, index: usize) {
        *self |=1 << index
    }
}

macro_rules! int_bitset {
    ($ty:ty) => {
        impl BitSet for $ty {
            fn clear(&mut self, index: usize) {
                *self &= !(1 << index);
            }

            fn is_set(&self, index: usize) -> bool {
                (*self >> index) & 1 == 1
            }

            fn set(&mut self, index: usize) {
                *self |=1 << index;
            }
        }
    }
}

macro_rules! hash {
    ($( $key:expr => $value:expr),*) => {{
        let mut hashmap = ::std::collections::HashMap::new();
        $(hashmap.insert($key, $value);)*
        hashmap
    }}
}
#[test]
fn test_bitset() {
    int_bitset!(i32);
}

#[test]
fn test_hashmap() {
    let hashmap = hash! {
        "one" => 1,
        "two" => 2
    };

    assert_eq!(*hashmap.get("one").unwrap(), 1)

}