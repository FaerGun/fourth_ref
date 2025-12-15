<?php

namespace App\Services;

use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

class RustIssService
{
    private string $baseUrl;

    public function __construct()
    {
        $this->baseUrl = getenv('RUST_BASE') ?: 'http://rust_iss:3000';
    }

    public function getLast(): array
    {
        try {
            $response = Http::timeout(5)
                ->get("{$this->baseUrl}/last");
            
            if ($response->successful()) {
                $data = $response->json();
                // Rust сервис возвращает { "ok": true, "data": {...} }
                if (isset($data['ok']) && $data['ok'] && isset($data['data'])) {
                    return $data['data'];
                }
                // Fallback для старого формата
                return $data['data'] ?? $data ?? [];
            }
        } catch (\Exception $e) {
            Log::warning("Rust ISS service error: " . $e->getMessage());
        }
        
        return [];
    }

    public function getTrend(): array
    {
        try {
            $response = Http::timeout(5)
                ->get("{$this->baseUrl}/iss/trend");
            
            if ($response->successful()) {
                $data = $response->json();
                // Rust сервис возвращает { "ok": true, "data": {...} }
                if (isset($data['ok']) && $data['ok'] && isset($data['data'])) {
                    return $data['data'];
                }
                // Fallback для старого формата
                return $data['data'] ?? $data ?? [];
            }
        } catch (\Exception $e) {
            Log::warning("Rust ISS trend error: " . $e->getMessage());
        }
        
        return [];
    }

    public function getOsdrList(int $limit = 20): array
    {
        try {
            $response = Http::timeout(5)
                ->get("{$this->baseUrl}/osdr/list", ['limit' => $limit]);
            
            if ($response->successful()) {
                $data = $response->json();
                return $data['data']['items'] ?? $data['items'] ?? [];
            }
        } catch (\Exception $e) {
            Log::warning("Rust OSDR list error: " . $e->getMessage());
        }
        
        return [];
    }
}


