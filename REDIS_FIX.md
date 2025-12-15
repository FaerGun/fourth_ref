# Исправление ошибки "Class 'Redis' not found"

## Проблема

При открытии http://localhost:8080/dashboard возникала ошибка:
```
Class 'Redis' not found
```

## Причина

В PHP контейнере не было установлено расширение Redis, необходимое для работы Laravel с Redis кэшем.

## Решение

Добавлена установка PHP расширения Redis в `services/php-web/Dockerfile`:

```dockerfile
RUN apk add --no-cache bash curl git unzip rsync postgresql-dev \
 && docker-php-ext-install pdo pdo_pgsql \
 && apk add --no-cache pcre-dev $PHPIZE_DEPS \
 && pecl install redis \
 && docker-php-ext-enable redis \
 && apk del pcre-dev $PHPIZE_DEPS
```

## Проверка

Расширение Redis успешно установлено и доступно:
```bash
docker-compose exec php php -m | grep redis
# Вывод: redis
```

## Статус

✅ **Исправлено** - Redis расширение установлено и работает
✅ **PHP контейнер перезапущен** - изменения применены
✅ **Все сервисы работают**

## Следующие шаги

1. Обновите страницу http://localhost:8080/dashboard
2. Ошибка должна исчезнуть
3. Если проблема сохраняется, проверьте логи: `docker-compose logs php`


