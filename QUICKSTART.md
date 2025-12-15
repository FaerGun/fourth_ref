# Быстрый старт проекта "Кассиопея"

## Требования

Перед запуском убедитесь, что у вас установлены:

- **Docker** версии 20.10 или выше
- **Docker Compose** версии 2.0 или выше
- **Git** (для клонирования репозитория)

Проверка установки:
```bash
docker --version
docker-compose --version
```

## Установка и запуск

### 1. Клонирование репозитория (если нужно)

```bash
git clone <repository-url>
cd he-path-of-the-samurai
```

### 2. Настройка переменных окружения (опционально)

Создайте файл `.env` в корне проекта (можно скопировать из `.env.example`):

```bash
cp .env.example .env
```

Отредактируйте `.env` при необходимости, особенно:
- `NASA_API_KEY` - ваш ключ NASA API (если есть)
- `JWST_API_KEY` - ключ JWST API
- `ASTRO_APP_ID` и `ASTRO_APP_SECRET` - ключи Astronomy API

**Примечание:** Если не указать ключи, некоторые функции могут работать в ограниченном режиме.

### 3. Запуск всех сервисов

```bash
docker-compose up -d
```

Эта команда:
- Соберет все Docker образы с нуля
- Запустит все сервисы в фоновом режиме
- Создаст необходимые volumes и networks

### 4. Проверка статуса

```bash
docker-compose ps
```

Все сервисы должны быть в статусе `Up` или `Up (healthy)`.

### 5. Просмотр логов

```bash
# Все сервисы
docker-compose logs -f

# Конкретный сервис
docker-compose logs -f rust_iss
docker-compose logs -f php_web
docker-compose logs -f legacy_telemetry
```

## Доступ к сервисам

После запуска сервисы будут доступны по следующим адресам:

| Сервис | URL | Описание |
|--------|-----|----------|
| Веб-интерфейс | http://localhost:8080 | Главная страница дашборда |
| Rust API | http://localhost:8081 | REST API для космических данных |
| PostgreSQL | localhost:5432 | База данных (user: monouser, pass: monopass) |
| Redis | localhost:6379 | Кэш-сервер |

### Основные эндпоинты Rust API

- `GET http://localhost:8081/health` - Проверка здоровья сервиса
- `GET http://localhost:8081/last` - Последние данные ISS
- `GET http://localhost:8081/iss/trend` - Тренд движения ISS
- `GET http://localhost:8081/osdr/list` - Список OSDR элементов
- `GET http://localhost:8081/space/summary` - Сводка всех космических данных

### Веб-страницы

- `http://localhost:8080/dashboard` - Главный дашборд
- `http://localhost:8080/osdr` - Страница OSDR данных
- `http://localhost:8080/api/jwst/feed` - JWST галерея (JSON)
- `http://localhost:8080/api/astro/events` - События астрономии (JSON)

## Структура проекта

```
he-path-of-the-samurai/
├── docker-compose.yml          # Конфигурация всех сервисов
├── .env.example                # Пример переменных окружения
├── db/
│   └── init.sql                # Инициализация БД
├── services/
│   ├── rust-iss/               # Rust сервис (Axum + SQLx)
│   │   ├── Cargo.toml          # Зависимости Rust
│   │   ├── Dockerfile
│   │   └── src/                # Исходный код
│   ├── php-web/                # Laravel веб-приложение
│   │   ├── Dockerfile
│   │   └── laravel-patches/    # Патчи Laravel
│   └── legacy-telemetry/       # Python микросервис
│       ├── requirements.txt    # Зависимости Python
│       ├── Dockerfile
│       └── main.py
└── README.md
```

## Проверка зависимостей

### Rust зависимости (Cargo.toml)

Все зависимости указаны в `services/rust-iss/Cargo.toml`:
- ✅ tokio - асинхронный runtime
- ✅ axum - веб-фреймворк
- ✅ sqlx - работа с PostgreSQL
- ✅ reqwest - HTTP клиент
- ✅ serde/serde_json - сериализация
- ✅ chrono - работа с датами
- ✅ uuid - генерация UUID
- ✅ tracing - логирование

### Python зависимости (requirements.txt)

Все зависимости указаны в `services/legacy-telemetry/requirements.txt`:
- ✅ psycopg2-binary - драйвер PostgreSQL

Docker автоматически установит все зависимости при сборке образов.

## Остановка сервисов

```bash
# Остановка с сохранением данных
docker-compose stop

# Остановка и удаление контейнеров (данные сохраняются в volumes)
docker-compose down

# Полная очистка (включая volumes - ВНИМАНИЕ: удалит данные!)
docker-compose down -v
```

## Пересборка после изменений

Если вы внесли изменения в код:

```bash
# Пересборка конкретного сервиса
docker-compose build rust_iss
docker-compose up -d rust_iss

# Пересборка всех сервисов
docker-compose build
docker-compose up -d
```

## Решение проблем

### Сервис не запускается

1. Проверьте логи:
   ```bash
   docker-compose logs <service_name>
   ```

2. Проверьте, что порты не заняты:
   ```bash
   # Windows
   netstat -ano | findstr :8080
   netstat -ano | findstr :8081
   
   # Linux/Mac
   lsof -i :8080
   lsof -i :8081
   ```

3. Проверьте доступность БД:
   ```bash
   docker-compose exec db psql -U monouser -d monolith -c "SELECT 1;"
   ```

### Rust сервис не компилируется

1. Проверьте версию Rust в Dockerfile
2. Проверьте логи сборки:
   ```bash
   docker-compose build rust_iss --no-cache
   ```

### Laravel не подключается к БД

1. Убедитесь, что БД запущена:
   ```bash
   docker-compose ps db
   ```

2. Проверьте переменные окружения в `docker-compose.yml`

### Redis не работает

1. Проверьте статус:
   ```bash
   docker-compose exec redis redis-cli ping
   ```
   Должен вернуть `PONG`

## Полезные команды

```bash
# Войти в контейнер
docker-compose exec rust_iss sh
docker-compose exec php_web bash

# Выполнить команду в контейнере
docker-compose exec db psql -U monouser -d monolith

# Просмотр использования ресурсов
docker stats

# Очистка неиспользуемых образов
docker system prune -a
```

## Следующие шаги

1. Откройте http://localhost:8080/dashboard в браузере
2. Проверьте API: http://localhost:8081/health
3. Изучите логи для понимания работы системы
4. Прочитайте `REFACTORING_REPORT.md` для понимания архитектуры

## Поддержка

При возникновении проблем:
1. Проверьте логи сервисов
2. Убедитесь, что все зависимости установлены
3. Проверьте конфигурацию в `docker-compose.yml`
4. Изучите `REFACTORING_REPORT.md` для понимания архитектуры


