use leptos::*;
use crate::state::{DashboardState, LogEntry, LogLevel, TerminalTab};

#[component]
pub fn Footer() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (collapsed, set_collapsed) = create_slice(
        state,
        |state| state.terminal_collapsed,
        |state, v| state.terminal_collapsed = v,
    );
    
    let (active_tab, set_active_tab) = create_slice(
        state,
        |state| state.active_tab,
        |state, v| state.active_tab = v,
    );

    let height_class = move || {
        if collapsed.get() {
            "h-[40px]"
        } else {
            "h-[200px]"
        }
    };

    view! {
        <footer class={format!("terminal flex-shrink-0 transition-all duration-300 {}", height_class)}>
            // Tabs bar
            <div class="flex items-center justify-between h-[40px] border-b border-subtle px-4">
                <div class="flex items-center gap-1">
                    <TabButton 
                        label="LOGS" 
                        is_active={move || active_tab.get() == TerminalTab::Logs}
                        on_click={move |_| set_active_tab.set(TerminalTab::Logs)}
                    />
                    <TabButton 
                        label="TERMINAL" 
                        is_active={move || active_tab.get() == TerminalTab::Terminal}
                        on_click={move |_| set_active_tab.set(TerminalTab::Terminal)}
                    />
                    <TabButton 
                        label="ERRORS (2)" 
                        is_active={move || active_tab.get() == TerminalTab::Errors}
                        on_click={move |_| set_active_tab.set(TerminalTab::Errors)}
                        badge=Some("2")
                    />
                    <TabButton 
                        label="METRICS" 
                        is_active={move || active_tab.get() == TerminalTab::Metrics}
                        on_click={move |_| set_active_tab.set(TerminalTab::Metrics)}
                    />
                    <TabButton 
                        label="ORDERS" 
                        is_active={move || active_tab.get() == TerminalTab::Orders}
                        on_click={move |_| set_active_tab.set(TerminalTab::Orders)}
                    />
                    <TabButton 
                        label="PERFORMANCE" 
                        is_active={move || active_tab.get() == TerminalTab::Performance}
                        on_click={move |_| set_active_tab.set(TerminalTab::Performance)}
                    />
                </div>
                
                // Collapse button
                <button 
                    class="btn btn-ghost w-8 h-8 p-0"
                    on:click={move |_| set_collapsed.set(!collapsed.get())}
                >
                    <svg 
                        class={format!("w-4 h-4 transition-transform {}", if collapsed.get() { "rotate-180" } else { "" })} 
                        fill="none" 
                        stroke="currentColor" 
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                    </svg>
                </button>
            </div>
            
            // Content
            {move || if !collapsed.get() {
                view! {
                    <div class="h-[160px] overflow-hidden">
                        {match active_tab.get() {
                            TerminalTab::Logs => view! { <LogsTab/> },
                            TerminalTab::Terminal => view! { <TerminalTabView/> },
                            TerminalTab::Errors => view! { <ErrorsTab/> },
                            TerminalTab::Metrics => view! { <MetricsTab/> },
                            TerminalTab::Orders => view! { <OrdersHistoryTab/> },
                            TerminalTab::Performance => view! { <PerformanceTab/> },
                        }}
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
        </footer>
    }
}

#[component]
fn TabButton(
    label: &'static str,
    is_active: impl Fn() -> bool + 'static,
    on_click: impl Fn() + 'static,
    #[prop(optional)]
    badge: Option<&'static str>,
) -> impl IntoView {
    view! {
        <button 
            class={move || if is_active() {
                "terminal-tab active"
            } else {
                "terminal-tab"
            }}
            on:click={move |_| on_click()}
        >
            {label}
            {badge.map(|b| view! {
                <span class="ml-1 px-1.5 py-0.5 bg-danger text-white text-[10px] rounded-full">{b}</span>
            })}
        </button>
    }
}

#[component]
fn LogsTab() -> impl IntoView {
    let state = expect_context::<RwSignal<DashboardState>>();
    
    let (logs, _set_logs) = create_slice(
        state,
        |state| state.logs.clone(),
        |state, v| state.logs = v,
    );

    view! {
        <div class="h-full overflow-y-auto font-mono text-xs p-3 space-y-1">
            <For
                each={move || logs.get()}
                key={|log| format!("{}-{}", log.timestamp, log.message)}
                children={|log| view! { <LogLine log=log/> }}
            />
        </div>
    }
}

#[component]
fn LogLine(log: LogEntry) -> impl IntoView {
    let level_color = match log.level {
        LogLevel::Info => "text-info",
        LogLevel::Warn => "text-warning",
        LogLevel::Error => "text-danger",
        LogLevel::Debug => "text-muted",
    };
    
    let level_text = match log.level {
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Error => "ERROR",
        LogLevel::Debug => "DEBUG",
    };

    view! {
        <div class="flex items-start gap-3 hover:bg-hover rounded px-1">
            <span class="text-muted shrink-0">{log.timestamp}</span>
            <span class={format!("shrink-0 w-12 {}", level_color)}>{level_text}</span>
            <span class="text-secondary">{log.message}</span>
            <span class="text-muted shrink-0 ml-auto">[{log.source}]</span>
        </div>
    }
}

#[component]
fn TerminalTabView() -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (history, set_history) = create_signal(vec![
        "AI Trading Bot Terminal v5.0.0".to_string(),
        "Type 'help' for available commands".to_string(),
        "".to_string(),
    ]);
    
    let on_submit = move |e: leptos::ev::KeyboardEvent| {
        if e.key() == "Enter" {
            let cmd = input.get();
            if !cmd.is_empty() {
                set_history.update(|h| {
                    h.push(format!("> {}", cmd));
                    // Process command
                    match cmd.as_str() {
                        "help" => {
                            h.push("Available commands:".to_string());
                            h.push("  status     - Show current status".to_string());
                            h.push("  buy SYM Q  - Quick buy order".to_string());
                            h.push("  sell SYM Q - Quick sell order".to_string());
                            h.push("  close all  - Close all positions".to_string());
                            h.push("  set risk % - Change risk per trade".to_string());
                            h.push("  logs       - Show recent logs".to_string());
                            h.push("  clear      - Clear terminal".to_string());
                        }
                        "status" => {
                            h.push("Status: DEMO mode, Running".to_string());
                            h.push("Positions: 2 open, 0 pending".to_string());
                            h.push("Equity: ₽523,450".to_string());
                        }
                        "close all" => {
                            h.push("Closing all positions...".to_string());
                        }
                        "clear" => {
                            h.clear();
                        }
                        _ => {
                            h.push(format!("Unknown command: {}", cmd));
                        }
                    }
                });
                set_input.set(String::new());
            }
        }
    };

    view! {
        <div class="h-full flex flex-col">
            <div class="flex-1 overflow-y-auto font-mono text-xs p-3 space-y-1">
                <For
                    each={move || history.get()}
                    key={|line| line.clone()}
                    children={|line| {
                        if line.starts_with(">") {
                            view! { <div class="text-success">{line}</div> }.into_view()
                        } else if line.starts_with("  ") {
                            view! { <div class="text-muted pl-4">{line}</div> }.into_view()
                        } else {
                            view! { <div class="text-secondary">{line}</div> }.into_view()
                        }
                    }}
                />
            </div>
            <div class="h-[32px] border-t border-subtle flex items-center px-3 gap-2">
                <span class="text-success font-mono text-sm">{">"}</span>
                <input 
                    type="text"
                    class="flex-1 bg-transparent border-none outline-none text-primary font-mono text-sm"
                    placeholder="Type command..."
                    prop:value={move || input.get()}
                    on:input={move |e| set_input.set(event_target_value(&e))}
                    on:keydown=on_submit
                />
                <button class="text-xs text-muted hover:text-primary">"[help]"</button>
            </div>
        </div>
    }
}

#[component]
fn ErrorsTab() -> impl IntoView {
    view! {
        <div class="h-full overflow-y-auto p-3 space-y-2">
            <div class="bg-danger-dim border border-danger rounded-md p-3">
                <div class="flex items-start gap-2">
                    <span class="text-danger">"✖"</span>
                    <div>
                        <div class="text-sm text-danger font-medium">"Order failed: insufficient funds"</div>
                        <div class="text-xs text-secondary mt-1">
                            "Need ₽50,000, have ₽12 available"
                            <br/>"Time: 13:01:30"
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="bg-danger-dim border border-danger rounded-md p-3">
                <div class="flex items-start gap-2">
                    <span class="text-danger">"✖"</span>
                    <div>
                        <div class="text-sm text-danger font-medium">"WebSocket connection lost"</div>
                        <div class="text-xs text-secondary mt-1">
                            "Reconnected after 5.2s"
                            <br/>"Time: 12:45:12"
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MetricsTab() -> impl IntoView {
    view! {
        <div class="h-full p-4">
            <div class="grid grid-cols-4 gap-4">
                <MetricCard title="Latency" value="45ms" status="good"/>
                <MetricCard title="CPU" value="12%" status="good"/>
                <MetricCard title="Memory" value="234MB" status="good"/>
                <MetricCard title="API Calls" value="120/min" status="good"/>
            </div>
            
            <div class="mt-4 grid grid-cols-2 gap-4 text-xs">
                <div class="bg-hover rounded p-3">
                    <div class="text-muted mb-2">"WebSocket"</div>
                    <div class="text-success">"● Connected"</div>
                    <div class="text-secondary mt-1">"Messages: 1,240/sec"</div>
                </div>
                <div class="bg-hover rounded p-3">
                    <div class="text-muted mb-2">"Database"</div>
                    <div class="text-success">"● Healthy"</div>
                    <div class="text-secondary mt-1">"Queries: 45/sec"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MetricCard(title: &'static str, value: &'static str, status: &'static str) -> impl IntoView {
    let status_color = match status {
        "good" => "text-success",
        "warning" => "text-warning",
        "danger" => "text-danger",
        _ => "text-secondary",
    };
    
    view! {
        <div class="bg-hover rounded-md p-3 text-center">
            <div class="text-xs text-muted mb-1">{title}</div>
            <div class={format!("text-lg font-mono font-semibold {}", status_color)}>
                {value}
            </div>
        </div>
    }
}

#[component]
fn OrdersHistoryTab() -> impl IntoView {
    view! {
        <div class="h-full overflow-y-auto p-3">
            <table class="w-full text-xs">
                <thead class="text-muted border-b border-subtle">
                    <tr>
                        <th class="text-left py-2">"Time"</th>
                        <th class="text-left">"Symbol"</th>
                        <th class="text-left">"Side"</th>
                        <th class="text-right">"Qty"</th>
                        <th class="text-right">"Price"</th>
                        <th class="text-right">"Status"</th>
                    </tr>
                </thead>
                <tbody class="text-secondary">
                    <tr class="border-b border-subtle/50">
                        <td class="py-2">"13:04:02"</td>
                        <td>"SBER"</td>
                        <td class="text-success">"BUY"</td>
                        <td class="text-right">"10"</td>
                        <td class="text-right font-mono">"250.45"</td>
                        <td class="text-right text-success">"FILLED"</td>
                    </tr>
                    <tr class="border-b border-subtle/50">
                        <td class="py-2">"13:02:15"</td>
                        <td>"GAZP"</td>
                        <td class="text-danger">"SELL"</td>
                        <td class="text-right">"5"</td>
                        <td class="text-right font-mono">"165.20"</td>
                        <td class="text-right text-success">"FILLED"</td>
                    </tr>
                    <tr>
                        <td class="py-2">"12:58:42"</td>
                        <td>"LKOH"</td>
                        <td class="text-success">"BUY"</td>
                        <td class="text-right">"2"</td>
                        <td class="text-right font-mono">"6500.00"</td>
                        <td class="text-right text-danger">"CANCELLED"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn PerformanceTab() -> impl IntoView {
    view! {
        <div class="h-full p-4 flex items-center justify-center text-muted">
            <div class="text-center">
                <svg class="w-16 h-16 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                </svg>
                <p class="text-sm">"Performance chart"</p>
                <p class="text-xs mt-1">"Equity curve visualization"</p>
            </div>
        </div>
    }
}
