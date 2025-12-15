<?php

namespace App\Repositories;

use Illuminate\Support\Facades\DB;

class CmsRepository
{
    public function findBySlug(string $slug): ?array
    {
        $page = DB::table('cms_pages')
            ->where('slug', $slug)
            ->first();
        
        return $page ? (array)$page : null;
    }
}



