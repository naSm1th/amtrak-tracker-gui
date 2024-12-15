use std::{array, thread::current, time::Duration};

use serde::Serialize;
use tauri::Emitter;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]

struct StationStateForTrain<'a> {
    train: &'a str,
    state: &'a str,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct StationUpdate<'a> {
    station: &'a str,
    state: [StationStateForTrain<'a>; 3],
}

async fn data_fetch_task(app: &tauri::AppHandle) {
    let mut current_state: bool = false;
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
        app.emit(
            "station-update",
            StationUpdate {
                station: "MKE",
                state: [
                    StationStateForTrain {
                        train: "EmpireBuilder",
                        state: if current_state {"Empty"} else {"Incoming"},
                    },
                    StationStateForTrain {
                        train: "Borealis",
                        state: "Empty",
                    },
                    StationStateForTrain {
                        train: "Hiawatha",
                        state: "Empty",
                    },
                ],
            },
        )
        .expect("Failed to emit message");
        println!("Sent message OwO");
        current_state = !current_state;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move { data_fetch_task(&app_handle).await });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
