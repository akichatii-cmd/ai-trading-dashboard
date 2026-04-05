// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod models;
mod websocket;
mod ws_server;

use std::sync::Arc;
use tauri::Manager;
use tracing::info;
use ws_server::{WsServerState, run_ws_server, spawn_mock_data_feed, broadcast_log};
use api::{add_log, LogLevel};

fn main() {
    tracing_subscriber::fmt::init();
    
    // Создаем глобальное состояние WebSocket сервера
    let ws_state = Arc::new(WsServerState::new());
    let ws_state_for_setup = ws_state.clone();
    
    tauri::Builder::default()
        .manage(ws_state.clone())
        .setup(move |app| {
            info!("Dashboard starting...");
            
            let ws_state = ws_state_for_setup.clone();
            let app_handle = app.handle();
            
            // Запускаем WebSocket сервер в отдельной задаче
            tauri::async_runtime::spawn(async move {
                if let Err(e) = run_ws_server(ws_state.clone()).await {
                    error!("WebSocket server error: {}", e);
                }
            });
            
            // Запускаем mock data feed для демо
            tauri::async_runtime::spawn(async move {
                spawn_mock_data_feed(ws_state.clone()).await;
            });
            
            // Также подключаемся к торговому боту (backward compatibility)
            let ws_state_bot = ws_state.clone();
            tauri::async_runtime::spawn(async move {
                websocket::connect_to_bot_with_state(app_handle, ws_state_bot).await;
            });
            
            add_log(LogLevel::Info, "Dashboard initialized", "main").await;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::get_bot_status,
            api::start_trading,
            api::stop_trading,
            api::emergency_stop,
            api::get_positions,
            api::get_orders,
            api::close_position,
            api::place_order,
            api::get_signals,
            api::get_logs,
            api::get_historical_data,
            api::modify_position_sl_tp,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
