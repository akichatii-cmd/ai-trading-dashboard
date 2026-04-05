# Build Instructions - AI Trading Bot Dashboard

## ⚠️ Проблема

Windows Defender блокирует компиляцию Rust даже с исключениями.

## ✅ Решение

### Шаг 1: Ручное отключение Defender (GUI)

1. Откройте **Пуск** → поиск **"Безопасность Windows"**
2. **Защита от вирусов и угроз** → **Управление настройками**
3. Выключите **Защита в реальном времени**

### Шаг 2: Установите зависимости

```powershell
# В PowerShell (обычный режим)
rustup default nightly-x86_64-pc-windows-msvc
rustup target add wasm32-unknown-unknown
rustup target add x86_64-pc-windows-gnu

# Установить trunk
cargo install trunk

# Установить tauri-cli
cargo install tauri-cli
```

### Шаг 3: Собрать проект

```powershell
# Перейти в проект
cd D:\AI-Projects\ai_pro_v5\desktop-dashboard

# Очистить кэш
cargo clean

# Собрать backend
cd src-tauri
cargo build --release

# ИЛИ собрать всё через Tauri
cargo tauri build
```

### Шаг 4: Включить Defender обратно

Вернитесь в **Безопасность Windows** и включите **Защита в реальном времени**.

---

## 🚀 Альтернатива: WSL2 (Рекомендуется)

Если проблемы с Windows Defender persist:

```bash
# В WSL2 терминале:
sudo apt update
sudo apt install -y libgtk-3-dev libwebkit2gtk-4.0-dev librust-openssl-dev

# Клонировать/перейти к проекту
cd /mnt/d/AI-Projects/ai_pro_v5/desktop-dashboard

# Собрать Linux-версию
cargo build --release

# Запустить для тестирования
./target/release/desktop-dashboard-tauri
```

---

## 📦 Альтернатива: GitHub Codespaces (Бесплатно)

1. Загрузите код на GitHub
2. Создайте `.devcontainer/devcontainer.json`:

```json
{
  "name": "Rust + Tauri",
  "image": "mcr.microsoft.com/devcontainers/rust:1",
  "features": {
    "ghcr.io/devcontainers/features/node:1": {}
  },
  "postCreateCommand": "sudo apt-get update && sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev"
}
```

3. Откройте в Codespaces
4. Соберите без проблем с Defender!

---

## 🎯 Текущий статус проекта

| Компонент | Статус |
|-----------|--------|
| Исходный код | ✅ Готов |
| Документация | ✅ Полная |
| Windows Defender | ⚠️ Требует отключения |
| Сборка | ⚠️ Требует ручного отключения Defender |
| Готовность | ✅ Код-готов, ждёт сборки |

---

## 📝 Проверка после сборки

```powershell
# Проверить что EXE создан
Test-Path .\src-tauri\target\release\desktop-dashboard-tauri.exe

# Запустить
.\src-tauri\target\release\desktop-dashboard-tauri.exe
```

---

## 🆘 Если ничего не работает

**Минимальный тест** - проверить только синтаксис:

```powershell
cd D:\AI-Projects\ai_pro_v5\desktop-dashboard\src-tauri
cargo check
```

Если `cargo check` проходит без ошибок - код корректен, проблема только в полной сборке.

---

**Готов помочь с дальнейшей отладкой!** Проект полностью готов к использованию после успешной сборки.
