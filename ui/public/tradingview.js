// TradingView Lightweight Charts Integration with WebSocket
// Real-time data feed from Rust backend (ws://localhost:8081)

let chart = null;
let candlestickSeries = null;
let volumeSeries = null;
let priceLines = [];
let wsConnection = null;
let currentSymbol = 'SBER';
let currentTimeframe = '5m';

// Price history cache
const priceHistory = new Map();

// Initialize the TradingView chart with WebSocket
function initTradingViewChart(containerId, symbol, timeframe) {
    currentSymbol = symbol;
    currentTimeframe = timeframe;
    
    const container = document.getElementById(containerId);
    if (!container) {
        console.error('Chart container not found:', containerId);
        return;
    }
    
    // Clear placeholder content
    container.innerHTML = '';
    
    // Check if LightweightCharts is available
    if (typeof LightweightCharts === 'undefined') {
        console.error('LightweightCharts library not loaded');
        showError(container, 'Chart library not available');
        return;
    }
    
    // Create chart
    chart = LightweightCharts.createChart(container, {
        width: container.clientWidth,
        height: container.clientHeight,
        layout: {
            background: { color: '#141419' },
            textColor: '#8a8a9a',
        },
        grid: {
            vertLines: { color: '#2a2a35' },
            horzLines: { color: '#2a2a35' },
        },
        crosshair: {
            mode: LightweightCharts.CrosshairMode.Normal,
        },
        rightPriceScale: {
            borderColor: '#2a2a35',
        },
        timeScale: {
            borderColor: '#2a2a35',
            timeVisible: true,
            secondsVisible: false,
        },
        handleScroll: {
            vertTouchDrag: false,
        },
    });
    
    // Add candlestick series
    candlestickSeries = chart.addCandlestickSeries({
        upColor: '#00ff88',
        downColor: '#ff3366',
        borderUpColor: '#00ff88',
        borderDownColor: '#ff3366',
        wickUpColor: '#00ff88',
        wickDownColor: '#ff3366',
    });
    
    // Load initial data
    loadHistoricalData(symbol, timeframe);
    
    // Add volume histogram
    volumeSeries = chart.addHistogramSeries({
        color: '#00ccff',
        priceFormat: { type: 'volume' },
        priceScaleId: '',
        scaleMargins: {
            top: 0.8,
            bottom: 0,
        },
    });
    
    // Fit content
    chart.timeScale().fitContent();
    
    // Handle resize
    const resizeObserver = new ResizeObserver(entries => {
        for (let entry of entries) {
            if (chart) {
                chart.applyOptions({
                    width: entry.contentRect.width,
                    height: entry.contentRect.height,
                });
            }
        }
    });
    resizeObserver.observe(container);
    
    // Add click handler for context menu
    chart.subscribeClick((param) => {
        if (param.point && param.time && candlestickSeries) {
            const price = candlestickSeries.coordinateToPrice(param.point.y);
            console.log('Chart clicked at:', param.time, price);
            
            // Dispatch custom event for Rust to handle
            window.dispatchEvent(new CustomEvent('chartClick', {
                detail: { 
                    time: param.time, 
                    price: price,
                    symbol: currentSymbol 
                }
            }));
        }
    });
    
    // Connect to WebSocket for real-time updates
    connectWebSocket(symbol);
    
    console.log('TradingView chart initialized for', symbol, timeframe);
}

// Load historical data from backend
async function loadHistoricalData(symbol, timeframe) {
    try {
        // Try to get historical data from Tauri invoke
        if (window.__TAURI__) {
            const data = await window.__TAURI__.invoke('get_historical_data', {
                symbol: symbol,
                timeframe: timeframe,
                limit: 200,
                from: null,
                to: null,
            });
            
            if (data && data.length > 0) {
                const candles = data.map(d => ({
                    time: d.time,
                    open: d.open,
                    high: d.high,
                    low: d.low,
                    close: d.close,
                }));
                
                candlestickSeries.setData(candles);
                
                // Set volume data if available
                const volumeData = data.map(d => ({
                    time: d.time,
                    value: d.volume || 0,
                    color: d.close >= d.open ? '#00ff88' : '#ff3366',
                }));
                volumeSeries.setData(volumeData);
            } else {
                // Fallback to mock data if no historical data
                useMockData(symbol, timeframe);
            }
        } else {
            // Fallback to mock data if Tauri not available
            useMockData(symbol, timeframe);
        }
    } catch (e) {
        console.warn('Failed to load historical data:', e);
        useMockData(symbol, timeframe);
    }
}

