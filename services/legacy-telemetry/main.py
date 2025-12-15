#!/usr/bin/env python3
"""
Legacy Telemetry Service
Переписанный с Pascal на Python микросервис для генерации CSV и записи в БД.
Сохраняет тот же контракт: генерация telemetry_*.csv и COPY в PostgreSQL.
"""
import os
import sys
import time
import random
import csv
import logging
from datetime import datetime
from pathlib import Path
import psycopg2
from psycopg2.extras import execute_values

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout),
        logging.StreamHandler(sys.stderr)
    ]
)
logger = logging.getLogger(__name__)


def get_env(key: str, default: str) -> str:
    """Получить переменную окружения или значение по умолчанию"""
    return os.getenv(key, default)


def rand_float(min_v: float, max_v: float) -> float:
    """Генерировать случайное число в диапазоне"""
    return min_v + random.random() * (max_v - min_v)


def generate_and_copy():
    """
    Генерирует CSV файл с телеметрией и копирует его в PostgreSQL.
    Сохраняет тот же контракт что и оригинальный Pascal код.
    """
    out_dir = Path(get_env('CSV_OUT_DIR', '/data/csv'))
    out_dir.mkdir(parents=True, exist_ok=True)
    
    ts = datetime.now().strftime('%Y%m%d_%H%M%S')
    fn = f'telemetry_{ts}.csv'
    fullpath = out_dir / fn
    
    # Генерируем CSV
    try:
        with open(fullpath, 'w', newline='', encoding='utf-8') as f:
            writer = csv.writer(f)
            writer.writerow(['recorded_at', 'voltage', 'temp', 'source_file'])
            
            now_str = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
            voltage = round(rand_float(3.2, 12.6), 2)
            temp = round(rand_float(-50.0, 80.0), 2)
            writer.writerow([now_str, f'{voltage:.2f}', f'{temp:.2f}', fn])
        
        logger.info(f"Generated CSV: {fullpath}")
    except Exception as e:
        logger.error(f"Failed to write CSV: {e}", exc_info=True)
        return
    
    # COPY в PostgreSQL
    try:
        conn = psycopg2.connect(
            host=get_env('PGHOST', 'db'),
            port=get_env('PGPORT', '5432'),
            user=get_env('PGUSER', 'monouser'),
            password=get_env('PGPASSWORD', 'monopass'),
            database=get_env('PGDATABASE', 'monolith')
        )
        
        with conn.cursor() as cur:
            # Используем COPY FROM для эффективной загрузки
            with open(fullpath, 'r') as f:
                # Пропускаем заголовок
                next(f)
                cur.copy_from(
                    f,
                    'telemetry_legacy',
                    columns=('recorded_at', 'voltage', 'temp', 'source_file'),
                    sep=','
                )
        
        conn.commit()
        conn.close()
        logger.info(f"Copied {fn} to PostgreSQL")
    except Exception as e:
        logger.error(f"Failed to copy to PostgreSQL: {e}", exc_info=True)


def main():
    """Основной цикл работы сервиса"""
    random.seed()
    period = int(get_env('GEN_PERIOD_SEC', '300'))
    
    logger.info(f"Starting Legacy Telemetry Service (period={period}s)")
    
    while True:
        try:
            generate_and_copy()
        except Exception as e:
            logger.error(f"Legacy error: {e}", exc_info=True)
        
        time.sleep(period)


if __name__ == '__main__':
    main()



