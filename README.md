# Plataforma de Visualización y Gestión de Investigación

Plataforma web para importar, clasificar y analizar la producción científica de la Facultad de Ingeniería de la UCT, reemplazando la gestión tradicional en planillas de cálculo. Permite la integración con ORCID y OpenAlex, así como la indexación de revistas WoS y Scopus.

## Taxonomía

Clasificación por jerarquía OpenAlex, mapeada a líneas de investigación institucionales:

```
Clasificación OpenAlex
│
└── Domain
    │
    └── Field
        │
        └── Subfield ─────── Línea de investigación institucional
            │
            └── Topic
```

**Líneas de investigación:** 

- Materiales Avanzados y Bioproductos
- Ciencias de la Tierra
- Sostenibilidad
- IA, Sistemas Complejos y Modelamiento Matemático 
- Educación en Ingeniería

## Stack

| Capa | Tecnología |
|------|------------|
| Servidor | Rust, Sword, sqlx |
| Cliente | SvelteKit + Tailwind v4, TanStack Query |
| BD | PostgreSQL |
| Infra | Docker Compose (server, client, postgres, nginx), GitHub Actions |

## Comandos

```bash
make run        # docker compose up
make lint       # cargo clippy + pnpm lint + check
make fmt        # cargo fmt + pnpm format
make migration name=x  # nueva migración sqlx
```
