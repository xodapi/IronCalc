# IronCalc Desktop (Tauri 2.0)

Десктопная версия IronCalc для работы в закрытом контуре без интернета.

## Требования

- Windows 10/11 (WebView2 обычно предустановлен)
- Rust 1.70+
- Node.js 18+

## Первичная настройка (требует интернет один раз)

```bash
# 1. Установить зависимости npm
cd webapp/IronCalc
npm install

# 2. Установить Tauri CLI
npm install -D @tauri-apps/cli@latest @tauri-apps/api@latest

# 3. Скачать Rust зависимости Tauri
cd src-tauri
cargo fetch

# 4. (Опционально) Создать vendor для офлайн-сборки
cargo vendor vendor-tauri

# Добавить в src-tauri/.cargo/config.toml:
# [source.crates-io]
# replace-with = "vendored-sources"
# [source.vendored-sources]
# directory = "vendor-tauri"
```

## Сборка (офлайн)

```bash
cd webapp/IronCalc

# Собрать frontend
npm run build

# Собрать десктоп-приложение
cd src-tauri
cargo build --release --offline
```

## Разработка

```bash
cd webapp/IronCalc
npx tauri dev
```

## Структура

```
src-tauri/
├── Cargo.toml          # Rust зависимости
├── tauri.conf.json     # Конфигурация Tauri
├── build.rs            # Build script
├── capabilities/       # Разрешения приложения
│   └── default.json
├── icons/              # Иконки приложения
└── src/
    ├── main.rs         # Entry point
    ├── lib.rs          # Library
    └── commands.rs     # Нативные команды (файлы)
```

## Примечания

- Tauri изолирован в `[workspace]` — не влияет на сборку основного ironcalc
- Используется `ironcalc = { path = "../../../xlsx" }` для нативных операций
- WebView2 bundled для офлайн-установки (`embedBootstrapper`)
