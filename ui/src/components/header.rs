use leptos::*;
use crate::state::{DashboardState, TradingMode};
use crate::tauri_api;

#[component]
pub fn Header() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (mode, set_mode) = create_slice(
        state,
        |state| state.system.mode.clone(),
        |state, mode| state.system.mode = mode,
    );
    
    let (running, set_running) = create_slice(
        state,
        |state| state.system.running,
        |state, running| state.system.running = running,
    );
    
    let on_start = move |_| {
        let mode_str = match mode.get() {
            TradingMode::Live => "live",
            TradingMode::Paper => "paper",
            TradingMode::Demo => "demo",
            TradingMode::Off => "demo",
        };
        
        spawn_local(async move {
            match tauri_api::start_trading(mode_str).await {
                Ok(msg) => {
                    set_running.set(true);
                    log::info!("{}", msg);
                }
                Err(e) => {
                    log::error!("Failed to start trading: {}", e);
                }
            }
        });
    };
    
    let on_stop = move |_| {
        spawn_local(async move {
            match tauri_api::stop_trading().await {
                Ok(msg) => {
                    set_running.set(false);
                    log::info!("{}", msg);
                }
                Err(e) => {
                    log::error!("Failed to stop trading: {}", e);
                }
            }
        });
    };
    
    let (show_kill_confirm, set_show_kill_confirm) = create_signal(false);
    
    let on_kill = move |_| {
        set_show_kill_confirm.set(true);
    };
    
    let confirm_kill = move |_| {
        spawn_local(async move {
            match tauri_api::emergency_stop().await {
                Ok(msg) => {
                    set_running.set(false);
                    set_show_kill_confirm.set(false);
                    log::info!("EMERGENCY STOP: {}", msg);
                }
                Err(e) => {
                    log::error!("Failed emergency stop: {}", e);
                }
            }
        });
    };

    view! {
        <header class="h-[60px] bg-card border-b border-subtle flex items-center justify-between px-6">
            // Logo & Title
            <div class="flex items-center gap-4">
                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-success to-info flex items-center justify-center">
                    <svg class="w-4 h-4 text-void" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/>
                    </svg>
                </div>
                <div>
                    <h1 class="text-sm font-semibold text-primary">"AI Trading Bot"</h1>
                    <p class="text-xs text-muted">"v5.0.0"</p>
                </div>
            </div>
            
            // Mode Selection
            <div class="flex items-center gap-2">
                <ModeRadio 
                    label="LIVE" 
                    mode={TradingMode::Live}
                    selected={mode.get() == TradingMode::Live}
                    on_click={move |_| set_mode.set(TradingMode::Live)}
                    color_class="text-success"
                />
                <ModeRadio 
                    label="PAPER" 
                    mode={TradingMode::Paper}
                    selected={mode.get() == TradingMode::Paper}
                    on_click={move |_| set_mode.set(TradingMode::Paper)}
                    color_class="text-warning"
                />
                <ModeRadio 
                    label="DEMO" 
                    mode={TradingMode::Demo}
                    selected={mode.get() == TradingMode::Demo}
                    on_click={move |_| set_mode.set(TradingMode::Demo)}
                    color_class="text-info"
                />
                <ModeRadio 
                    label="OFF" 
                    mode={TradingMode::Off}
                    selected={mode.get() == TradingMode::Off}
                    on_click={move |_| set_mode.set(TradingMode::Off)}
                    color_class="text-muted"
                />
            </div>
            
            // Action Buttons
            <div class="flex items-center gap-3">
                <button 
                    class="btn btn-primary px-6"
                    on:click=on_start
                    disabled={running.get()}
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"/>
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    "START"
                </button>
                
                <button 
                    class="btn btn-warning text-void px-6"
                    on:click=on_stop
                    disabled={!running.get()}
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z"/>
                    </svg>
                    "STOP"
                </button>
                
                <button 
                    class="btn btn-danger px-6"
                    on:click=on_kill
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                    </svg>
                    "KILL"
                </button>
                
                <button class="btn btn-ghost w-10 h-10 p-0">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                    </svg>
                </button>
            </div>
        </header>
        
        // Kill Confirmation Modal
        {move || if show_kill_confirm.get() {
            view! {
                <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
                    <div class="bg-card border border-danger rounded-lg p-6 max-w-md w-full mx-4 shadow-2xl">
                        <div class="flex items-center gap-3 mb-4">
                            <div class="w-10 h-10 rounded-full bg-danger/20 flex items-center justify-center">
                                <svg class="w-6 h-6 text-danger" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                                </svg>
                            </div>
                            <h3 class="text-lg font-semibold text-danger">"Emergency Stop"</h3>
                        </div>
                        
                        <p class="text-secondary mb-2">
                            "This will immediately:"
                        </p>
                        <ul class="text-sm text-secondary space-y-1 mb-6 list-disc list-inside">
                            <li>"Close ALL open positions at market price"</li>
                            <li>"Cancel ALL pending orders"</li>
                            <li>"Stop ALL trading activity"</li>
                        </ul>
                        
                        <div class="flex gap-3">
                            <button 
                                class="btn btn-danger flex-1"
                                on:click=confirm_kill
                            >
                                "YES, STOP EVERYTHING"
                            </button>
                            <button 
                                class="btn btn-secondary flex-1"
                                on:click={move |_| set_show_kill_confirm.set(false)}
                            >
                                "Cancel"
                            </button>
                        </div>
                    </div>
                </div>
            }.into_view()
        } else {
            view! {}.into_view()
        }}
    }
}

#[component]
fn ModeRadio(
    label: &'static str,
    mode: TradingMode,
    selected: bool,
    on_click: impl Fn(TradingMode) + 'static,
    color_class: &'static str,
) -> impl IntoView {
    let mode_clone = mode.clone();
    let class = move || {
        if selected {
            format!("mode-radio selected {}", match mode {
                TradingMode::Live => "live",
                TradingMode::Paper => "paper",
                TradingMode::Demo => "demo",
                TradingMode::Off => "",
            })
        } else {
            "mode-radio".to_string()
        }
    };
    
    view! {
        <div 
            class={class}
            on:click={move |_| on_click(mode_clone.clone())}
        >
            <span class={format!("status-dot {}", match mode {
                TradingMode::Live => "status-live",
                TradingMode::Paper => "status-paper",
                TradingMode::Demo => "status-demo",
                TradingMode::Off => "status-off",
            })}></span>
            <span class="text-xs font-medium">{label}</span>
        </div>
    }
}
