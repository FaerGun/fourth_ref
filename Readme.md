# Проект "Кассиопея" - Система сбора космических данных

## Описание

Распределенная система для сбора, хранения и отображения данных о космических объектах:
- Международная космическая станция (ISS)
- NASA OSDR (Open Science Data Repository)
- JWST (James Webb Space Telescope)
- Astronomy API события
- Телеметрия legacy-систем

## Быстрый старт

```bash
# 1. Клонируйте репозиторий
git clone <repository-url>
cd he-path-of-the-samurai

# 2. (Опционально) Настройте .env файл
cp .env.example .env

# 3. Запустите все сервисы
docker-compose up -d

# 4. Откройте в браузере
# http://localhost:8080/dashboard
```

**Подробная инструкция:** см. [QUICKSTART.md](QUICKSTART.md)

## Архитектура

```
┌─────────────┐
│   Browser   │
└──────┬──────┘
       │
┌──────▼──────┐
│   Nginx     │ (Port 8080)
└──────┬──────┘
       │
┌──────▼──────┐
│   Laravel   │ (PHP Web App)
└──────┬──────┘
       │
   ┌───┴───┬──────────────┬─────────────┐
   │       │              │             │
┌──▼──┐ ┌──▼──┐    ┌──────▼─────┐  ┌───▼────┐
│Redis│ │ Rust│    │ PostgreSQL │  │Legacy  │
│Cache│ │ API │    │   (DB)     │  │Python  │
└─────┘ └──┬──┘    └────────────┘  └────────┘
           │
      ┌────┴────┐
      │External │
      │  APIs   │
      └─────────┘
```

## Компоненты

### 1. Rust API Service (`rust_iss`)
- **Технологии:** Rust, Axum, SQLx, Tokio
- **Порт:** 8081
- **Функции:**
  - Сбор данных из внешних API (ISS, NASA, JWST)
  - Хранение в PostgreSQL
  - REST API для веб-приложения
  - Фоновые задачи с защитой от наложения

### 2. Laravel Web App (`php_web`)
- **Технологии:** PHP, Laravel, Bootstrap
- **Порт:** 8080 (через Nginx)
- **Функции:**
  - Дашборды с картами и графиками
  - API-прокси для внешних сервисов
  - CMS для статических страниц

### 3. Legacy Telemetry Service (`legacy_telemetry`)
- **Технологии:** Python 3.11, psycopg2
- **Функции:**
  - Генерация CSV файлов с телеметрией
  - Запись в PostgreSQL через COPY

### 4. PostgreSQL (`iss_db`)
- **Порт:** 5432
- **База:** monolith
- **Таблицы:**
  - `iss_fetch_log` - логи ISS данных
  - `osdr_items` - элементы OSDR
  - `space_cache` - кэш космических данных
  - `telemetry_legacy` - телеметрия

### 5. Redis (`redis_cache`)
- **Порт:** 6379
- **Функции:** Кэширование для ускорения ответов

## API Endpoints

### Rust API (http://localhost:8081)

- `GET /health` - Проверка здоровья
- `GET /last` - Последние данные ISS
- `GET /fetch` - Принудительный сбор ISS данных
- `GET /iss/trend` - Тренд движения ISS
- `GET /osdr/sync` - Синхронизация OSDR
- `GET /osdr/list?limit=20` - Список OSDR элементов
- `GET /space/:src/latest` - Последние данные источника (apod, neo, flr, cme, spacex)
- `GET /space/refresh?src=apod,neo` - Обновление кэша
- `GET /space/summary` - Сводка всех данных

### Laravel API (http://localhost:8080)

- `GET /dashboard` - Главный дашборд
- `GET /osdr` - Страница OSDR
- `GET /api/iss/last` - Прокси к Rust API
- `GET /api/iss/trend` - Прокси к Rust API
- `GET /api/jwst/feed` - JWST галерея
- `GET /api/astro/events` - Астрономические события

## Переменные окружения

Основные переменные (см. `.env.example`):

```bash
# База данных
DATABASE_URL=postgres://monouser:monopass@db:5432/monolith

# NASA API
NASA_API_KEY=your_key_here

# Интервалы обновления (секунды)
ISS_EVERY_SECONDS=120
APOD_EVERY_SECONDS=43200

# JWST API
JWST_API_KEY=your_key_here

# Astronomy API
ASTRO_APP_ID=your_id
ASTRO_APP_SECRET=your_secret
```

## Разработка

### Локальная разработка Rust сервиса

```bash
cd services/rust-iss
cargo build
cargo run
```

### Локальная разработка Python сервиса

```bash
cd services/legacy-telemetry
pip install -r requirements.txt
python main.py
```

### Локальная разработка Laravel

```bash
cd services/php-web
composer install
php artisan serve
```

## Тестирование

```bash
# Проверка здоровья всех сервисов
curl http://localhost:8081/health
curl http://localhost:8080/dashboard

# Проверка БД
docker-compose exec db psql -U monouser -d monolith -c "SELECT COUNT(*) FROM iss_fetch_log;"

# Проверка Redis
docker-compose exec redis redis-cli ping
```

## Документация

- [QUICKSTART.md](QUICKSTART.md) - Быстрый старт
- [REFACTORING_REPORT.md](REFACTORING_REPORT.md) - Отчет о рефакторинге
- [services/legacy-telemetry/README.md](services/legacy-telemetry/README.md) - Документация legacy сервиса

## Лицензия

Проект создан для компании "Кассиопея"

## Контакты

Для вопросов и поддержки обращайтесь к команде разработки.
