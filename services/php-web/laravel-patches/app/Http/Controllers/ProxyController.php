<?php

namespace App\Http\Controllers;

use Illuminate\Http\Response;
use Illuminate\Support\Facades\Http;
use App\Services\RustIssService;

class ProxyController extends Controller
{
    private RustIssService $rustIssService;

    public function __construct(RustIssService $rustIssService)
    {
        $this->rustIssService = $rustIssService;
    }

    public function last()
    {
        $data = $this->rustIssService->getLast();
        return response()->json($data, 200);
    }

    public function trend()
    {
        $data = $this->rustIssService->getTrend();
        return response()->json($data, 200);
    }
}
