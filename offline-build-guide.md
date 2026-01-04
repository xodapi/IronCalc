# Портабельная сборка IronCalc для оффлайн-использования

## Структура проекта

IronCalc - это многокомпонентный проект, включающий:
- `base` - основная библиотека на Rust для обработки электронных таблиц
- `xlsx` - модуль для работы с XLSX-файлами
- `bindings/wasm` - веб-биндинги
- `bindings/python` - Python-биндинги
- `bindings/nodejs` - Node.js-биндинги
- `webapp` - веб-приложение

## Подготовка оффлайн-сборки

### 1. Скачивание всех зависимостей

Для подготовки оффлайн-сборки сначала нужно скачать все зависимости в кэш:

```bash
# Перейдите в директорию проекта
cd IronCalc

# Загрузите и закэшируйте все Rust-зависимости
cargo fetch

# Загрузите и закэшируйте npm-зависимости для веб-приложения
cd webapp/IronCalc
npm install
npm cache verify
cd ../app.ironcalc.com/frontend
npm install
npm cache verify
```

### 2. Создание директории с закэшированными зависимостями

```bash
mkdir offline_cache
cd offline_cache

# Копируем кэш Cargo
mkdir -p cargo_registry
cp -r $CARGO_HOME/registry/* cargo_registry/ 2>/dev/null || true
cp -r $CARGO_HOME/git/* cargo_git/ 2>/dev/null || true

# Копируем кэш npm
mkdir npm_cache
npm pack @emotion/react @emotion/styled @mui/material @mui/system i18next lucide-react react-colorful react-i18next
```

### 3. Альтернативный подход: создание vendor-директории

Вы также можете создать vendor-директорию с исходниками зависимостей:

```bash
# В корне проекта
cargo vendor vendor/
```

### 4. Сборка проекта в оффлайн-режиме

После подготовки кэша, для оффлайн-сборки используйте:

```bash
# Для Rust-компонентов
cargo build --offline --release

# Для wasm-биндингов (предполагая, что они уже собраны)
cd bindings/wasm
wasm-pack build --target web --offline

# Для веб-приложения (предполагая, что зависимости уже установлены)
cd webapp/IronCalc
npm ci --offline  # установит зависимости из node_modules
npm run build
```

## Портабельная сборка для дистрибуции

### 1. Сборка релизных бинарных файлов

```bash
# Собрать основную библиотеку и утилиты
cd xlsx
cargo build --release --offline

# Собрать бинарные файлы для конвертации
cargo build --release --bin xlsx_2_icalc --offline
```

### 2. Подготовка директории распространения

```bash
mkdir ironcalc_portable
cd ironcalc_portable

# Копируем релизные бинарные файлы
mkdir bin
cp ../target/release/ironcalc* bin/ 2>/dev/null || true
cp ../target/release/xlsx_2_icalc bin/

# Копируем веб-приложение
mkdir web
cp -r ../webapp/IronCalc/dist/* web/ 2>/dev/null || true

# Копируем примеры
mkdir examples
cp -r ../xlsx/tests/docs/*.xlsx examples/ 2>/dev/null || true
cp -r ../xlsx/tests/templates/*.xlsx examples/ 2>/dev/null || true
```

### 3. Создание автономного запускаемого файла

Для портабельной версии вы можете использовать встроенный веб-сервер:

```bash
# Собрать веб-сервер
cd webapp/app.ironcalc.com/server
cargo build --release --offline

# Подготовить автономный пакет
mkdir ironcalc_offline
cd ironcalc_offline
cp ../target/release/ironcalc_server ./
cp Rocket.toml ./
mkdir static
cp -r ../webapp/IronCalc/dist/* static/
```

## Использование оффлайн-версии

### Запуск веб-версии:
```bash
# В директории с ironcalc_server
./ironcalc_server
# Приложение будет доступно по адресу http://localhost:8000
```

### Использование библиотеки в Rust-проекте:

1. Создайте новую директорию для вашего проекта
2. Добавьте в Cargo.toml путь к локальной библиотеке:

```toml
[dependencies]
ironcalc = { path = "/path/to/ironcalc/xlsx" }
```

## Создание portable-версии для Windows

Для Windows-пользователей создайте bat-скрипт для запуска:

```batch
@echo off
echo Запуск IronCalc в оффлайн-режиме...
cd /d "%~dp0"
if exist "ironcalc_server.exe" (
    start "" "http://localhost:8000"
    ironcalc_server.exe
) else (
    echo Отсутствует исполняемый файл ironcalc_server.exe
    pause
)
```

## Docker без интернета

Если Docker доступен, можно создать автономный образ:

```bash
# Собрать образ (необходим интернет для первой сборки)
docker build --target server-runtime -t ironcalc:offline .

# Сохранить образ для переноса
docker save ironcalc:offline -o ironcalc_offline.tar

# Загрузить образ на другой системе
docker load -i ironcalc_offline.tar

# Запустить
docker run -p 8000:8000 ironcalc:offline
```

## Заключение

Для полной оффлайн-сборки рекомендуется:

1. Подготовить кэш зависимостей на системе с интернетом
2. Скопировать проект и кэш на целевую систему
3. Использовать флаг `--offline` для всех команд cargo
4. Использовать `npm ci` вместо `npm install` для установки из кэша