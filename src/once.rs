use std::ops::{Deref, DerefMut};

#[cfg(target_arch = "wasm32")]
pub struct Once<T>(::std::cell::OnceCell<T>);

#[cfg(not(target_arch = "wasm32"))]
pub struct Once<T>(std::sync::OnceLock<T>);

impl<T> Default for Once<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Once<T> {
    pub const fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        return Self(::std::cell::OnceCell::new());
        #[cfg(not(target_arch = "wasm32"))]
        return Self(std::sync::OnceLock::new());
    }

    pub fn set(&self, val: T) -> Result<(), T> {
        self.0.set(val)
    }

    pub fn get(&self) -> Option<&T> {
        self.0.get()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Lazy<T>(once_cell::sync::Lazy<T>);

#[cfg(target_arch = "wasm32")]
pub struct Lazy<T>(once_cell::unsync::Lazy<T>);

impl<T> Lazy<T> {
    pub const fn new(f: fn() -> T) -> Self {
        #[cfg(target_arch = "wasm32")]
        return Self(once_cell::unsync::Lazy::new(f));
        #[cfg(not(target_arch = "wasm32"))]
        return Self(once_cell::sync::Lazy::new(f));
    }
}

impl<T> Deref for Lazy<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Lazy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
