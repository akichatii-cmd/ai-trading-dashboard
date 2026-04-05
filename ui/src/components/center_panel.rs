use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::state::{DashboardState, Order, OrderSide, OrderStatus, Timeframe};
use crate::tauri_api;
use rust_decimal::Decimal;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn initTradingViewChart(container_id: &str, symbol: &str, timeframe: &str);
    
    #[wasm_bindgen(js_namespace = window)]
    fn updateChartPrice(price: f64);
    
    #[wasm_bindgen(js_namespace = window)]
    fn addPositionLines(position_id: &str, entry: f64, sl: Option<f64>, tp: Option<f64>, trail: Option<f64>);
    
    #[wasm_bindgen(js_namespace = window)]
    fn clearPriceLines();
    
    #[wasm_bindgen(js_namespace = window)]
    fn addPriceLine(line_type: &str, price: f64, position_id: &str, draggable: bool);
}

#[component]
pub fn CenterPanel() -> impl IntoView {
    view! {
        <main class="flex flex-col gap-4 h-full overflow-hidden">
            <ChartToolbar/>
            <Chart/>
            <IndicatorPanel/>
            <ActiveOrders/>
        </main>
    }
}

#[component]
fn ChartToolbar() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (selected_symbol, set_symbol) = create_slice(
        state,
        |state| state.selected_symbol.clone(),
        |state, v| state.selected_symbol = v,
    );
    
    let (timeframe, set_timeframe) = create_slice(
        state,
        |state| state.selected_timeframe.clone(),
        |state, v| state.selected_timeframe = v,
    );
    
    let symbols = vec!["SBER", "GAZP", "LKOH", "YNDX", "TCSG", "MOEX", "PLZL", "MGNT"];
    let timeframes = vec![
        Timeframe::M1, Timeframe::M5, Timeframe::M15, 
        Timeframe::M30, Timeframe::H1, Timeframe::H4, Timeframe::D1
    ];
    
    // Auto-trading toggle
    let (auto_trading, set_auto_trading) = create_signal(false);

    view! {
        <div class="flex items-center justify-between h-[40px] bg-card rounded-md px-4 border border-subtle">
            <div class="flex items-center gap-4">
                // Symbol selector
                <div class="relative">
                    <select 
                        class="bg-hover text-primary text-sm rounded px-3 py-1 border border-subtle focus:border-focus outline-none"
                        prop:value={move || selected_symbol.get()}
                        on:change={move |e| {
                            let value = event_target_value(&e);
                            set_symbol.set(value.clone());
                            spawn_local(async move {
                                let tf = timeframe.get().as_str();
                                initTradingViewChart("chart-container", &value, tf);
                            });
                        }}
                    >
                        {symbols.into_iter().map(|s| {
                            view! { <option value=s>{s}</option> }
                        }).collect_view()}
                    </select>
                </div>
                
                // Timeframe buttons
                <div class="flex items-center gap-1">
                    {timeframes.into_iter().map(|tf| {
                        let tf_clone = tf.clone();
                        let is_active = move || timeframe.get() == tf_clone;
                        view! {
                            <button 
                                class={move || if is_active() {
                                    "px-2 py-1 text-xs rounded bg-active text-primary border border-focus"
                                } else {
                                    "px-2 py-1 text-xs rounded bg-transparent text-secondary hover:text-primary"
                                }}
                                on:click={move |_| {
                                    set_timeframe.set(tf.clone());
                                    let sym = selected_symbol.get();
                                    initTradingViewChart("chart-container", &sym, tf.as_str());
                                }}
                            >
                                {tf.as_str().to_uppercase()}
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
            
            // Indicators toggles
            <div class="flex items-center gap-3">
                <button class="flex items-center gap-1 text-xs text-secondary hover:text-primary">
                    <span>"SMA"</span>
                    <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                    </svg>
                </button>
                <button class="flex items-center gap-1 text-xs text-secondary hover:text-primary">
                    <span>"BB"</span>
                    <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                    </svg>
                </button>
                <button class="flex items-center gap-1 text-xs text-secondary hover:text-primary">
                    <span>"VOL"</span>
                </button>
                <div class="w-px h-4 bg-subtle"></div>
                <button 
                    class={move || if auto_trading.get() {
                        "text-xs text-success hover:text-success font-semibold"
                    } else {
                        "text-xs text-secondary hover:text-primary"
                    }}
                    on:click={move |_| set_auto_trading.update(|v| *v = !*v)}
                >
                    {move || if auto_trading.get() { "⚡ Auto: ON" } else { "⚡ Auto" }}
                </button>
            </div>
        </div>
    }
}

#[component]
fn Chart() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (selected_symbol, _set_symbol) = create_slice(
        state,
        |state| state.selected_symbol.clone(),
        |state, v| state.selected_symbol = v,
    );
    
    let (timeframe, _set_timeframe) = create_slice(
        state,
        |state| state.selected_timeframe.clone(),
        |state, v| state.selected_timeframe = v,
    );
    
    let (positions, _set_positions) = create_slice(
        state,
        |state| state.positions.clone(),
        |state, v| state.positions = v,
    );
    
    // Current price (from WebSocket)
    let (current_price, set_current_price) = create_signal(Decimal::new(25045, 2));
    let (price_change, _set_price_change) = create_signal(Decimal::new(124, 2));
    
    let chart_ref = create_node_ref::<leptos::html::Div>();
    
    // Initialize TradingView chart on mount
    create_effect(move |_| {
        if let Some(_el) = chart_ref.get() {
            let sym = selected_symbol.get();
            let tf = timeframe.get().as_str();
            initTradingViewChart("chart-container", &sym, tf);
            
            // Setup price line moved event listener
            let window = web_sys::window().unwrap();
            let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
                if let Ok(custom_event) = e.dyn_into::<web_sys::CustomEvent>() {
                    if let Ok(detail) = custom_event.detail().dyn_into::<js_sys::Object>() {
                        // Get values from JS event
                        let type_ = js_sys::Reflect::get(&detail, &"type".into()).ok()
                            .and_then(|v| v.as_string()).unwrap_or_default();
                        let position_id = js_sys::Reflect::get(&detail, &"positionId".into()).ok()
                            .and_then(|v| v.as_string()).unwrap_or_default();
                        let new_price = js_sys::Reflect::get(&detail, &"newPrice".into()).ok()
                            .and_then(|v| v.as_f64()).unwrap_or(0.0);
                        
                        // Call Tauri API to modify position
                        spawn_local(async move {
                            let sl = if type_ == "sl" { Some(new_price) } else { None };
                            let tp = if type_ == "tp" { Some(new_price) } else { None };
                            let trail = if type_ == "trailing" { Some(new_price) } else { None };
                            
                            match tauri_api::modify_position_sl_tp(&position_id, sl, tp, trail).await {
                                Ok(_) => log::info!("Updated {} to {}", type_, new_price),
                                Err(e) => log::error!("Failed to update SL/TP: {}", e),
                            }
                        });
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            window.add_event_listener_with_callback("priceLineMoved", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }
    });
    
    // Update chart lines when positions change
    create_effect(move |_| {
        let pos_list = positions.get();
        
        // Find position for current symbol
        let current_symbol = selected_symbol.get();
        if let Some(pos) = pos_list.iter().find(|p| p.symbol == current_symbol) {
            let entry = pos.entry_price.to_f64().unwrap_or(0.0);
            let sl = pos.stop_loss.as_ref().and_then(|p| p.to_f64());
            let tp = pos.take_profit.as_ref().and_then(|p| p.to_f64());
            let trail = pos.trailing_stop.as_ref().and_then(|p| p.to_f64());
            
            addPositionLines(&pos.id, entry, sl, tp, trail);
        } else {
            clearPriceLines();
        }
    });

    view! {
        <div class="card flex-1 min-h-[400px] relative">
            <div _ref=chart_ref id="chart-container" class="w-full h-full">
                // Placeholder while TradingView loads
                <div class="w-full h-full flex items-center justify-center text-muted">
                    <div class="text-center">
                        <svg class="w-16 h-16 mx-auto mb-4 opacity-50 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"/>
                        </svg>
                        <p class="text-sm">"Loading TradingView Chart..."</p>
                        <p class="text-xs text-secondary mt-1">{selected_symbol}" / "{move || timeframe.get().as_str().to_uppercase()}</p>
                    </div>
                </div>
            </div>
            
            // Price overlay
            <div class="absolute top-4 left-4 bg-card/90 backdrop-blur border border-subtle rounded-md p-3">
                <div class="flex items-center gap-4">
                    <div>
                        <span class="text-xs text-muted">{selected_symbol}</span>
                        <div class="text-lg font-mono font-bold text-primary">
                            "₽"{move || format_decimal(current_price.get())}
                        </div>
                    </div>
                    <div class="text-right">
                        <span class="text-xs text-muted">"24h"</span>
                        <div class={move || if price_change.get() >= Decimal::ZERO { "text-sm font-mono text-success" } else { "text-sm font-mono text-danger" }}>
                            {move || if price_change.get() >= Decimal::ZERO { "+" } else { "" }}
                            {move || format_decimal(price_change.get())}"%"
                        </div>
                    </div>
                </div>
            </div>
            
            // Quick action buttons
            <div class="absolute top-4 right-4 flex gap-2">
                <button class="btn btn-success text-xs py-1 px-2" title="Buy Market">
                    "Buy"
                </button>
                <button class="btn btn-danger text-xs py-1 px-2" title="Sell Market">
                    "Sell"
                </button>
            </div>
            
            // SL/TP help tooltip
            <div class="absolute bottom-4 left-4 bg-card/90 backdrop-blur border border-subtle rounded-md p-2 text-xs text-muted">
                "Double-click SL/TP line to drag • Double-click again to confirm"
            </div>
        </div>
    }
}

#[component]
fn IndicatorPanel() -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 gap-4">
            // RSI
            <div class="bg-card rounded-md p-3 border border-subtle">
                <div class="flex items-center justify-between mb-2">
                    <span class="text-xs text-muted">"RSI(14)"</span>
                    <span class="text-xs text-info">"34 ○ Oversold"</span>
                </div>
                <div class="progress-bar h-1.5">
                    <div class="progress-fill bg-info" style="width: 34%"></div>
                </div>
                <div class="flex justify-between text-[10px] text-muted mt-1">
                    <span>"0"</span>
                    <span>"30"</span>
                    <span>"70"</span>
                    <span>"100"</span>
                </div>
            </div>
            
            // MACD
            <div class="bg-card rounded-md p-3 border border-subtle">
                <div class="flex items-center justify-between mb-2">
                    <span class="text-xs text-muted">"MACD"</span>
                    <span class="text-xs text-success">"↑ Bullish"</span>
                </div>
                <div class="progress-bar h-1.5">
                    <div class="progress-fill bg-success" style="width: 65%"></div>
                </div>
                <div class="text-[10px] text-muted mt-1 font-mono">
                    "MACD: 0.45 | Signal: 0.32 | Hist: 0.13"
                </div>
            </div>
        </div>
    }
}

#[component]
fn ActiveOrders() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (orders, _set_orders) = create_slice(
        state,
        |state| state.orders.clone(),
        |state, v| state.orders = v,
    );
    
    // New order modal
    let (show_new_order, set_show_new_order) = create_signal(false);

    view! {
        <div class="card h-[200px] flex flex-col">
            <div class="flex items-center justify-between mb-3">
                <h3 class="text-xs font-medium text-muted uppercase tracking-wider">"Active Orders"</h3>
                <button 
                    class="btn btn-primary text-xs py-1 px-3"
                    on:click={move |_| set_show_new_order.set(true)}
                >
                    "+ New Order"
                </button>
            </div>
            
            <div class="flex-1 overflow-y-auto -mx-4 px-4 space-y-2">
                <For
                    each={move || orders.get()}
                    key={|o| o.id.clone()}
                    children={|order| view! { <OrderCard order=order/> }}
                />
                
                {move || if orders.get().is_empty() {
                    view! {
                        <div class="text-center py-8 text-muted text-sm">
                            "No active orders"
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }}
            </div>
            
            // New Order Modal
            {move || if show_new_order.get() {
                view! { <NewOrderModal on_close={move || set_show_new_order.set(false)}/> }.into_view()
            } else {
                view! {}.into_view()
            }}
        </div>
    }
}

#[component]
fn OrderCard(order: Order) -> impl IntoView {
    let status_color = match order.status {
        OrderStatus::Working => "text-warning",
        OrderStatus::Filled => "text-success",
        OrderStatus::Cancelled => "text-danger",
        OrderStatus::Rejected => "text-danger",
        _ => "text-muted",
    };
    
    let side_color = match order.side {
        OrderSide::Buy => "text-success",
        OrderSide::Sell => "text-danger",
    };
    
    let fill_pct = if order.quantity > 0 {
        (order.filled_quantity as f64 / order.quantity as f64 * 100.0) as u32
    } else {
        0
    };

    view! {
        <div class="bg-hover rounded-md p-3 border border-subtle">
            <div class="flex items-center justify-between mb-2">
                <div class="flex items-center gap-2">
                    <span class={format!("text-xs font-semibold {}", side_color)}>
                        {match order.side {
                            OrderSide::Buy => "BUY",
                            OrderSide::Sell => "SELL",
                        }}
                    </span>
                    <span class="text-sm font-semibold text-primary">{order.symbol.clone()}</span>
                    <span class="text-xs text-secondary">{order.quantity}" lot"</span>
                </div>
                <span class={format!("text-xs font-medium {}", status_color)}>
                    {format!("{:?}", order.status).to_uppercase()}
                </span>
            </div>
            
            <div class="flex items-center justify-between text-xs text-secondary mb-2">
                <span>
                    "@ "{order.price.map(|p| format!("{:.2}", p)).unwrap_or_else(|| "MARKET".to_string())}
                    " ("{format!("{:?}", order.order_type).to_uppercase()}")"
                </span>
                <span>"Filled: "{order.filled_quantity}"/"{order.quantity}</span>
            </div>
            
            // Progress bar
            <div class="progress-bar mb-2">
                <div 
                    class={format!("progress-fill {}", if fill_pct >= 100 { "bg-success" } else { "bg-info" })} 
                    style={format!("width: {}%", fill_pct.min(100))}
                ></div>
            </div>
            
            <div class="flex gap-2">
                <button class="btn btn-secondary text-xs flex-1 py-1">
                    "MODIFY"
                </button>
                <button class="btn btn-danger text-xs flex-1 py-1">
                    "CANCEL"
                </button>
                <button class="btn btn-warning text-xs flex-1 py-1 text-void">
                    "→ MARKET"
                </button>
            </div>
        </div>
    }
}

#[component]
fn NewOrderModal(on_close: impl Fn() + 'static) -> impl IntoView {
    let (symbol, set_symbol) = create_signal("SBER".to_string());
    let (side, set_side) = create_signal(OrderSide::Buy);
    let (order_type, set_order_type) = create_signal("limit".to_string());
    let (quantity, set_quantity) = create_signal(1u32);
    let (price, set_price) = create_signal("".to_string());
    
    let submit = move |_| {
        let sym = symbol.get();
        let sd = match side.get() {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        };
        let ot = order_type.get();
        let qty = quantity.get();
        let prc = if price.get().is_empty() { None } else { Some(price.get()) };
        
        spawn_local(async move {
            match tauri_api::place_order(&sym, sd, &ot, qty, prc.as_deref()).await {
                Ok(msg) => {
                    log::info!("Order placed: {}", msg);
                    on_close();
                }
                Err(e) => {
                    log::error!("Failed to place order: {}", e);
                }
            }
        });
    };

    view! {
        <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
            <div class="bg-card border border-subtle rounded-lg p-6 max-w-md w-full mx-4 shadow-2xl">
                <h3 class="text-lg font-semibold text-primary mb-4">"New Order"</h3>
                
                <div class="space-y-4">
                    // Symbol
                    <div>
                        <label class="text-xs text-muted mb-1 block">"Symbol"</label>
                        <input 
                            type="text"
                            class="w-full bg-hover border border-subtle rounded px-3 py-2 text-primary"
                            prop:value={move || symbol.get()}
                            on:input={move |e| set_symbol.set(event_target_value(&e))}
                        />
                    </div>
                    
                    // Side
                    <div>
                        <label class="text-xs text-muted mb-1 block">"Side"</label>
                        <div class="flex gap-2">
                            <button 
                                class={move || if side.get() == OrderSide::Buy { "btn btn-success flex-1" } else { "btn btn-secondary flex-1" }}
                                on:click={move |_| set_side.set(OrderSide::Buy)}
                            >
                                "BUY"
                            </button>
                            <button 
                                class={move || if side.get() == OrderSide::Sell { "btn btn-danger flex-1" } else { "btn btn-secondary flex-1" }}
                                on:click={move |_| set_side.set(OrderSide::Sell)}
                            >
                                "SELL"
                            </button>
                        </div>
                    </div>
                    
                    // Order Type
                    <div>
                        <label class="text-xs text-muted mb-1 block">"Order Type"</label>
                        <select 
                            class="w-full bg-hover border border-subtle rounded px-3 py-2 text-primary"
                            on:change={move |e| set_order_type.set(event_target_value(&e))}
                        >
                            <option value="market">"Market"</option>
                            <option value="limit">"Limit"</option>
                            <option value="stop">"Stop"</option>
                        </select>
                    </div>
                    
                    // Quantity
                    <div>
                        <label class="text-xs text-muted mb-1 block">"Quantity (lots)"</label>
                        <input 
                            type="number"
                            class="w-full bg-hover border border-subtle rounded px-3 py-2 text-primary"
                            prop:value={move || quantity.get()}
                            on:input={move |e| {
                                if let Ok(v) = event_target_value(&e).parse::<u32>() {
                                    set_quantity.set(v);
                                }
                            }}
                        />
                    </div>
                    
                    // Price (for limit/stop)
                    {move || if order_type.get() != "market" {
                        view! {
                            <div>
                                <label class="text-xs text-muted mb-1 block">"Price"</label>
                                <input 
                                    type="text"
                                    class="w-full bg-hover border border-subtle rounded px-3 py-2 text-primary"
                                    prop:value={move || price.get()}
                                    on:input={move |e| set_price.set(event_target_value(&e))}
                                />
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>
                
                <div class="flex gap-3 mt-6">
                    <button 
                        class="btn btn-primary flex-1"
                        on:click=submit
                    >
                        "Place Order"
                    </button>
                    <button 
                        class="btn btn-secondary flex-1"
                        on:click={move |_| on_close()}
                    >
                        "Cancel"
                    </button>
                </div>
            </div>
        </div>
    }
}

fn format_decimal(d: Decimal) -> String {
    let s = d.to_string();
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() == 2 {
        format!("{}.{:.2}", parts[0], &parts[1][..parts[1].len().min(2)])
    } else {
        s
    }
}
