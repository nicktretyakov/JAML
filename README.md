# Solid SSG CMS

**Максимально сложный** демонстрационный проект на Rust:

- **SOLID** через трейты, DI (`Arc<dyn Trait>`), узкие интерфейсы
- **Параллелизм**: Rayon (data-parallel генерация страниц) + Tokio + DashMap
- **Static Site Generation** с Markdown → HTML + Tera-шаблоны
- **REST API** (Axum) для CRUD контента и триггера сборки
- **Frontend (JavaScript)** — админ-панель с отправкой форм и вызовом API
- **CMS** — раздача сгенерированного статического контента + API

## Архитектура (слои)

```
domain/          — сущности, value objects, DomainEvent (чистый бизнес)
core/            — трейты (ContentRepository, SiteBuilder, MarkdownProcessor…) + ошибки
infra/           — реализации (FileRepo + DashMap, Tera, Pulldown, FS writer, EventBus)
concurrency/     — ParallelPageGenerator (Rayon), AsyncWorkerPool
services/        — ContentService, ConcurrentBuildService (use-cases)
api/             — Axum handlers, DTO, routes, AppState (DI)
```

### SOLID в коде

| Принцип | Реализация |
|---------|------------|
| **S**ingle Responsibility | Отдельные сервисы, генератор, репозиторий, writer |
| **O**pen/Closed | Новые `ContentKind` / `Renderable` без изменения ядра |
| **L**iskov | Все `dyn Trait` взаимозаменяемы |
| **I**nterface Segregation | Узкие трейты вместо «бога» |
| **D**ependency Inversion | `main` собирает `Arc<dyn …>` и внедряет |

## Запуск

```bash
cargo run
```

После старта:

- Админ UI: http://localhost:3000/
- API health: http://localhost:3000/api/health
- Сгенерированный сайт: http://localhost:3000/site/
- Контент: `GET/POST /api/content`, `POST /api/build`

## Основные эндпоинты

| Method | Path | Описание |
|--------|------|----------|
| GET | `/api/health` | Healthcheck |
| GET | `/api/content` | Список контента |
| POST | `/api/content` | Создать |
| GET | `/api/content/:slug` | Получить |
| PUT | `/api/content/:id` | Обновить |
| POST | `/api/content/:id/publish` | Опубликовать |
| DELETE | `/api/content/:id` | Удалить |
| POST | `/api/build` | Параллельная генерация сайта |

## Frontend integration

Файл `static/js/app.js` экспортирует `window.SolidCMS` с методами `list/create/publish/build`.  
Админ-панель (`public/index.html`) использует его для отправки форм и запуска сборки.

## Параллельность

- `rayon::par_iter` по всем опубликованным `ContentItem`
- Количество потоков = `available_parallelism()` (или `SiteConfig.parallel_workers`)
- `DashMap` для lock-free concurrent repository
- Tokio runtime для API + async repository методов

## Расширение

1. Добавить новый `ContentKind` → реализовать `Renderable`
2. Заменить `ConcurrentFileRepository` на PostgreSQL (тот же трейт)
3. Подписаться на `DomainEvent` через `EventBus::subscribe`
4. Добавить WebSocket для live-прогресса сборки

MIT License
