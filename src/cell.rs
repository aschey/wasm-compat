macro_rules! cell {
    ($name:ident, $type_name:ident, $atomic_type_name:path) => {
        #[cfg(target_arch = "wasm32")]
        pub struct $name(::std::cell::RefCell<$type_name>);

        #[cfg(not(target_arch = "wasm32"))]
        pub struct $name($atomic_type_name);

        impl $name {
            pub const fn new(val: $type_name) -> Self {
                #[cfg(target_arch = "wasm32")]
                return Self(::std::cell::RefCell::new(val));
                #[cfg(not(target_arch = "wasm32"))]
                return Self(paste::paste!($atomic_type_name::new(val)));
            }

            #[cfg(target_arch = "wasm32")]
            pub fn set(&self, val: $type_name) {
                *self.0.borrow_mut() = val;
            }

            #[cfg(not(target_arch = "wasm32"))]
            pub fn set(&self, val: $type_name) {
                self.0.store(val, std::sync::atomic::Ordering::SeqCst);
            }

            #[cfg(target_arch = "wasm32")]
            pub fn get(&self) -> $type_name {
                *self.0.borrow()
            }

            #[cfg(not(target_arch = "wasm32"))]
            pub fn get(&self) -> $type_name {
                self.0.load(std::sync::atomic::Ordering::SeqCst)
            }
        }
    };
}

cell!(BoolCell, bool, std::sync::atomic::AtomicBool);

cell!(U64Cell, u64, std::sync::atomic::AtomicU64);
cell!(U32Cell, u32, std::sync::atomic::AtomicU32);
cell!(U16Cell, u16, std::sync::atomic::AtomicU16);
cell!(U8Cell, u8, std::sync::atomic::AtomicU8);
cell!(UsizeCell, usize, std::sync::atomic::AtomicUsize);

cell!(I64Cell, i64, std::sync::atomic::AtomicI64);
cell!(I32Cell, i32, std::sync::atomic::AtomicI32);
cell!(I16Cell, i16, std::sync::atomic::AtomicI16);
cell!(I8Cell, i8, std::sync::atomic::AtomicI8);
cell!(IsizeCell, isize, std::sync::atomic::AtomicIsize);
