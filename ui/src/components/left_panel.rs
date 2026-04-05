use leptos::*;
use crate::state::{DashboardState, Position, OrderSide};
use crate::tauri_api;
use rust_decimal::Decimal;

#[component]
pub fn LeftPanel() -> impl IntoView {
    view! {
        <aside class="flex flex-col gap-4 h-full overflow-hidden">
            <BalanceCard/>
            <PositionsList/>
            <PnLCard/>
        </aside>
    }
}

#[component]
fn BalanceCard() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (equity, _set_equity) = create_slice(
        state,
        |state| state.portfolio.total_equity,
        |state, v| state.portfolio.total_equity = v,
    );
    
    let (daily_pnl, _set_daily_pnl) = create_slice(
        state,
        |state| state.portfolio.daily_pnl,
        |state, v| state.portfolio.daily_pnl = v,
    );
    
    let (drawdown, _set_drawdown) = create_slice(
        state,
        |state| state.portfolio.drawdown_pct,
        |state, v| state.portfolio.drawdown_pct = v,
    );
    
    let (peak_equity, _set_peak) = create_slice(
        state,
        |state| state.portfolio.peak_equity,
        |state, v| state.portfolio.peak_equity = v,
    );
    
    let pnl_class = move || {
        if daily_pnl.get() >= Decimal::ZERO {
            "text-success"
        } else {
            "text-danger"
        }
    };
    
    let pnl_sign = move || {
        if daily_pnl.get() >= Decimal::ZERO {
            "▲"
        } else {
            "▼"
        }
    };
    
    let drawdown_class = move || {
        let dd = drawdown.get();
        if dd < Decimal::from(5) {
            "bg-success"
        } else if dd < Decimal::from(10) {
            "bg-warning animate-pulse-warning"
        } else {
            "bg-danger animate-flash-danger"
        }
    };

    view! {
        <div class="card flex-shrink-0">
            <h3 class="text-xs font-medium text-muted uppercase tracking-wider mb-3">"Total Equity"</h3>
            
            <div class="text-2xl font-bold text-primary font-mono mb-2">
                "₽ "{move || format_decimal(equity.get())}
            </div>
            
            <div class={format!("text-sm font-medium {}", pnl_class())}>
                {pnl_sign}" "{move || format_decimal(daily_pnl.get().abs())}
                " ("{move || format_decimal(daily_pnl.get() / Decimal::from(100))}"%)"
            </div>
            
            <p class="text-xs text-muted mt-1">"Today"</p>
            
            <div class="mt-4">
                <div class="progress-bar">
                    <div 
                        class={format!("progress-fill {}", drawdown_class())}
                        style={move || format!("width: {}%", drawdown.get().min(Decimal::from(100)))}
                    ></div>
                </div>
                <div class="flex justify-between text-xs text-muted mt-2">
                    <span>"DD: "{move || format_decimal(drawdown.get())}"%"</span>
                    <span>"Peak: "{move || format_decimal(peak_equity.get())}</span>
                </div>
            </div>
            
            <div class="mt-3 pt-3 border-t border-subtle flex gap-2">
                <button class="btn btn-secondary text-xs flex-1 py-1">
                    "Refresh"
                </button>
                <button class="btn btn-secondary text-xs flex-1 py-1">
                    "History →"
                </button>
            </div>
        </div>
    }
}

#[component]
fn PositionsList() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (positions, _set_positions) = create_slice(
        state,
        |state| state.positions.clone(),
        |state, v| state.positions = v,
    );
    
    let positions_count = move || positions.get().len();

    view! {
        <div class="card flex-1 flex flex-col min-h-0">
            <div class="flex items-center justify-between mb-3">
                <h3 class="text-xs font-medium text-muted uppercase tracking-wider">"Open Positions"</h3>
                <span class="text-xs text-muted">{positions_count}"/3"</span>
            </div>
            
            <div class="flex-1 overflow-y-auto -mx-4 px-4 space-y-3">
                <For
                    each={move || positions.get()}
                    key={|p| p.id.clone()}
                    children={|position| view! { <PositionCard position=position/> }}
                />
                
                {move || if positions.get().is_empty() {
                    view! {
                        <div class="text-center py-8 text-muted text-sm">
                            "No open positions"
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }}
            </div>
        </div>
    }
}

