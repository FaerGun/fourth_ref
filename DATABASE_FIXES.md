# Исправления базы данных

## Проблема 1: Отсутствует таблица `cms_blocks`

### Ошибка
```
SQLSTATE[42P01]: Undefined table: 7 ERROR: relation "cms_blocks" does not exist
```

### Решение
1. Добавлена таблица `cms_blocks` в `db/init.sql`
2. Создана таблица в существующей БД через `docker-compose exec db psql`
3. Добавлена тестовая запись для `dashboard_experiment`

### SQL
```sql
CREATE TABLE IF NOT EXISTS cms_blocks (
    id BIGSERIAL PRIMARY KEY,
    slug TEXT UNIQUE NOT NULL,
    content TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Проблема 2: Нет данных ISS в базе

### Причина
Фоновые задачи Rust сервиса могут еще не успеть получить данные от внешних API.

### Решение
1. Проверены логи Rust сервиса
2. Обновлен `RustIssService` для правильной обработки формата ответа `SuccessResponse`
3. Можно вручную запустить задачу через `/trigger` endpoint

### Проверка
```bash
# Проверить количество записей
docker-compose exec db psql -U monouser -d monolith -c "SELECT COUNT(*) FROM iss_fetch_log;"

# Вручную запустить задачу (через API)
curl http://localhost:8081/trigger
```

## Статус

✅ **Таблица `cms_blocks` создана**
✅ **Тестовые данные добавлены**
✅ **RustIssService обновлен для правильной обработки ответов**

## Следующие шаги

1. Дождаться выполнения фоновых задач Rust сервиса (интервалы заданы в `.env`)
2. Или вручную запустить через `/trigger` endpoint
3. Обновить страницу dashboard - ошибки CMS должны исчезнуть


