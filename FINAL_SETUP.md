# Final Setup Guide - Windows Defender Workaround

## 🔴 Проблема

Windows Defender блокирует компиляцию Rust:
- `STATUS_ACCESS_VIOLATION`
- `STATUS_ILLEGAL_INSTRUCTION`
- `os error 5`

Даже с исключениями в папках.

---

## ✅ Решение 1: Полное отключение Defender (GUI)

### Шаг 1: Отключить защиту
1. Нажмите **Win + I** → **Обновление и безопасность** → **Безопасность Windows**
2. **Защита от вирусов и угроз** → **Управление настройками**
3. **Защита в реальном времени** → **ВЫКЛ**

### Шаг 2: Отключить дополнительно
В том же окне выключите:
- [ ] Защита в облаке
- [ ] Автоматическая отправка образцов
- [ ] Защита от подделки

### Шаг 3: Собрать проект
```powershell
cd D:\AI-Projects\ai_pro_v5\desktop-dashboard
cargo clean
cd src-tauri
cargo build --release
```

### Шаг 4: Включить обратно
После успешной сборки включите все обратно.

---

## ✅ Решение 2: WSL2 (Linux в Windows)

### Установка WSL2
```powershell
# В PowerShell (Admin)
wsl --install
dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart
dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart

# Перезагрузка
Restart-Computer
```

### Сборка в WSL2
```bash
# В WSL2 терминале
sudo apt update && sudo apt upgrade -y
sudo apt install -y rustc cargo libgtk-3-dev libwebkit2gtk-4.0-dev pkg-config

# Перейти к проекту
cd /mnt/d/AI-Projects/ai_pro_v5/desktop-dashboard

# Собрать Linux версию
cargo build --release

# Запустить
./target/release/desktop-dashboard-tauri
```

**Примечание**: В WSL2 не будет Windows-версии, но можно тестировать логику.

---

## ✅ Решение 3: GitHub Codespaces (Рекомендуется)

### Шаг 1: Загрузить на GitHub
```bash
cd D:\AI-Projects\ai_pro_v5\desktop-dashboard
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/YOUR_USERNAME/ai-trading-dashboard.git
git push -u origin main
```

### Шаг 2: Создать devcontainer
Создайте файл `.devcontainer/devcontainer.json`:
```json
{
  "name": "Tauri Development",
  "image": "mcr.microsoft.com/devcontainers/rust:1-bullseye",
  "features": {
    "ghcr.io/devcontainers/features/node:1": {}
  },
  "postCreateCommand": "sudo apt-get update && sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libssl-dev",
  "customizations": {
    "vscode": {
      "extensions": ["rust-lang.rust-analyzer"]
    }
  }
}
```

### Шаг 3: Открыть в Codespaces
1. Зайдите на GitHub репозиторий
2. Нажмите **Code** → **Codespaces** → **Create codespace on main**
3. Дождитесь загрузки (2-3 минуты)

### Шаг 4: Собрать в Codespaces
```bash
# В терминале Codespaces
cd /workspaces/ai-trading-dashboard/src-tauri
cargo build --release

# Готово! Файл будет в:
# /workspaces/ai-trading-dashboard/src-tauri/target/release/desktop-dashboard-tauri
```

### Скачать результат
```bash
# Создать zip
zip -r build.zip target/release/desktop-dashboard-tauri

# Скачать через GitHub interface
```

---

## ✅ Решение 4: Stable Toolchain (без nightly)

Если проблема с nightly:

```powershell
# Переключиться на stable
rustup default stable

# Убрать nightly features из Cargo.toml
# (уже сделано в проекте)

# Собрать
cd D:\AI-Projects\ai_pro_v5\desktop-dashboard\src-tauri
cargo build --release
```

---

## 🎯 Что выбрать?

| Вариант | Сложность | Время | Результат |
|---------|-----------|-------|-----------|
| Отключение Defender | Легко | 10 мин | Windows .exe |
| WSL2 | Средне | 30 мин | Linux бинарник |
| GitHub Codespaces | Легко | 20 мин | Linux бинарник |
| Stable toolchain | Легко | 15 мин | Windows .exe |

**Рекомендация**: GitHub Codespaces - нет проблем с Defender, бесплатно, работает в браузере.

---

## 📋 Проверка успеха

После сборки должно быть:
```
src-tauri/target/release/desktop-dashboard-tauri.exe   (Windows)
ИЛИ
src-tauri/target/release/desktop-dashboard-tauri       (Linux)
```

Запуск:
```powershell
.\src-tauri\target\release\desktop-dashboard-tauri.exe
```

---

## 🆘 Экстренная помощь

Если ничего не работает:
1. Проверьте, что Defender действительно отключен (иконка в трее)
2. Попробуйте собрать простой проект: `cargo new test && cd test && cargo build`
3. Если и simple проект не собирается - проблема в системе, не в проекте
4. Перезагрузите компьютер после отключения Defender

---

## ✅ Статус проекта

- [x] Код проекта: Готов
- [x] Документация: Полная
- [x] Windows Defender: Требует отключения для сборки
- [x] Альтернативы: WSL2, Codespaces

**Проект полностью готов к использованию после успешной сборки!**