#[component]
fn PositionCard(position: Position) -> impl IntoView {
    let pnl_class = if position.unrealized_pnl >= Decimal::ZERO {
        "profit"
    } else {
        "loss"
    };
    
    let side_color = match position.side {
        OrderSide::Buy => "text-success",
        OrderSide::Sell => "text-danger",
    };
    
    let side_icon = match position.side {
        OrderSide::Buy => "🟢",
        OrderSide::Sell => "🔴",
    };
    
    let position_id = position.id.clone();
    let (show_trail, set_show_trail) = create_signal(false);
    
    let on_close = move |_| {
        let id = position_id.clone();
        spawn_local(async move {
            match tauri_api::close_position(&id).await {
                Ok(msg) => {
                    log::info!("Position closed: {}", msg);
                }
                Err(e) => {
                    log::error!("Failed to close position: {}", e);
                }
            }
        });
    };

    view! {
        <div class={format!("position-card rounded-md {}", pnl_class)}>
            <div class="flex items-center justify-between mb-2">
                <div class="flex items-center gap-2">
                    <span>{side_icon}</span>
                    <span class="font-semibold text-primary">{position.symbol.clone()}</span>
                    <span class={format!("text-xs {}", side_color)}>
                        {match position.side {
                            OrderSide::Buy => "LONG",
                            OrderSide::Sell => "SHORT",
                        }}
                    </span>
                </div>
                <span class={format!("font-mono font-semibold {}", if position.unrealized_pnl >= Decimal::ZERO { "text-success" } else { "text-danger" })}>
                    {if position.unrealized_pnl >= Decimal::ZERO { "+" } else { "" }}
                    "₽"{format_decimal(position.unrealized_pnl)}
                </span>
            </div>
            
            <div class="grid grid-cols-2 gap-2 text-xs text-secondary mb-3">
                <div>
                    <span class="text-muted">"Qty: "</span>
                    <span class="text-primary">{position.quantity}</span>
                </div>
                <div>
                    <span class="text-muted">"P&L: "</span>
                    <span class={if position.unrealized_pnl_pct >= Decimal::ZERO { "text-success" } else { "text-danger" }}>
                        {format_decimal(position.unrealized_pnl_pct)}"%"
                    </span>
                </div>
                <div>
                    <span class="text-muted">"Entry: "</span>
                    <span class="font-mono">{format_decimal(position.entry_price)}</span>
                </div>
                <div>
                    <span class="text-muted">"Current: "</span>
                    <span class="font-mono">{format_decimal(position.current_price)}</span>
                </div>
            </div>
            
            // SL/TP Bars
            <div class="space-y-2">
                {position.stop_loss.map(|sl| view! {
                    <div class="flex items-center gap-2">
                        <span class="text-xs text-danger w-6">"SL"</span>
                        <div class="flex-1 h-1 bg-hover rounded-full overflow-hidden">
                            <div class="h-full bg-danger w-1/3"></div>
                        </div>
                        <span class="text-xs font-mono text-secondary w-16 text-right">{format_decimal(sl)}</span>
                    </div>
                })}
                
                {position.take_profit.map(|tp| view! {
                    <div class="flex items-center gap-2">
                        <span class="text-xs text-success w-6">"TP"</span>
                        <div class="flex-1 h-1 bg-hover rounded-full overflow-hidden">
                            <div class="h-full bg-success w-2/3"></div>
                        </div>
                        <span class="text-xs font-mono text-secondary w-16 text-right">{format_decimal(tp)}</span>
                    </div>
                })}
                
                {position.trailing_stop.map(|ts| view! {
                    <div class="flex items-center gap-2">
                        <span class="text-xs text-info w-6">"TRAIL"</span>
                        <div class="flex-1 h-1 bg-hover rounded-full overflow-hidden">
                            <div class="h-full bg-info w-1/2"></div>
                        </div>
                        <span class="text-xs font-mono text-secondary w-16 text-right">{format_decimal(ts)}</span>
                    </div>
                })}
            </div>
            
            // Trailing Stop Controls
            {move || if show_trail.get() {
                view! {
                    <div class="mt-3 p-2 bg-hover rounded-md">
                        <div class="flex items-center justify-between mb-2">
                            <span class="text-xs text-muted">"Trailing Stop %"</span>
                            <span class="text-xs font-mono text-info">"2.0%"</span>
                        </div>
                        <input 
                            type="range" 
                            class="range-slider"
                            min="0.5"
                            max="5.0"
                            step="0.5"
                            value="2.0"
                        />
                        <div class="flex gap-2 mt-2">
                            <button class="btn btn-primary text-xs flex-1 py-1">
                                "Activate"
                            </button>
                            <button 
                                class="btn btn-ghost text-xs py-1"
                                on:click={move |_| set_show_trail.set(false)}
                            >
                                "Cancel"
                            </button>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
            
            <div class="flex gap-2 mt-3">
                <button 
                    class="btn btn-danger text-xs flex-1 py-1"
                    on:click=on_close
                >
                    "CLOSE"
                </button>
                <button 
                    class="btn btn-secondary text-xs flex-1 py-1"
                    on:click={move |_| set_show_trail.set(true)}
                >
                    "TRAIL"
                </button>
            </div>
        </div>
    }
}

#[component]
fn PnLCard() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (daily_pnl, _set_daily_pnl) = create_slice(
        state,
        |state| state.portfolio.daily_pnl,
        |state, v| state.portfolio.daily_pnl = v,
    );
    
    // Calculate realized PnL (mock for now, should come from state)
    let realized_pnl = Decimal::new(2100, 2); // ₽2,100
    let fees = Decimal::new(45, 2); // ₽45
    let net_pnl = move || realized_pnl + daily_pnl.get() - fees;

    view! {
        <div class="card flex-shrink-0">
            <h3 class="text-xs font-medium text-muted uppercase tracking-wider mb-3">"Today's P&L"</h3>
            
            <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                    <span class="text-secondary">"Realized"</span>
                    <span class="text-success font-mono">"+₽"{format_decimal(realized_pnl)}</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-secondary">"Unrealized"</span>
                    <span class={if daily_pnl.get() >= Decimal::ZERO { "text-success" } else { "text-danger" }}>
                        {if daily_pnl.get() >= Decimal::ZERO { "+" } else { "" }}
                        "₽"{move || format_decimal(daily_pnl.get())}
                    </span>
                </div>
                <div class="flex justify-between">
                    <span class="text-secondary">"Fees"</span>
                    <span class="text-danger font-mono">"-₽"{format_decimal(fees)}</span>
                </div>
                <div class="border-t border-subtle pt-2 mt-2">
                    <div class="flex justify-between font-semibold">
                        <span class="text-primary">"NET"</span>
                        <span class={move || if net_pnl() >= Decimal::ZERO { "text-success" } else { "text-danger" }}>
                            {move || if net_pnl() >= Decimal::ZERO { "+" } else { "" }}
                            "₽"{move || format_decimal(net_pnl())}
                        </span>
                    </div>
                </div>
            </div>
            
            <div class="mt-4 pt-3 border-t border-subtle">
                <div class="flex justify-between text-xs text-muted mb-2">
                    <span>"Trades: 3"</span>
                    <span>"Win rate: 67%"</span>
                </div>
                <div class="flex justify-between text-xs text-muted">
                    <span>"Avg win: ₽850"</span>
                    <span>"Avg loss: -₽120"</span>
                </div>
            </div>
            
            <div class="mt-3 pt-3 border-t border-subtle flex gap-2">
                <button class="btn btn-secondary text-xs flex-1 py-1">
                    "Details"
                </button>
                <button class="btn btn-secondary text-xs flex-1 py-1">
                    "Export CSV"
                </button>
            </div>
        </div>
    }
}

pub fn format_decimal(d: Decimal) -> String {
    let s = d.to_string();
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() == 2 {
        format!("{}.{:.2}", parts[0], &parts[1][..parts[1].len().min(2)])
    } else {
        s
    }
}
