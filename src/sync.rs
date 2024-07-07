use std::ops::{Deref, DerefMut};

#[cfg(target_arch = "wasm32")]
pub struct Mutex<T>(std::cell::RefCell<T>);

#[cfg(not(target_arch = "wasm32"))]
pub struct Mutex<T>(std::sync::Mutex<T>);

#[cfg(target_arch = "wasm32")]
pub struct MutexRef<'a, T>(std::cell::Ref<'a, T>);

#[cfg(target_arch = "wasm32")]
pub struct MutexRefMut<'a, T>(std::cell::RefMut<'a, T>);

#[cfg(not(target_arch = "wasm32"))]
pub struct MutexRef<'a, T>(std::sync::MutexGuard<'a, T>);

#[cfg(not(target_arch = "wasm32"))]
pub struct MutexRefMut<'a, T>(std::sync::MutexGuard<'a, T>);

impl<'a, T> Deref for MutexRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for MutexRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> Deref for MutexRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Mutex<T> {
    pub const fn new(val: T) -> Self {
        #[cfg(target_arch = "wasm32")]
        return Self(std::cell::RefCell::new(val));
        #[cfg(not(target_arch = "wasm32"))]
        return Self(std::sync::Mutex::new(val));
    }

    pub fn lock(&self) -> MutexRef<T> {
        #[cfg(not(target_arch = "wasm32"))]
        return MutexRef(self.0.lock().unwrap());
        #[cfg(target_arch = "wasm32")]
        return MutexRef(self.0.borrow());
    }

    pub fn lock_mut(&self) -> MutexRefMut<T> {
        #[cfg(not(target_arch = "wasm32"))]
        return MutexRefMut(self.0.lock().unwrap());
        #[cfg(target_arch = "wasm32")]
        return MutexRefMut(self.0.borrow_mut());
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub struct RwLock<T>(std::cell::RefCell<T>);

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub struct RwLock<T>(std::sync::RwLock<T>);

#[cfg(not(target_arch = "wasm32"))]
pub struct RwLockReadGuard<'a, T>(std::sync::RwLockReadGuard<'a, T>);

#[cfg(not(target_arch = "wasm32"))]
pub struct RwLockWriteGuard<'a, T>(std::sync::RwLockWriteGuard<'a, T>);

#[cfg(target_arch = "wasm32")]
pub struct RwLockReadGuard<'a, T>(std::cell::Ref<'a, T>);

#[cfg(target_arch = "wasm32")]
pub struct RwLockWriteGuard<'a, T>(std::cell::RefMut<'a, T>);

impl<'a, T> Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> RwLock<T> {
    pub const fn new(val: T) -> Self {
        #[cfg(target_arch = "wasm32")]
        return Self(std::cell::RefCell::new(val));
        #[cfg(not(target_arch = "wasm32"))]
        return Self(std::sync::RwLock::new(val));
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        #[cfg(not(target_arch = "wasm32"))]
        return RwLockReadGuard(self.0.read().unwrap());
        #[cfg(target_arch = "wasm32")]
        return RwLockReadGuard(self.0.borrow());
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        #[cfg(not(target_arch = "wasm32"))]
        return RwLockWriteGuard(self.0.write().unwrap());
        #[cfg(target_arch = "wasm32")]
        return RwLockWriteGuard(self.0.borrow_mut());
    }
}
