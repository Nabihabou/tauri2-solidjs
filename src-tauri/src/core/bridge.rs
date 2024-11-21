use std::borrow::BorrowMut;
use std::fmt::{self, Debug};
use std::sync::Mutex;

use serde::{Serialize, Serializer};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use tauri::{AppHandle, RunEvent};
use tauri::{Emitter, State};

#[derive(Default, Clone, Debug, Copy)]
enum BridgeStatus {
    #[default]
    Disconnected,
    Connected,
    Loading,
    Authenticated,
}

impl Serialize for BridgeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            BridgeStatus::Disconnected => serializer.serialize_str("disconnected"),
            BridgeStatus::Connected => serializer.serialize_str("connected"),
            BridgeStatus::Loading => serializer.serialize_str("loading"),
            BridgeStatus::Authenticated => serializer.serialize_str("authenticated"),
        }
    }
}

pub struct Bridge<R: Runtime> {
    app: AppHandle<R>,
    state: Mutex<BridgeState>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct BridgeState {
    status: BridgeStatus,
}

impl<R: Runtime> Bridge<R> {
    fn state(&self) -> &Bridge<R> {
        self.app.state::<Bridge<R>>().inner()
    }

    pub fn connect(&mut self) -> Result<BridgeStatus, String> {
        let mut state = *self.state.lock().unwrap();
        state.status = BridgeStatus::Loading;

        println!("Bridge connecting...");

        self.app
            .emit("bridge:status", BridgeStatus::Loading)
            .unwrap();
        Ok(state.status.clone())
    }
}

// Commands
#[tauri::command]
pub fn connect<R: Runtime>(_app: AppHandle<R>, bridge: State<'_, Mutex<Bridge<R>>>) {
    println!("Command -> bridge:connect");
    match bridge.lock().unwrap().connect() {
        Ok(_) => println!("Bridge connected"),
        Err(err) => println!("Bridge connection error: {:?}", err),
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("bridge")
        .invoke_handler(tauri::generate_handler![connect])
        .setup(|app, _api| {
            app.manage(Mutex::new(Bridge {
                app: app.clone(),
                state: Mutex::new(BridgeState::default()),
            }));
            println!("> Bridge plugin initialized");

            Ok(())
        })
        .build()
}
