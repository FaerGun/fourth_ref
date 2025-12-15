<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Services\AstronomyApiService;

class AstroController extends Controller
{
    private AstronomyApiService $service;

    public function __construct(AstronomyApiService $service)
    {
        $this->service = $service;
    }

    public function events(Request $request)
    {
        $data = $this->service->getEvents($request);
        
        if (isset($data['error'])) {
            return response()->json($data, 500);
        }
        
        return response()->json($data);
    }
}
