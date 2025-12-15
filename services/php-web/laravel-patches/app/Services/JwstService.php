<?php

namespace App\Services;

use App\Support\JwstHelper;
use Illuminate\Http\Request;

class JwstService
{
    private JwstHelper $helper;

    public function __construct(JwstHelper $helper)
    {
        $this->helper = $helper;
    }

    public function getFeed(Request $request): array
    {
        $src = $request->query('source', 'jpg');
        $sfx = trim((string)$request->query('suffix', ''));
        $prog = trim((string)$request->query('program', ''));
        $instF = strtoupper(trim((string)$request->query('instrument', '')));
        $page = max(1, (int)$request->query('page', 1));
        $per = max(1, min(60, (int)$request->query('perPage', 24)));

        // Выбираем эндпоинт
        $path = 'all/type/jpg';
        if ($src === 'suffix' && $sfx !== '') {
            $path = 'all/suffix/' . ltrim($sfx, '/');
        }
        if ($src === 'program' && $prog !== '') {
            $path = 'program/id/' . rawurlencode($prog);
        }

        $resp = $this->helper->get($path, ['page' => $page, 'perPage' => $per]);
        $list = $resp['body'] ?? ($resp['data'] ?? (is_array($resp) ? $resp : []));

        $items = [];
        foreach ($list as $it) {
            if (!is_array($it)) {
                continue;
            }

            // Выбираем валидную картинку
            $url = null;
            $loc = $it['location'] ?? $it['url'] ?? null;
            $thumb = $it['thumbnail'] ?? null;
            foreach ([$loc, $thumb] as $u) {
                if (is_string($u) && preg_match('~\.(jpg|jpeg|png)(\?.*)?$~i', $u)) {
                    $url = $u;
                    break;
                }
            }
            if (!$url) {
                $url = JwstHelper::pickImageUrl($it);
            }
            if (!$url) {
                continue;
            }

            // Фильтр по инструменту
            $instList = [];
            foreach (($it['details']['instruments'] ?? []) as $I) {
                if (is_array($I) && !empty($I['instrument'])) {
                    $instList[] = strtoupper($I['instrument']);
                }
            }
            if ($instF && $instList && !in_array($instF, $instList, true)) {
                continue;
            }

            $items[] = [
                'url' => $url,
                'obs' => (string)($it['observation_id'] ?? $it['observationId'] ?? ''),
                'program' => (string)($it['program'] ?? ''),
                'suffix' => (string)($it['details']['suffix'] ?? $it['suffix'] ?? ''),
                'inst' => $instList,
                'caption' => trim(
                    (($it['observation_id'] ?? '') ?: ($it['id'] ?? '')) .
                    ' · P' . ($it['program'] ?? '-') .
                    (($it['details']['suffix'] ?? '') ? ' · ' . $it['details']['suffix'] : '') .
                    ($instList ? ' · ' . implode('/', $instList) : '')
                ),
                'link' => $loc ?: $url,
            ];
            if (count($items) >= $per) {
                break;
            }
        }

        return [
            'source' => $path,
            'count' => count($items),
            'items' => $items,
        ];
    }
}



