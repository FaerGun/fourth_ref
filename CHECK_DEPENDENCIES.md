# Проверка зависимостей проекта

## ✅ Статус проверки

Все зависимости проекта проверены и готовы к использованию.

## Rust зависимости (rust_iss)

Файл: `services/rust-iss/Cargo.toml`

### Основные зависимости:
- ✅ **tokio** (1.x) - Асинхронный runtime с поддержкой макросов, многопоточности и времени
- ✅ **axum** (0.7) - Веб-фреймворк для Rust
- ✅ **sqlx** (0.7) - Асинхронный драйвер PostgreSQL с поддержкой Tokio, JSON и chrono
- ✅ **reqwest** (0.11) - HTTP клиент с поддержкой JSON, сжатия и TLS
- ✅ **serde** (1.x) - Сериализация/десериализация
- ✅ **serde_json** (1.x) - JSON поддержка для serde
- ✅ **chrono** (0.4) - Работа с датами и временем
- ✅ **uuid** (1.0) - Генерация UUID (v4, serde)
- ✅ **tracing** (0.1) - Структурированное логирование
- ✅ **tracing-subscriber** (0.3) - Подписчик для tracing с env-filter
- ✅ **dotenvy** (0.15) - Загрузка .env файлов
- ✅ **anyhow** (1.x) - Упрощенная обработка ошибок
- ✅ **thiserror** (1.x) - Создание типов ошибок

### Проверка установки:
```bash
cd services/rust-iss
cargo check
```

Все зависимости будут автоматически загружены при сборке Docker образа.

## Python зависимости (legacy_telemetry)

Файл: `services/legacy-telemetry/requirements.txt`

### Зависимости:
- ✅ **psycopg2-binary** (2.9.9) - Драйвер PostgreSQL для Python

### Проверка установки:
```bash
cd services/legacy-telemetry
pip install -r requirements.txt
```

Все зависимости будут автоматически установлены при сборке Docker образа.

## Docker образы

Все базовые образы указаны в `docker-compose.yml`:

- ✅ **postgres:16** - PostgreSQL база данных
- ✅ **redis:7-alpine** - Redis кэш-сервер
- ✅ **nginx:1.27-alpine** - Nginx веб-сервер
- ✅ **rust:slim** - Базовый образ для сборки Rust (в Dockerfile)
- ✅ **debian:12-slim** - Runtime образ для Rust сервиса
- ✅ **php:8.3-fpm-alpine** - PHP-FPM для Laravel
- ✅ **composer:2** - Composer для PHP зависимостей
- ✅ **python:3.11-slim** - Python для legacy сервиса

## PHP зависимости (Laravel)

PHP зависимости управляются через Composer и будут установлены автоматически при сборке образа через `entrypoint.sh`.

## Проверка всех зависимостей перед запуском

### 1. Проверка Docker и Docker Compose

```bash
docker --version
# Должно быть: Docker version 20.10 или выше

docker-compose --version
# Должно быть: Docker Compose version 2.0 или выше
```

### 2. Проверка доступности портов

```bash
# Windows PowerShell
netstat -ano | findstr :8080
netstat -ano | findstr :8081
netstat -ano | findstr :5432
netstat -ano | findstr :6379

# Linux/Mac
lsof -i :8080
lsof -i :8081
lsof -i :5432
lsof -i :6379
```

Порты должны быть свободны.

### 3. Проверка конфигурации docker-compose

```bash
cd he-path-of-the-samurai
docker-compose config
```

Должно вывести валидную конфигурацию без ошибок.

### 4. Проверка структуры проекта

Убедитесь, что все файлы на месте:

```
he-path-of-the-samurai/
├── docker-compose.yml ✅
├── .env.example ✅
├── db/
│   └── init.sql ✅
├── services/
│   ├── rust-iss/
│   │   ├── Cargo.toml ✅
│   │   ├── Dockerfile ✅
│   │   └── src/ ✅
│   ├── php-web/
│   │   ├── Dockerfile ✅
│   │   └── laravel-patches/ ✅
│   └── legacy-telemetry/
│       ├── requirements.txt ✅
│       ├── Dockerfile ✅
│       └── main.py ✅
```

## Автоматическая проверка при сборке

При выполнении `docker-compose build`:

1. **Rust сервис** - Cargo автоматически загрузит все зависимости из `Cargo.toml`
2. **Python сервис** - pip автоматически установит зависимости из `requirements.txt`
3. **PHP сервис** - Composer установит Laravel зависимости через entrypoint.sh

## Решение проблем с зависимостями

### Rust зависимости не загружаются

```bash
# Очистка кэша и пересборка
cd services/rust-iss
docker-compose build rust_iss --no-cache
```

### Python зависимости не устанавливаются

```bash
# Проверка requirements.txt
cd services/legacy-telemetry
pip install -r requirements.txt

# Пересборка образа
docker-compose build legacy_telemetry --no-cache
```

### PHP зависимости не устанавливаются

Проверьте `entrypoint.sh` в `services/php-web/` - он должен запускать `composer install`.

## Итоговая проверка

После выполнения всех проверок запустите:

```bash
docker-compose up -d
docker-compose ps
```

Все сервисы должны быть в статусе `Up` или `Up (healthy)`.

## Следующие шаги

После успешной проверки зависимостей:
1. Следуйте инструкциям в [QUICKSTART.md](QUICKSTART.md)
2. Откройте http://localhost:8080/dashboard
3. Проверьте API: http://localhost:8081/health


