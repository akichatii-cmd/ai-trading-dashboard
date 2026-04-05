use leptos::*;
use crate::state::{DashboardState, TradeSignal, SignalAction, OrderSide, RiskStatus, RiskSystemStatus};
use rust_decimal::Decimal;

#[component]
pub fn RightPanel() -> impl IntoView {
    view! {
        <aside class="flex flex-col gap-4 h-full overflow-hidden">
            <SignalCard/>
            <MarketCondition/>
            <RiskDashboard/>
            <IndicatorsMini/>
        </aside>
    }
}

#[component]
fn SignalCard() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (signals, _set_signals) = create_slice(
        state,
        |state| state.signals.clone(),
        |state, v| state.signals = v,
    );
    
    let latest_signal = move || {
        signals.get().last().cloned()
    };
    
    let signal_class = move || {
        if let Some(_sig) = latest_signal() {
            "signal-card active"
        } else {
            "signal-card"
        }
    };
    
    let (executing, set_executing) = create_signal(false);

    view! {
        <div class={signal_class}>
            <div class="text-xs text-muted uppercase tracking-wider mb-2">"🎯 Signal"</div>
            
            {move || latest_signal().map(|sig| {
                let action_color = match sig.action {
                    SignalAction::Buy => "text-success",
                    SignalAction::Sell => "text-danger",
                    SignalAction::Hold => "text-muted",
                };
                
                let action_text = match sig.action {
                    SignalAction::Buy => "BUY ▲",
                    SignalAction::Sell => "SELL ▼",
                    SignalAction::Hold => "HOLD",
                };
                
                let signal_id = sig.symbol.clone();
                let is_executing = executing.get();
                
                view! {
                    <div class="animate-fade-in">
                        <div class="text-4xl font-bold text-primary mb-2">
                            {sig.symbol.clone()}
                        </div>
                        
                        <div class={format!("text-2xl font-bold {} mb-4", action_color)}>
                            {action_text}
                        </div>
                        
                        // Confidence bar
                        <div class="mb-4">
                            <div class="text-xs text-muted mb-1">
                                "Confidence: "{sig.confidence}"%"
                            </div>
                            <div class="progress-bar h-2">
                                <div 
                                    class={format!("progress-fill {}", if sig.confidence >= 70.0 { "bg-success" } else if sig.confidence >= 50.0 { "bg-warning" } else { "bg-danger" })} 
                                    style={format!("width: {}%", sig.confidence)}
                                ></div>
                            </div>
                        </div>
                        
                        // Signal info
                        <div class="text-xs text-secondary mb-4">
                            "Strategy: "{sig.strategy.clone()}
                            <br/>"Timeframe: "{sig.timeframe.clone()}
                        </div>
                        
                        // Levels
                        <div class="border-t border-subtle pt-4 mb-4">
                            <div class="grid grid-cols-2 gap-3 text-sm">
                                <div class="text-left">
                                    <span class="text-muted text-xs">"Entry"</span>
                                    <div class="font-mono text-primary">{format_decimal(sig.price)}</div>
                                </div>
                                <div class="text-right">
                                    <span class="text-muted text-xs">"Stop Loss"</span>
                                    <div class="font-mono text-danger">{format_decimal(sig.stop_loss)}</div>
                                </div>
                                <div class="text-left">
                                    <span class="text-muted text-xs">"Take Profit"</span>
                                    <div class="font-mono text-success">{format_decimal(sig.take_profit)}</div>
                                </div>
                                <div class="text-right">
                                    <span class="text-muted text-xs">"R:R"</span>
                                    <div class="font-mono text-info">
                                        "1:"{format_decimal((sig.take_profit - sig.price) / (sig.price - sig.stop_loss))}
                                    </div>
                                </div>
                            </div>
                        </div>
                        
                        // Execute button
                        <button 
                            class={if is_executing { "btn btn-primary w-full py-3 text-lg font-semibold opacity-50" } else { "btn btn-primary w-full py-3 text-lg font-semibold" }}
                            on:click={move |_| {
                                set_executing.set(true);
                                // TODO: Call execute_signal API
                                set_timeout(move || set_executing.set(false), std::time::Duration::from_secs(2));
                            }}
                            disabled={is_executing}
                        >
                            {if is_executing { 
                                view! { <span class="animate-pulse">"Executing..."</span> }.into_view()
                            } else { 
                                "▶ EXECUTE BUY NOW".into_view() 
                            }}
                        </button>
                        
                        // Ignore / Auto
                        <div class="flex justify-between mt-3">
                            <button class="btn btn-ghost text-xs">
                                "IGNORE"
                            </button>
                            <div class="flex items-center gap-2">
                                <span class="text-xs text-muted">"AUTO:"</span>
                                <button class="btn btn-secondary text-xs py-1 px-2">
                                    "ON"
                                </button>
                            </div>
                        </div>
                    </div>
                }
            }).unwrap_or_else(|| {
                view! {
                    <div class="text-center py-8 text-muted">
                        <svg class="w-12 h-12 mx-auto mb-3 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                        </svg>
                        <p class="text-sm">"No active signals"</p>
                        <p class="text-xs mt-1">"Waiting for market conditions..."</p>
                    </div>
                }.into_view()
            })}
        </div>
    }
}

