@extends('layouts.app')
@section('content')
<div class="container my-3">
  <h3 class="mb-3">{{ $title }}</h3>
  {{-- Защита от XSS: используем {!! !!} только для доверенного контента, иначе {{ }} --}}
  {!! $body !!}
</div>
@endsection
