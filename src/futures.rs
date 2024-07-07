use std::future::Future;
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[cfg(target_arch = "wasm32")]
pub fn spawn(fut: impl Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(fut);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_local(fut: impl Future<Output = ()> + 'static) {
    tokio::task::spawn_local(fut);
}

#[cfg(target_arch = "wasm32")]
pub fn spawn_local(fut: impl Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(fut);
}

#[cfg(target_arch = "wasm32")]
pub async fn sleep(duration: Duration) {
    let ms = duration.as_millis() as i32;
    use wasm_bindgen_futures::JsFuture;
    let fut: JsFuture = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    })
    .into();
    fut.await.unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await
}
