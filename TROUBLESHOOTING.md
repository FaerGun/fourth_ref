# Решение проблем

## Ошибка 404 - Not Found

### Проблема: "Cannot open document for: /dashboard"

**Причина:** Сервисы не запущены или не готовы.

### Решение:

1. **Проверьте статус сервисов:**
   ```bash
   cd "C:\Users\shved\Desktop\refuck\forth\he-path-of-the-samurai"
   docker-compose ps
   ```

2. **Если сервисы не запущены, запустите их:**
   ```bash
   docker-compose up -d
   ```

3. **Дождитесь полной инициализации (может занять 1-2 минуты):**
   ```bash
   # Проверьте логи PHP сервиса
   docker-compose logs php -f
   ```
   
   Дождитесь сообщения: `[php] starting php-fpm`

4. **Проверьте, что все сервисы запущены:**
   ```bash
   docker-compose ps
   ```
   
   Все сервисы должны быть в статусе `Up` или `Up (healthy)`:
   - ✅ db (iss_db)
   - ✅ redis (redis_cache)
   - ✅ rust_iss
   - ✅ php (php_web)
   - ✅ nginx (web_nginx)
   - ✅ legacy_telemetry

5. **Проверьте доступность:**
   - http://localhost:8080/ (должен редиректить на /dashboard)
   - http://localhost:8080/dashboard
   - http://localhost:8081/health

### Если проблема сохраняется:

1. **Пересоберите образы:**
   ```bash
   docker-compose down
   docker-compose build --no-cache
   docker-compose up -d
   ```

2. **Проверьте логи nginx:**
   ```bash
   docker-compose logs nginx
   ```

3. **Проверьте логи PHP:**
   ```bash
   docker-compose logs php
   ```

4. **Проверьте, что Laravel создан:**
   ```bash
   docker-compose exec php ls -la /var/www/html/
   ```
   
   Должны быть файлы: `artisan`, `composer.json`, `public/index.php`

5. **Проверьте маршруты Laravel:**
   ```bash
   docker-compose exec php php /var/www/html/artisan route:list
   ```
   
   Должен быть маршрут `GET /dashboard`

## Другие частые проблемы

### Порт уже занят

**Ошибка:** `Bind for 0.0.0.0:8080 failed: port is already allocated`

**Решение:**
```bash
# Windows - найти процесс на порту 8080
netstat -ano | findstr :8080

# Остановить процесс или изменить порт в docker-compose.yml
```

### База данных не готова

**Ошибка:** `Connection refused` или `database does not exist`

**Решение:**
```bash
# Проверьте статус БД
docker-compose ps db

# Дождитесь статуса "healthy"
# Проверьте подключение
docker-compose exec db psql -U monouser -d monolith -c "SELECT 1;"
```

### Laravel не создается

**Проблема:** PHP сервис не может создать Laravel проект

**Решение:**
```bash
# Проверьте логи
docker-compose logs php

# Убедитесь, что есть интернет для composer
docker-compose exec php ping -c 3 8.8.8.8

# Пересоздайте контейнер
docker-compose up -d --force-recreate php
```

### Rust сервис не компилируется

**Ошибка:** Ошибки компиляции Rust

**Решение:**
```bash
# Проверьте логи сборки
docker-compose build rust_iss --no-cache

# Убедитесь, что Cargo.toml корректен
cat services/rust-iss/Cargo.toml
```

### Redis не работает

**Проблема:** Laravel не может подключиться к Redis

**Решение:**
```bash
# Проверьте Redis
docker-compose exec redis redis-cli ping
# Должен вернуть: PONG

# Проверьте переменные окружения
docker-compose exec php env | grep REDIS
```

## Полная переустановка

Если ничего не помогает:

```bash
# 1. Остановите все сервисы
docker-compose down -v

# 2. Очистите Docker
docker system prune -a

# 3. Пересоберите все
docker-compose build --no-cache

# 4. Запустите
docker-compose up -d

# 5. Дождитесь инициализации
docker-compose logs -f
```

## Проверка работоспособности

После запуска проверьте:

1. **Health check Rust API:**
   ```bash
   curl http://localhost:8081/health
   ```
   Должен вернуть: `{"ok":true,"status":"ok","now":"..."}`

2. **Главная страница:**
   ```bash
   curl http://localhost:8080/
   ```
   Должен быть редирект на /dashboard

3. **Dashboard:**
   ```bash
   curl http://localhost:8080/dashboard
   ```
   Должен вернуть HTML страницу

4. **База данных:**
   ```bash
   docker-compose exec db psql -U monouser -d monolith -c "\dt"
   ```
   Должны быть таблицы: `iss_fetch_log`, `osdr_items`, `space_cache`, `telemetry_legacy`

## Получение помощи

Если проблема не решена:

1. Соберите логи:
   ```bash
   docker-compose logs > logs.txt
   ```

2. Проверьте конфигурацию:
   ```bash
   docker-compose config > config.txt
   ```

3. Проверьте статус всех сервисов:
   ```bash
   docker-compose ps > status.txt
   ```


