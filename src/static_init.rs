#[cfg(not(target_arch = "wasm32"))]
pub struct LocalKey<T>(T);

#[cfg(not(target_arch = "wasm32"))]
impl<T> LocalKey<T> {
    pub const fn new(val: T) -> Self {
        Self(val)
    }

    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&self.0)
    }
}

#[macro_export]
macro_rules! static_init {
    // empty (base case for the recursion)
    () => {};

    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = const $init:block; $($rest:tt)*) => (
        #[cfg(target_arch="wasm32")]
        thread_local! {
            $vis static $name: $t = const $init;
        }

        #[cfg(not(target_arch="wasm32"))]
        $vis static $name: $crate::static_init::LocalKey<$t> = const $crate::static_init::LocalKey::new($init);

        $crate::local!($($rest)*);
    );

    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = const $init:block) => (
        #[cfg(target_arch="wasm32")]
        thread_local! {
            $vis static $name: $t = const $init;
        }

        #[cfg(not(target_arch="wasm32"))]
        $vis static $name: $crate::static_init::LocalKey<$t> = const $crate::static_init::LocalKey::new($init);

        $crate::static_init!($($rest)*);
    );

    // process multiple declarations
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => (
        #[cfg(target_arch="wasm32")]
        thread_local! {
            $vis static $name: $t = $init;
        }

        #[cfg(not(target_arch="wasm32"))]
        $vis static $name: $crate::static_init::LocalKey<$t> = $crate::static_init::LocalKey::new($init);

        $crate::static_init!($($rest)*);
    );

    // handle a single declaration
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr) => (
        #[cfg(target_arch="wasm32")]
        thread_local! {
            $vis static $name: $t = const $init;
        }
        #[cfg(not(target_arch="wasm32"))]
        $vis static $name: $crate::static_init::LocalKey<$t> = $crate::static_init::LocalKey::new($init);

        $crate::static_init!($($rest)*);
    );
}
