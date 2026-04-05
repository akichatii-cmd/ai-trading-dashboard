# ⚡ Быстрый старт с Codespaces

## Шаг 1: Подготовка (2 минуты)

### Вариант A: Автоматически (скрипт)
```batch
# Запустить скрипт
cd D:\AI-Projects\ai_pro_v5\desktop-dashboard
.\scripts\push-to-github.bat
```

### Вариант B: Вручную
```powershell
cd D:\AI-Projects\ai_pro_v5\desktop-dashboard
git init
git add .
git commit -m "Initial commit"
```

## Шаг 2: Создать репозиторий на GitHub (2 минуты)

1. Откройте https://github.com/new
2. Название: `ai-trading-dashboard`
3. **Не** ставьте галочку "Add README"
4. Создайте репозиторий
5. Скопируйте команды из раздела "…or push an existing repository"

Пример команд:
```bash
git remote add origin https://github.com/ВАШ_НИК/ai-trading-dashboard.git
git push -u origin main
```

## Шаг 3: Открыть в Codespaces (3 минуты)

1. Откройте ваш репозиторий
2. Нажмите зелёную кнопку **<> Code**
3. Выберите вкладку **Codespaces**
4. Нажмите **Create codespace on main**

Ждите пока загрузится (покажется VS Code в браузере).

## Шаг 4: Собрать проект (10 минут)

В терминале Codespaces (внизу экрана):

```bash
cd src-tauri
cargo build --release
```

Ждите окончания сборки (зелёная галочка ✅).

## Шаг 5: Скачать результат

1. Откройте панель **Explorer** слева
2. Перейдите в `src-tauri/target/release/`
3. Найдите файл `desktop-dashboard-tauri`
4. Правый клик → **Download**

## 🎉 Готово!

Теперь у вас есть Linux-версия приложения.

Для Windows-сборки используйте вкладку **Actions** на GitHub - там автоматически собирается Windows версия.

---

**Нужна помощь?** См. полное руководство: [CODESPACES_GUIDE.md](CODESPACES_GUIDE.md)
