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
                self.0.store(val, ::std::sync::atomic::Ordering::SeqCst);
            }

            #[cfg(target_arch = "wasm32")]
            pub fn get(&self) -> $type_name {
                *self.0.borrow()
            }

            #[cfg(not(target_arch = "wasm32"))]
            pub fn get(&self) -> $type_name {
                self.0.load(::std::sync::atomic::Ordering::SeqCst)
            }
        }
    };
}

macro_rules! numeric_cell {
    ($name:ident, $type_name:ident) => {
        impl $name {
            #[cfg(target_arch = "wasm32")]
            pub fn add(&self, val: $type_name) -> $type_name {
                let prev = self.0.borrow();
                *self.0.borrow_mut() += val;
                *prev
            }

            #[cfg(not(target_arch = "wasm32"))]
            pub fn add(&self, val: $type_name) -> $type_name {
                self.0.fetch_add(val, ::std::sync::atomic::Ordering::SeqCst)
            }

            #[cfg(target_arch = "wasm32")]
            pub fn sub(&self, val: $type_name) -> $type_name {
                let prev = self.0.borrow();
                *self.0.borrow_mut() -= val;
                *prev
            }

            #[cfg(not(target_arch = "wasm32"))]
            pub fn sub(&self, val: $type_name) -> $type_name {
                self.0.fetch_sub(val, ::std::sync::atomic::Ordering::SeqCst)
            }

            #[cfg(target_arch = "wasm32")]
            pub fn max(&self, val: $type_name) -> $type_name {
                let prev = self.0.borrow();
                *self.0.borrow_mut() = prev.max(val);
                *prev
            }

            #[cfg(not(target_arch = "wasm32"))]
            pub fn max(&self, val: $type_name) -> $type_name {
                self.0.fetch_max(val, ::std::sync::atomic::Ordering::SeqCst)
            }

            #[cfg(target_arch = "wasm32")]
            pub fn min(&self, val: $type_name) -> $type_name {
                let prev = self.0.borrow();
                *self.0.borrow_mut() = prev.min(val);
                *prev
            }

            #[cfg(not(target_arch = "wasm32"))]
            pub fn min(&self, val: $type_name) -> $type_name {
                self.0.fetch_min(val, ::std::sync::atomic::Ordering::SeqCst)
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

numeric_cell!(U64Cell, u64);
numeric_cell!(U32Cell, u32);
numeric_cell!(U16Cell, u16);
numeric_cell!(U8Cell, u8);
numeric_cell!(UsizeCell, usize);

numeric_cell!(I64Cell, i64);
numeric_cell!(I32Cell, i32);
numeric_cell!(I16Cell, i16);
numeric_cell!(I8Cell, i8);
numeric_cell!(IsizeCell, isize);
