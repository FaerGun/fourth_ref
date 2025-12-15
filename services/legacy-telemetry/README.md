# Legacy Telemetry Service

Микросервис для генерации CSV файлов с телеметрией и записи их в PostgreSQL.

## Описание

Это переписанная версия оригинального Pascal legacy-модуля на Python. Сохраняет тот же контракт:
- Генерация CSV файлов с форматом: `recorded_at,voltage,temp,source_file`
- Запись данных в таблицу `telemetry_legacy` через PostgreSQL COPY

## Переменные окружения

- `CSV_OUT_DIR` - директория для сохранения CSV (по умолчанию: `/data/csv`)
- `GEN_PERIOD_SEC` - период генерации в секундах (по умолчанию: 300)
- `PGHOST` - хост PostgreSQL (по умолчанию: `db`)
- `PGPORT` - порт PostgreSQL (по умолчанию: `5432`)
- `PGUSER` - пользователь PostgreSQL (по умолчанию: `monouser`)
- `PGPASSWORD` - пароль PostgreSQL (по умолчанию: `monopass`)
- `PGDATABASE` - база данных (по умолчанию: `monolith`)

## Формат CSV

```csv
recorded_at,voltage,temp,source_file
2024-01-01 12:00:00,7.50,25.30,telemetry_20240101_120000.csv
```

## Логирование

Логи пишутся в stdout и stderr в формате:
```
2024-01-01 12:00:00 - __main__ - INFO - Generated CSV: /data/csv/telemetry_20240101_120000.csv
```

## Cron (опционально)

Для запуска по расписанию можно использовать cron внутри контейнера или внешний cron:

```cron
*/5 * * * * /entrypoint.sh
```



