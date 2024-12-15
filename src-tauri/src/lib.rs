use std::sync::Arc;
use std::time::Duration;

use prost::Message;

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

pub trait PrettyPositionString {
    fn to_pretty_position_string(
        &self,
        routes: &std::collections::HashMap<String, gtfs_structures::Route, std::hash::RandomState>,
        stops: &std::collections::HashMap<
            String,
            Arc<gtfs_structures::Stop>,
            std::hash::RandomState,
        >,
    ) -> std::option::Option<String>;
}

impl PrettyPositionString for gtfs_rt::VehiclePosition {
    fn to_pretty_position_string(
        &self,
        routes: &std::collections::HashMap<String, gtfs_structures::Route, std::hash::RandomState>,
        stops: &std::collections::HashMap<
            String,
            Arc<gtfs_structures::Stop>,
            std::hash::RandomState,
        >,
    ) -> std::option::Option<String> {
        if self.position.is_some() {
            let current_position = self.position.as_ref().unwrap();
            let this_trip = self.trip.as_ref().unwrap();
            let this_route_plan = routes.get(
                this_trip
                    .route_id
                    .as_ref()
                    .unwrap_or(&"<no route id>".to_string()),
            );

            Some(format!(
                "{} (direction {}): currently {} {} ({}), speed {} mph",
                this_route_plan
                    .and_then(|this_route| Some(
                        this_route
                            .long_name
                            .clone()
                            .unwrap_or("<no route name>".to_string())
                    ))
                    .unwrap_or("<no route found>".to_string()),
                this_trip.direction_id(),
                match self.current_status() {
                    gtfs_rt::vehicle_position::VehicleStopStatus::InTransitTo => "in transit to",
                    gtfs_rt::vehicle_position::VehicleStopStatus::StoppedAt => "stopped at",
                    gtfs_rt::vehicle_position::VehicleStopStatus::IncomingAt => "incoming at",
                },
                match stops.get(self.stop_id.as_ref().unwrap_or(&"".to_string())) {
                    Some(stop) => stop.name.clone().unwrap_or("".to_string()),
                    None => "".to_string(),
                },
                self.stop_id.as_ref().unwrap_or(&"".to_string()),
                current_position.speed.unwrap_or(0.0) * 60.0 * 60.0 / 1.60934 / 1000.0
            ))
        } else {
            None
        }
    }
}

async fn data_fetch_task(app: &tauri::AppHandle) {
    // get static data from server
    let _amtrak_url = "https://content.amtrak.com/content/gtfs/GTFS.zip";
    let amtrak_midwest_routes =
        ["Empire Builder", "Borealis", "Hiawatha Service"].map(|entity| entity.to_string());

    let gtfs = gtfs_structures::GtfsReader::default()
        .read_stop_times(false)
        .read_from_url_async(_amtrak_url)
        .await
        .expect("failed to read gtfs");

    let amtrak_midwest_route_ids: std::vec::Vec<_> = gtfs
        .routes
        .iter()
        .filter(|entity| {
            amtrak_midwest_routes.contains(&entity.1.long_name.as_ref().unwrap_or(&"".to_string()))
        })
        .map(|entity| entity.0)
        .collect();

    loop {
        // get realtime position data
        let result = reqwest::get("https://asm-backend.transitdocs.com/gtfs/amtrak").await;

        if result.is_ok() {
            let body = result.unwrap().bytes().await;
            if body.is_ok() {
                let rt_data = body.unwrap();
                let message = gtfs_rt::FeedMessage::decode(rt_data);
                if message.is_ok() {
                    message
                        .unwrap()
                        .entity
                        .iter()
                        .filter(|entity| entity.vehicle.is_some())
                        .filter(|entity| {
                            let route_id = &entity
                                .vehicle
                                .as_ref()
                                .unwrap()
                                .trip
                                .as_ref()
                                .unwrap()
                                .route_id
                                .clone()
                                .unwrap_or("".to_string());
                            amtrak_midwest_route_ids.iter().any(|&id| id == route_id)
                                && entity
                                    .vehicle
                                    .as_ref()
                                    .unwrap()
                                    .position
                                    .as_ref()
                                    .unwrap()
                                    .bearing
                                    .is_some()
                        })
                        .for_each(|entity| {
                            match entity
                                .vehicle
                                .as_ref()
                                .unwrap()
                                .to_pretty_position_string(&gtfs.routes, &gtfs.stops)
                            {
                                Some(trip_data) => println!("{}", trip_data),
                                None => println!("No data???"),
                            }
                        });
                }
            }
        }

        // send to frontend
        app.emit(
            "station-update",
            StationUpdate {
                station: "MKE",
                state: [
                    StationStateForTrain {
                        train: "EmpireBuilder",
                        state: "Incoming",
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
        tokio::time::sleep(Duration::from_secs(10)).await;
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