#[component]
fn MarketCondition() -> impl IntoView {
    view! {
        <div class="card">
            <h3 class="text-xs font-medium text-muted uppercase tracking-wider mb-3">"Market Condition"</h3>
            
            <div class="space-y-3">
                // Volatility
                <div>
                    <div class="flex justify-between text-xs mb-1">
                        <span class="text-secondary">"Volatility"</span>
                        <span class="text-success">"1.8% ✓ OK"</span>
                    </div>
                    <div class="progress-bar">
                        <div class="progress-fill bg-success" style="width: 18%"></div>
                    </div>
                </div>
                
                // Trend Strength
                <div>
                    <div class="flex justify-between text-xs mb-1">
                        <span class="text-secondary">"Trend Strength"</span>
                        <span class="text-success">"Strong Up (ADX: 45)"</span>
                    </div>
                    <div class="progress-bar">
                        <div class="progress-fill bg-success" style="width: 45%"></div>
                    </div>
                </div>
                
                // Volume
                <div>
                    <div class="flex justify-between text-xs mb-1">
                        <span class="text-secondary">"Volume"</span>
                        <span class="text-success">"Above avg (+40%)"</span>
                    </div>
                    <div class="progress-bar">
                        <div class="progress-fill bg-info" style="width: 70%"></div>
                    </div>
                </div>
            </div>
            
            // Correlation alert
            <div class="mt-3 pt-3 border-t border-subtle">
                <div class="flex items-start gap-2 text-xs">
                    <span class="text-warning">"⚠️"</span>
                    <div class="text-secondary">
                        <span class="text-warning">"Correlation alert:"</span>
                        <br/>"GAZP/LKOH: 0.82"
                        <br/>"Avoid simultaneous positions"
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn RiskDashboard() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    // Mock risk data - should come from state
    let daily_loss_pct = 0.5; // 0.5%
    let daily_loss_limit = 2.0; // 2.0%
    let positions_used = 2;
    let positions_limit = 3;
    let drawdown_pct = 3.0; // 3%
    let drawdown_limit = 10.0; // 10%
    let exposure_used = 150000.0; // ₽150k
    let exposure_limit = 500000.0; // ₽500k
    
    // Calculate percentages
    let daily_loss_pct_of_limit = (daily_loss_pct / daily_loss_limit) * 100.0;
    let positions_pct = (positions_used as f64 / positions_limit as f64) * 100.0;
    let drawdown_pct_of_limit = (drawdown_pct / drawdown_limit) * 100.0;
    let exposure_pct = (exposure_used / exposure_limit) * 100.0;
    
    // Status based on percentages
    let overall_status = if daily_loss_pct_of_limit > 80.0 || drawdown_pct_of_limit > 80.0 {
        RiskSystemStatus::LimitApproaching
    } else if daily_loss_pct_of_limit > 50.0 || drawdown_pct_of_limit > 50.0 {
        RiskSystemStatus::Caution
    } else {
        RiskSystemStatus::AllSystemsGo
    };

    view! {
        <div class="card">
            <h3 class="text-xs font-medium text-muted uppercase tracking-wider mb-3">"Risk Limits"</h3>
            
            <div class="space-y-3">
                // Daily Loss
                <RiskBar 
                    label="Daily Loss"
                    current={daily_loss_pct}
                    limit={daily_loss_limit}
                    percent={daily_loss_pct_of_limit}
                    warning_threshold={50.0}
                    critical_threshold={80.0}
                    format="{:.1}%"
                />
                
                // Max Positions
                <RiskBar 
                    label="Max Positions"
                    current={positions_used as f64}
                    limit={positions_limit as f64}
                    percent={positions_pct}
                    warning_threshold={66.0}
                    critical_threshold={90.0}
                    format="{:.0}/{:.0}"
                />
                
                // Drawdown
                <RiskBar 
                    label="Drawdown"
                    current={drawdown_pct}
                    limit={drawdown_limit}
                    percent={drawdown_pct_of_limit}
                    warning_threshold={50.0}
                    critical_threshold={80.0}
                    format="{:.1}%"
                />
                
                // Exposure
                <RiskBar 
                    label="Exposure"
                    current={exposure_used}
                    limit={exposure_limit}
                    percent={exposure_pct}
                    warning_threshold={70.0}
                    critical_threshold={90.0}
                    format="₽{:.0}k"
                    value_divisor={1000.0}
                />
            </div>
            
            // Overall status
            <div class="mt-3 pt-3 border-t border-subtle flex items-center gap-2">
                {move || match overall_status {
                    RiskSystemStatus::AllSystemsGo => view! {
                        <>
                            <span class="status-dot status-live"></span>
                            <span class="text-xs text-success font-medium">"ALL SYSTEMS GO"</span>
                        </>
                    },
                    RiskSystemStatus::Caution => view! {
                        <>
                            <span class="status-dot status-paper"></span>
                            <span class="text-xs text-warning font-medium animate-pulse">"CAUTION"</span>
                        </>
                    },
                    RiskSystemStatus::LimitApproaching => view! {
                        <>
                            <span class="status-dot animate-flash-danger"></span>
                            <span class="text-xs text-danger font-medium animate-pulse">"LIMIT APPROACHING"</span>
                        </>
                    },
                    RiskSystemStatus::TradingLocked => view! {
                        <>
                            <span class="status-dot status-off"></span>
                            <span class="text-xs text-muted font-medium">"TRADING LOCKED"</span>
                        </>
                    },
                }}
            </div>
        </div>
    }
}

#[component]
fn RiskBar(
    label: &'static str,
    current: f64,
    limit: f64,
    percent: f64,
    warning_threshold: f64,
    critical_threshold: f64,
    format: &'static str,
    #[prop(default = 1.0)]
    value_divisor: f64,
) -> impl IntoView {
    // Determine color based on percentage
    let (bar_class, text_class) = if percent >= critical_threshold {
        ("bg-danger animate-flash-danger", "text-danger")
    } else if percent >= warning_threshold {
        ("bg-warning animate-pulse-warning", "text-warning")
    } else {
        ("bg-success", "text-success")
    };
    
    // Format values
    let current_str = if format.contains("k") {
        format!("{:.0}k", current / value_divisor)
    } else {
        format!(format, current)
    };
    
    let limit_str = if format.contains("k") {
        format!("{:.0}k", limit / value_divisor)
    } else if format.contains("/") {
        format!("{:.0}", limit)
    } else {
        format!(format, limit)
    };

    view! {
        <div>
            <div class="flex justify-between text-xs mb-1">
                <span class="text-secondary">{label}</span>
                <span class={text_class}>
                    {current_str}" / "{limit_str}
                </span>
            </div>
            <div class="progress-bar h-1.5">
                <div 
                    class={format!("progress-fill {}", bar_class)}
                    style={format!("width: {}%", percent.min(100.0))}
                ></div>
            </div>
        </div>
    }
}

#[component]
fn IndicatorsMini() -> impl IntoView {
    view! {
        <div class="card text-xs">
            <div class="grid grid-cols-2 gap-x-4 gap-y-2">
                <div class="flex justify-between">
                    <span class="text-muted">"RSI(14)"</span>
                    <span class="text-info">"34 ○ Oversold"</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-muted">"MACD"</span>
                    <span class="text-success">"↑ Bullish"</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-muted">"SMA20"</span>
                    <span class="font-mono">"101.50"</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-muted">"SMA50"</span>
                    <span class="font-mono text-success">"98.20 ▲"</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-muted">"ATR(14)"</span>
                    <span class="font-mono">"1.8%"</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-muted">"BB"</span>
                    <span class="text-secondary">"Lower 20%"</span>
                </div>
                <div class="flex justify-between col-span-2">
                    <span class="text-muted">"Volume"</span>
                    <span class="text-info">"1.2M ↑ +40%"</span>
                </div>
            </div>
            
            <button class="w-full mt-3 text-center text-muted hover:text-primary text-xs">
                "[Expand full]"
            </button>
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
