<?php

namespace App\Http\Controllers;

use App\Repositories\CmsRepository;

class CmsController extends Controller
{
    private CmsRepository $repository;

    public function __construct(CmsRepository $repository)
    {
        $this->repository = $repository;
    }

    public function page(string $slug)
    {
        // Санитизация входного параметра
        $slug = htmlspecialchars(trim($slug), ENT_QUOTES, 'UTF-8');
        
        $page = $this->repository->findBySlug($slug);
        if (!$page) {
            abort(404);
        }
        
        // Защита от XSS: экранируем HTML в title, body будет обработан в Blade
        return view('cms.page', [
            'title' => htmlspecialchars($page['title'], ENT_QUOTES, 'UTF-8'),
            'body' => $page['body'], // Будет экранировано в Blade через {!! !!} только если нужно
        ]);
    }
}
