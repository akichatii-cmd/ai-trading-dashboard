# Codespaces Fix Guide

## Проблема
Codespace создан, но код не загружен или не в той папке.

## ✅ Решение

Выполните в терминале Codespaces:

```bash
# 1. Перейти в workspaces
cd /workspaces

# 2. Клонировать ваш репозиторий
git clone https://github.com/akichatii-cmd/ai-trading-dashboard.git

# 3. Перейти в папку проекта
cd ai-trading-dashboard

# 4. Проверить что файлы есть
ls -la
# Должны видеть: src-tauri, ui, docs, Cargo.toml, etc.
```

## Если Rust не установлен

```bash
# Установить Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Проверить
rustc --version
```

## Установить зависимости

```bash
# Системные библиотеки для Tauri
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev pkg-config

# Rust targets
rustup target add wasm32-unknown-unknown
```

## Собрать проект

```bash
cd /workspaces/ai-trading-dashboard/src-tauri
cargo build --release
```

Ждите 10-15 минут...

## Скачать результат

После успешной сборки:

```bash
# Проверить что файл создан
ls -la /workspaces/ai-trading-dashboard/src-tauri/target/release/desktop-dashboard-tauri

# Создать архив для скачивания
cd /workspaces/ai-trading-dashboard
tar -czvf ai-trading-dashboard-linux.tar.gz -C src-tauri/target/release desktop-dashboard-tauri
```

Затем в VS Code (Codespaces):
1. Откройте папку `/workspaces/ai-trading-dashboard` в Explorer
2. Найдите файл `ai-trading-dashboard-linux.tar.gz`
3. Правый клик → **Download**

## 🔥 Альтернатива: Rebuild Container

Если ничего не работает:

1. Нажмите `Ctrl+Shift+P` (или `Cmd+Shift+P` на Mac)
2. Наберите: `Rebuild Container`
3. Выберите **Codespaces: Rebuild Container**
4. Дождитесь пересборки (2-3 минуты)
5. Код должен появиться автоматически

## ⚡ Быстрая проверка

После всех шагов:

```bash
# Должно показать путь
pwd
# /workspaces/ai-trading-dashboard

# Должны быть файлы
ls src-tauri/
# Cargo.toml  build.rs  src/

# Сборка должна работать
cd src-tauri && cargo check
```

Если `cargo check` проходит без ошибок - всё готово к полной сборке!