// Connect to WebSocket for real-time price updates
function connectWebSocket(symbol) {
    // Close existing connection
    if (wsConnection) {
        wsConnection.close();
    }
    
    const wsUrl = 'ws://localhost:8081';
    wsConnection = new WebSocket(wsUrl);
    
    wsConnection.onopen = () => {
        console.log('Chart WebSocket connected');
        
        // Subscribe to price updates
        const subscribeMsg = JSON.stringify({
            type: 'subscribe',
            data: {
                channels: ['prices']
            }
        });
        wsConnection.send(subscribeMsg);
    };
    
    wsConnection.onmessage = (event) => {
        try {
            const msg = JSON.parse(event.data);
            
            if (msg.type === 'price.update' && msg.data) {
                const { symbol: updateSymbol, price, ts } = msg.data;
                
                // Only update if it's our current symbol
                if (updateSymbol === symbol) {
                    updateRealtimePrice(price, ts);
                }
            }
        } catch (e) {
            console.error('Failed to parse WebSocket message:', e);
        }
    };
    
    wsConnection.onerror = (error) => {
        console.error('Chart WebSocket error:', error);
    };
    
    wsConnection.onclose = () => {
        console.log('Chart WebSocket closed, reconnecting...');
        setTimeout(() => connectWebSocket(symbol), 5000);
    };
}

// Update chart with real-time price
function updateRealtimePrice(price, timestamp) {
    if (!candlestickSeries) return;
    
    const time = Math.floor(timestamp / 1000); // Convert to seconds
    
    // Get current candle data
    const data = candlestickSeries.data();
    if (data.length === 0) return;
    
    const lastCandle = data[data.length - 1];
    const currentTime = Math.floor(Date.now() / 1000);
    const timeframeSeconds = getTimeframeSeconds(currentTimeframe);
    
    // Check if we need a new candle
    if (currentTime - lastCandle.time >= timeframeSeconds) {
        // Create new candle
        const newCandle = {
            time: lastCandle.time + timeframeSeconds,
            open: lastCandle.close,
            high: price,
            low: price,
            close: price,
        };
        candlestickSeries.update(newCandle);
    } else {
        // Update current candle
        lastCandle.close = price;
        if (price > lastCandle.high) lastCandle.high = price;
        if (price < lastCandle.low) lastCandle.low = price;
        candlestickSeries.update(lastCandle);
    }
}

// Get timeframe in seconds
function getTimeframeSeconds(tf) {
    const map = {
        '1m': 60,
        '5m': 300,
        '15m': 900,
        '30m': 1800,
        '1h': 3600,
        '4h': 14400,
        '1d': 86400,
    };
    return map[tf] || 300;
}

// ============================================================================
// SL/TP Price Lines with Drag Support
// ============================================================================

// Add a draggable price line (SL/TP)
function addPriceLine(type, price, positionId, draggable = true) {
    if (!candlestickSeries) return null;
    
    const colors = {
        'sl': '#ff3366',
        'tp': '#00ff88',
        'entry': '#00ccff',
        'trailing': '#ffcc00'
    };
    
    const titles = {
        'sl': 'Stop Loss',
        'tp': 'Take Profit',
        'entry': 'Entry',
        'trailing': 'Trailing Stop'
    };
    
    const line = candlestickSeries.createPriceLine({
        price: price,
        color: colors[type] || '#8a8a9a',
        lineWidth: 2,
        lineStyle: LightweightCharts.LineStyle.Dashed,
        axisLabelVisible: true,
        title: `${titles[type]} ${price.toFixed(2)}`,
    });
    
    // Store metadata
    line._type = type;
    line._positionId = positionId;
    line._draggable = draggable;
    
    priceLines.push(line);
    
    // Add drag support
    if (draggable) {
        setupPriceLineDrag(line);
    }
    
    return line;
}

