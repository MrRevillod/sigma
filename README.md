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

## Self-hosting

La imagen cliente publicada en GHCR no trae ninguna URL de API fija. El contenedor cliente corre nginx, sirve la SPA y **proxya `/api` al backend**, de modo que el navegador solo habla con el dominio del cliente (same-origin): sin CORS y con cookies first-party.

```
Navegador → https://<dominio-cliente>/api → nginx cliente → ${API_URL} → API
```

### Variables

Contenedor **cliente**:

| Variable | Descripción |
|----------|-------------|
| `API_URL` | URL del API, **solo `esquema://host[:puerto]`, sin path ni slash final** (ej. `https://api.mi-uni.cl`). El path `/api` lo aporta la petición. |

Contenedor **server**:

| Variable | Descripción |
|----------|-------------|
| `FRONTEND_URL` | URL pública del cliente (se usa en los enlaces de recuperación de contraseña). |
| `CORS_ALLOW_ORIGINS` | Orígenes CORS como **array TOML** (ej. `["https://app.mi-uni.cl"]`). Déjalo `[]` si todos los clientes pasan por el proxy. |
| `JWT_SECRET`, `POSTGRES_*`, `SMTP_*`, `SEED_ADMIN_*` | Credenciales y configuración del server. |

### Ejemplo

```bash
docker run -d --name acad-mgr-client \
  -e API_URL=https://api.mi-uni.cl \
  -p 8080:80 \
  ghcr.io/<owner>/academic-mgr-client:latest
```

### Requisitos y notas

- **TLS**: las cookies usan `Secure` + `SameSite=Strict`; termina TLS en tu ingress/borde (certbot, Traefik, LB). Sin HTTPS el login no funciona.
- **Rate limiting y cabeceras de seguridad**: vienen en el nginx del cliente (`config/nginx.prod.conf.template`, `config/nginx-security-headers.conf`). Ajusta los límites o muévelos a tu ingress si lo prefieres.
- **CORS**: solo se necesita si algún navegador consume el API **directo** desde otro origen; en ese caso agrega ese origen en `CORS_ALLOW_ORIGINS`. Nunca uses `*` con credenciales.
- **Recomendado**: no expongas el API públicamente; que solo sea alcanzable por el nginx del cliente dentro de la red interna.
