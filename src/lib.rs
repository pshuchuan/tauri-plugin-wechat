use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Wechat;
#[cfg(mobile)]
use mobile::Wechat;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the wechat APIs.
pub trait WechatExt<R: Runtime> {
  fn wechat(&self) -> &Wechat<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WechatExt<R> for T {
  fn wechat(&self) -> &Wechat<R> {
    self.state::<Wechat<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("wechat")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let wechat = mobile::init(app, api)?;
      #[cfg(desktop)]
      let wechat = desktop::init(app, api)?;
      app.manage(wechat);
      Ok(())
    })
    .build()
}
