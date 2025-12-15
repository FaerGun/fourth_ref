<?php

namespace App\Http\Controllers;

use App\Services\RustIssService;

class IssController extends Controller
{
    private RustIssService $rustIssService;

    public function __construct(RustIssService $rustIssService)
    {
        $this->rustIssService = $rustIssService;
    }

    public function index()
    {
        $last = $this->rustIssService->getLast();
        $trend = $this->rustIssService->getTrend();

        return view('iss', [
            'last' => $last,
            'trend' => $trend,
            'base' => getenv('RUST_BASE') ?: 'http://rust_iss:3000'
        ]);
    }
}
