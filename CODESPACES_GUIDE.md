# GitHub Codespaces Setup Guide

> Полное руководство по сборке проекта в облаке без проблем Windows Defender

## 🚀 Быстрый старт (5 минут)

### Шаг 1: Создать репозиторий на GitHub

```powershell
# Перейти в папку проекта
cd D:\AI-Projects\ai_pro_v5\desktop-dashboard

# Инициализировать git
git init

# Добавить все файлы
git add .

# Создать первый коммит
git commit -m "Initial commit: AI Trading Dashboard v1.0"

# Создать репозиторий на GitHub (через web или gh CLI)
# Или используйте существующий:
git remote add origin https://github.com/YOUR_USERNAME/ai-trading-dashboard.git

# Отправить код
git push -u origin main
```

**Или через GitHub Web:**
1. Зайдите на https://github.com/new
2. Назовите репозиторий `ai-trading-dashboard`
3. Не добавляйте README (он уже есть)
4. Создайте репозиторий
5. Скопируйте команды из раздела "…or push an existing repository"

### Шаг 2: Открыть в Codespaces

1. Откройте ваш репозиторий на GitHub
2. Нажмите зелёную кнопку **<> Code**
3. Перейдите на вкладку **Codespaces**
4. Нажмите **Create codespace on main**

![Codespaces Create](https://docs.github.com/assets/cb-66037/images/help/codespaces/new-codespace-button.png)

Ждите 2-3 минуты пока среда загрузится.

### Шаг 3: Собрать проект

В терминале Codespaces (уже открыт):

```bash
# Перейти в папку бэкенда
cd src-tauri

# Собрать release версию
cargo build --release

# Ждать 5-10 минут...
```

### Шаг 4: Проверить результат

```bash
# Проверить что файл создан
ls -la target/release/desktop-dashboard-tauri

# Запустить
./target/release/desktop-dashboard-tauri
```

---

## 📦 Скачать собранный файл

### Вариант 1: Через VS Code интерфейс

1. Откройте панель **Explorer** (слева)
2. Найдите `src-tauri/target/release/desktop-dashboard-tauri`
3. Правый клик → **Download**

### Вариант 2: Через терминал

```bash
# Создать архив
cd /workspaces/ai-trading-dashboard
tar -czvf build-linux.tar.gz src-tauri/target/release/desktop-dashboard-tauri

# Переместить в доступное место
mv build-linux.tar.gz ~/build-linux.tar.gz
```

Затем скачайте через вкладку **Ports** или **Explorer**.

### Вариант 3: GitHub Releases (автоматически)

При push в main, GitHub Actions автоматически соберёт и выложит артефакты:

1. Зайдите в репозиторий → **Actions**
2. Откройте последний workflow run
3. Внизу будут артефакты для скачивания

---

## 🛠️ Разработка в Codespaces

### Запуск dev сервера

```bash
# В одном терминале - бэкенд
cd src-tauri
cargo run

# В другом терминале - фронтенд
cd ui
trunk serve
```

### Перенаправление портов

Codespaces автоматически перенаправляет порты:
- **Port 8080** - Tauri приложение
- **Port 8081** - WebSocket сервер
- **Port 3000** - Trunk dev server

Откройте вкладку **Ports** (рядом с Terminal) чтобы увидеть URL.

### Установленные инструменты

В Codespaces уже установлено:
- ✅ Rust + Cargo
- ✅ Node.js 20
- ✅ Tauri CLI
- ✅ Trunk
- ✅ rust-analyzer (VS Code extension)
- ✅ GitHub CLI

---

## 🔧 Расширенная настройка

### Добавить Tinkoff токен

Для paper trading:

```bash
# В Codespaces терминале
cd src-tauri
cp .env.example .env

# Отредактировать через VS Code
# (откройте файл и добавьте токен)
```

**Важно:** Не коммитьте `.env` файл! Он уже в `.gitignore`.

### Сохранить изменения

```bash
# Добавить изменения
git add .

# Коммит
git commit -m "Update: description of changes"

# Push на GitHub
git push
```

---

## 💡 Полезные команды

### Проверка проекта

```bash
# Проверить синтаксис
cargo check

# Форматировать код
cargo fmt

# Запустить линтер
cargo clippy

# Тесты
cargo test
```

### Работа с портами

```bash
# Посмотреть открытые порты
gp ports list

# Сделать порт публичным
gp ports visibility 8080:public
```

### Бесплатные часы Codespaces

GitHub предоставляет бесплатно:
- **120 часов в месяц** для личных аккаунтов
- **15 GB хранилища**

Проверить использование: https://github.com/settings/billing

---

## 🐛 Troubleshooting

### Проблема: Порт не открывается

```bash
# Перенаправить вручную
gp ports forward 8080:8080
```

### Проблема: Сборка медленная

Codespaces использует shared CPU. Для быстрой сборки:

```bash
# Использовать все ядра
cargo build --release -j$(nproc)
```

### Проблема: Нет доступа к файлам

```bash
# Проверить права
ls -la

# При необходимости
sudo chown -R $(whoami) .
```

---

## 📚 Что дальше?

После успешной сборки:

1. **Скачайте** бинарный файл
2. **Протестируйте** на Linux (или используйте WSL2)
3. **Настройте** Tinkoff API токен
4. **Запустите** paper trading

Подробнее: [PAPER_TRADING_GUIDE.md](docs/PAPER_TRADING_GUIDE.md)

---

## ✅ Чек-лист

- [ ] Код загружен на GitHub
- [ ] Codespace создан
- [ ] Сборка прошла успешно (`cargo build --release`)
- [ ] Бинарный файл скачан
- [ ] Приложение запускается

---

## 🎯 Почему Codespaces?

| Плюс | Описание |
|------|----------|
| ✅ Бесплатно | 120 часов/месяц |
| ✅ Нет Windows Defender | Сборка работает всегда |
| ✅ Быстро | 2 CPU, 8 GB RAM |
| ✅ VS Code в браузере | Не нужен локальный редактор |
| ✅ Linux окружение | Стандарт для Rust |
| ✅ Автосохранение | Все изменения на GitHub |

---

**Готово к работе! Создавайте Codespace и начинайте сборку.** 🚀
