<?php

namespace App\Services;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

class AstronomyApiService
{
    private string $appId;
    private string $secret;

    public function __construct()
    {
        $this->appId = env('ASTRO_APP_ID', '');
        $this->secret = env('ASTRO_APP_SECRET', '');
    }

    public function getEvents(Request $request): array
    {
        // Валидация и санитизация входных параметров
        $lat = max(-90.0, min(90.0, (float)$request->query('lat', 55.7558)));
        $lon = max(-180.0, min(180.0, (float)$request->query('lon', 37.6176)));
        $days = max(1, min(30, (int)$request->query('days', 7)));

        if ($this->appId === '' || $this->secret === '') {
            Log::error('Missing ASTRO_APP_ID/ASTRO_APP_SECRET');
            return ['error' => 'Configuration error'];
        }

        $from = now('UTC')->toDateString();
        $to = now('UTC')->addDays($days)->toDateString();

        $auth = base64_encode($this->appId . ':' . $this->secret);
        
        try {
            $response = Http::timeout(25)
                ->withHeaders([
                    'Authorization' => 'Basic ' . $auth,
                    'Content-Type' => 'application/json',
                    'User-Agent' => 'monolith-iss/1.0'
                ])
                ->get('https://api.astronomyapi.com/api/v2/bodies/events', [
                    'latitude' => $lat,
                    'longitude' => $lon,
                    'from' => $from,
                    'to' => $to,
                ]);

            if ($response->successful()) {
                $data = $response->json() ?? [];
                // Нормализация ответа для фронтенда
                if (isset($data['data']['events'])) {
                    $events = [];
                    foreach ($data['data']['events'] as $event) {
                        $events[] = [
                            'name' => $event['name'] ?? 'Unknown',
                            'type' => $event['type'] ?? 'event',
                            'when' => $event['when'] ?? $event['date'] ?? '',
                            'extra' => $event['description'] ?? $event['details'] ?? '',
                        ];
                    }
                    return $events;
                }
                return $data;
            } else {
                $status = $response->status();
                $body = $response->body();
                Log::warning("Astronomy API error: HTTP {$status}, Response: {$body}");
                
                // Если 403, возможно проблема с credentials или endpoint недоступен на бесплатном тарифе
                if ($status === 403) {
                    // Пробуем альтернативный endpoint или возвращаем понятное сообщение
                    Log::warning("Astronomy API 403: App ID = " . substr($this->appId, 0, 8) . "...");
                    return [
                        'error' => 'API access denied',
                        'code' => 403,
                        'message' => 'Этот endpoint может быть недоступен на бесплатном тарифе AstronomyAPI. Проверьте ключи API или используйте другой endpoint.',
                        'hint' => 'Для работы нужны валидные ASTRO_APP_ID и ASTRO_APP_SECRET. Получите их на https://astronomyapi.com/'
                    ];
                }
                
                return [
                    'error' => 'API request failed',
                    'code' => $status,
                    'message' => $body ?: 'Unknown error'
                ];
            }
        } catch (\Exception $e) {
            Log::error("Astronomy API exception: " . $e->getMessage());
            return [
                'error' => 'Request failed',
                'message' => $e->getMessage()
            ];
        }
    }
}


