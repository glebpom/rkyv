macro_rules! fuzz_target {
    ($name:ident as $ty:ty) => {
        #[test]
        fn $name() {
            let data =
                include_bytes!(concat!("fuzz/", stringify!($name), ".bin"));
            let _ = rkyv::from_bytes::<$ty, rkyv::rancor::Error>(data);
        }
    };
}

fuzz_target!(
    hashmap_string_oob_read as std::collections::HashMap<String, Vec<u32>>
);