// Setup drag functionality for a price line
function setupPriceLineDrag(priceLine) {
    // Note: Lightweight Charts doesn't natively support dragging price lines
    // We implement a workaround using crosshair and click events
    
    let isDragging = false;
    let startY = 0;
    let startPrice = priceLine.options().price;
    
    // Subscribe to crosshair move to track position
    chart.subscribeCrosshairMove((param) => {
        if (!isDragging || !param.point) return;
        
        const price = candlestickSeries.coordinateToPrice(param.point.y);
        if (price !== null) {
            priceLine.applyOptions({
                price: price,
                title: `${priceLine._type.toUpperCase()} ${price.toFixed(2)} (dragging...)`
            });
        }
    });
    
    // Double-click to start/stop dragging
    chart.subscribeClick((param) => {
        if (!param.point) return;
        
        const clickPrice = candlestickSeries.coordinateToPrice(param.point.y);
        const linePrice = priceLine.options().price;
        
        // Check if click is near the price line (within 0.5%)
        const tolerance = linePrice * 0.005;
        if (Math.abs(clickPrice - linePrice) < tolerance) {
            if (!isDragging) {
                // Start dragging
                isDragging = true;
                startPrice = linePrice;
                chart.applyOptions({
                    handleScroll: false,
                    handleScale: false,
                });
                console.log('Started dragging', priceLine._type, 'line');
            } else {
                // Stop dragging - confirm change
                isDragging = false;
                const newPrice = priceLine.options().price;
                chart.applyOptions({
                    handleScroll: true,
                    handleScale: true,
                });
                
                // Update title
                priceLine.applyOptions({
                    title: `${priceLine._type.toUpperCase()} ${newPrice.toFixed(2)}`
                });
                
                // Dispatch event for Rust to handle
                window.dispatchEvent(new CustomEvent('priceLineMoved', {
                    detail: {
                        type: priceLine._type,
                        positionId: priceLine._positionId,
                        oldPrice: startPrice,
                        newPrice: newPrice,
                    }
                }));
                
                console.log('Finished dragging', priceLine._type, 'from', startPrice, 'to', newPrice);
            }
        }
    });
}

// Update a price line
function updatePriceLine(type, newPrice) {
    const line = priceLines.find(l => l._type === type);
    if (line) {
        line.applyOptions({
            price: newPrice,
            title: `${type.toUpperCase()} ${newPrice.toFixed(2)}`
        });
    }
}

// Remove a price line
function removePriceLine(type) {
    const index = priceLines.findIndex(l => l._type === type);
    if (index !== -1) {
        const line = priceLines[index];
        candlestickSeries.removePriceLine(line);
        priceLines.splice(index, 1);
    }
}

// Clear all price lines
function clearPriceLines() {
    priceLines.forEach(line => {
        candlestickSeries.removePriceLine(line);
    });
    priceLines = [];
}

// Add SL/TP for a position
function addPositionLines(positionId, entryPrice, stopLoss, takeProfit, trailingStop) {
    clearPriceLines();
    
    // Entry line (not draggable)
    addPriceLine('entry', entryPrice, positionId, false);
    
    // SL line (draggable)
    if (stopLoss) {
        addPriceLine('sl', stopLoss, positionId, true);
    }
    
    // TP line (draggable)
    if (takeProfit) {
        addPriceLine('tp', takeProfit, positionId, true);
    }
    
    // Trailing stop line (draggable)
    if (trailingStop) {
        addPriceLine('trailing', trailingStop, positionId, true);
    }
}

// ============================================================================
// Legacy functions for compatibility
// ============================================================================

// Update current price marker
function updateChartPrice(price) {
    if (!candlestickSeries) return;
    
    const data = candlestickSeries.data();
    if (data.length > 0) {
        const lastCandle = data[data.length - 1];
        lastCandle.close = price;
        candlestickSeries.update(lastCandle);
    }
}

// Add a marker (entry, SL, TP) - legacy
function addChartMarker(markerType, price, time) {
    if (!candlestickSeries) return;
    
    const markers = [{
        time: time,
        position: markerType === 'entry' ? 'inBar' : 'aboveBar',
        color: markerType === 'entry' ? '#00ff88' : markerType === 'sl' ? '#ff3366' : '#ffcc00',
        shape: markerType === 'entry' ? 'arrowUp' : 'arrowDown',
        text: markerType.toUpperCase(),
        size: 2,
    }];
    
    candlestickSeries.setMarkers(markers);
}

// Clear all markers
function clearMarkers() {
    if (candlestickSeries) {
        candlestickSeries.setMarkers([]);
    }
}

// Generate mock data as fallback
function useMockData(symbol, timeframe) {
    console.log('Using mock data for', symbol, timeframe);
    const data = generateMockData(timeframe);
    candlestickSeries.setData(data);
    
    const volumeData = generateVolumeData(data);
    volumeSeries.setData(volumeData);
}

// Generate mock candlestick data
function generateMockData(timeframe) {
    const data = [];
    const now = new Date();
    let price = 250;
    let time = new Date(now.getTime() - 100 * getTimeframeMs(timeframe));
    
    for (let i = 0; i < 100; i++) {
        const open = price;
        const change = (Math.random() - 0.5) * 5;
        const close = open + change;
        const high = Math.max(open, close) + Math.random() * 2;
        const low = Math.min(open, close) - Math.random() * 2;
        
        data.push({
            time: time.getTime() / 1000,
            open: open,
            high: high,
            low: low,
            close: close,
        });
        
        price = close;
        time = new Date(time.getTime() + getTimeframeMs(timeframe));
    }
    
    return data;
}

