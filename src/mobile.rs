use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_wechat);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Wechat<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.tauriplugins.wechat", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_wechat)?;
  Ok(Wechat(handle))
}

/// Access to the wechat APIs.
pub struct Wechat<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Wechat<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }
}

pub struct Sdk{
  APP_ID:String,
  IWXAPI:String,
  state:String
}

impl Sdk {
  pub fn new(appid:String,scope:String) -> Self {

  }



}