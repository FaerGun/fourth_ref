<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Services\RustIssService;
use App\Services\JwstService;

class DashboardController extends Controller
{
    private RustIssService $rustIssService;
    private JwstService $jwstService;

    public function __construct(RustIssService $rustIssService, JwstService $jwstService)
    {
        $this->rustIssService = $rustIssService;
        $this->jwstService = $jwstService;
    }

    public function index()
    {
        // Минимум: карта МКС и пустые контейнеры, JWST-галерея подтянется через /api/jwst/feed
        $iss = $this->rustIssService->getLast();
        $trend = []; // Фронт сам заберёт /api/iss/trend (через nginx прокси)

        return view('dashboard', [
            'iss' => $iss,
            'trend' => $trend,
            'jw_gallery' => [], // Не нужно сервером
            'jw_observation_raw' => [],
            'jw_observation_summary' => [],
            'jw_observation_images' => [],
            'jw_observation_files' => [],
            'metrics' => [
                'iss_speed' => $iss['payload']['velocity'] ?? null,
                'iss_alt' => $iss['payload']['altitude'] ?? null,
                'neo_total' => 0,
            ],
        ]);
    }

    /**
     * /api/jwst/feed — серверный прокси/нормализатор JWST картинок.
     */
    public function jwstFeed(Request $request)
    {
        $data = $this->jwstService->getFeed($request);
        return response()->json($data);
    }
}
