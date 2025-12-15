<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Services\RustIssService;
use App\Services\OsdrDataTransformer;

class OsdrController extends Controller
{
    private RustIssService $rustIssService;
    private OsdrDataTransformer $transformer;

    public function __construct(RustIssService $rustIssService, OsdrDataTransformer $transformer)
    {
        $this->rustIssService = $rustIssService;
        $this->transformer = $transformer;
    }

    public function index(Request $request)
    {
        // Валидация и санитизация входных параметров
        $limit = max(1, min(100, (int)$request->query('limit', 20)));
        
        $items = $this->rustIssService->getOsdrList($limit);
        $items = $this->transformer->flatten($items);

        return view('osdr', [
            'items' => $items,
            'src' => (getenv('RUST_BASE') ?: 'http://rust_iss:3000') . '/osdr/list?limit=' . $limit,
        ]);
    }
}