// Generate volume data
function generateVolumeData(candleData) {
    return candleData.map(candle => ({
        time: candle.time,
        value: Math.random() * 1000000,
        color: candle.close >= candle.open ? '#00ff88' : '#ff3366',
    }));
}

// Get interval in milliseconds
function getTimeframeMs(timeframe) {
    const multipliers = {
        '1m': 60 * 1000,
        '5m': 5 * 60 * 1000,
        '15m': 15 * 60 * 1000,
        '30m': 30 * 60 * 1000,
        '1h': 60 * 60 * 1000,
        '4h': 4 * 60 * 60 * 1000,
        '1d': 24 * 60 * 60 * 1000,
    };
    return multipliers[timeframe] || 5 * 60 * 1000;
}

// Show error message
function showError(container, message) {
    container.innerHTML = `
        <div class="w-full h-full flex items-center justify-center text-muted">
            <div class="text-center">
                <p class="text-sm">${message}</p>
                <p class="text-xs text-secondary mt-1">Please check console for errors</p>
            </div>
        </div>
    `;
}

// ============================================================================
// Sound Alerts
// ============================================================================

const AudioContext = window.AudioContext || window.webkitAudioContext;
let audioContext = null;

// Initialize audio context (must be called after user interaction)
function initAudio() {
    if (!audioContext) {
        audioContext = new AudioContext();
    }
}

// Play sound alert
function playAlert(type) {
    initAudio();
    if (!audioContext) return;
    
    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();
    
    oscillator.connect(gainNode);
    gainNode.connect(audioContext.destination);
    
    const now = audioContext.currentTime;
    
    switch(type) {
        case 'signal':
            // High pitch double beep for signal
            oscillator.frequency.setValueAtTime(880, now);
            oscillator.frequency.setValueAtTime(880, now + 0.1);
            gainNode.gain.setValueAtTime(0.3, now);
            gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.5);
            oscillator.start(now);
            oscillator.stop(now + 0.5);
            
            // Second beep
            const osc2 = audioContext.createOscillator();
            const gain2 = audioContext.createGain();
            osc2.connect(gain2);
            gain2.connect(audioContext.destination);
            osc2.frequency.setValueAtTime(1100, now + 0.3);
            gain2.gain.setValueAtTime(0.3, now + 0.3);
            gain2.gain.exponentialRampToValueAtTime(0.01, now + 0.8);
            osc2.start(now + 0.3);
            osc2.stop(now + 0.8);
            break;
            
        case 'order_filled':
            // Success sound
            oscillator.frequency.setValueAtTime(523.25, now); // C5
            oscillator.frequency.exponentialRampToValueAtTime(1046.5, now + 0.1);
            gainNode.gain.setValueAtTime(0.2, now);
            gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.3);
            oscillator.start(now);
            oscillator.stop(now + 0.3);
            break;
            
        case 'risk_alert':
            // Warning sound
            oscillator.type = 'sawtooth';
            oscillator.frequency.setValueAtTime(200, now);
            oscillator.frequency.linearRampToValueAtTime(400, now + 0.2);
            gainNode.gain.setValueAtTime(0.3, now);
            gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.5);
            oscillator.start(now);
            oscillator.stop(now + 0.5);
            break;
            
        case 'emergency':
            // Alarm sound
            oscillator.type = 'square';
            oscillator.frequency.setValueAtTime(800, now);
            gainNode.gain.setValueAtTime(0.4, now);
            
            // Oscillate volume for alarm effect
            for (let i = 0; i < 5; i++) {
                gainNode.gain.setValueAtTime(0.4, now + i * 0.2);
                gainNode.gain.setValueAtTime(0.1, now + i * 0.2 + 0.1);
            }
            
            gainNode.gain.exponentialRampToValueAtTime(0.01, now + 1.0);
            oscillator.start(now);
            oscillator.stop(now + 1.0);
            break;
    }
}

// Volume control
let alertVolume = 0.3;

function setAlertVolume(volume) {
    alertVolume = Math.max(0, Math.min(1, volume));
}

// Export functions for WASM
window.initTradingViewChart = initTradingViewChart;
window.updateChartPrice = updateChartPrice;
window.addChartMarker = addChartMarker;
window.clearMarkers = clearMarkers;
window.addPriceLine = addPriceLine;
window.updatePriceLine = updatePriceLine;
window.removePriceLine = removePriceLine;
window.clearPriceLines = clearPriceLines;
window.addPositionLines = addPositionLines;
window.playAlert = playAlert;
window.setAlertVolume = setAlertVolume;
window.initAudio = initAudio;
