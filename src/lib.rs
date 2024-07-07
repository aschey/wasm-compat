#[cfg(feature = "cell")]
pub mod cell;
#[cfg(feature = "futures")]
pub mod futures;
#[cfg(feature = "once")]
pub mod once;
#[cfg(feature = "static-init")]
pub mod static_init;
#[cfg(feature = "sync")]
pub mod sync;
#[cfg(feature = "time")]
pub mod time;
