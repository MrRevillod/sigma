Este manual describe cómo operar el **Sistema de Gestión Académica** de la Facultad de Ingeniería de la Universidad Católica de Temuco. Está dirigido a las personas con acceso de administración.

## Qué es el sistema

Es una plataforma web interna que centraliza:

- **Académicos** de la facultad: datos personales, información laboral, categorización y grados académicos.
- **Publicaciones** científicas, importadas automáticamente desde **ORCID** y enriquecidas con **OpenAlex**.
- **Indicadores** de productividad, colaboración e indexación por facultad, departamento y línea de investigación.
- **Administración** de catálogos, líneas de investigación y usuarios.

El sitio tiene dos caras: una **pública** (cualquier persona puede ver académicos, publicaciones y estadísticas) y una **privada** (requiere iniciar sesión y permite crear, editar y administrar).

## Roles y permisos

La plataforma ofrece un único rol: **Administrador**. Cualquier persona con una cuenta puede acceder a todas las funciones de administración una vez que inicia sesión. Las cuentas se gestionan en la sección **Usuarios**. Es por esta característica que la plataforma ofrece una vista pública pensada para cualquier persona pero con limites de edición, y una vista privada para quienes tienen acceso de administración.

## De dónde vienen los datos

| Dato                          | Origen                                                     |
| ----------------------------- | ---------------------------------------------------------- |
| Perfil del académico          | Ingreso manual o importación CSV                           |
| Publicaciones                 | Sincronización automática desde ORCID + OpenAlex           |
| Indexación (WoS / Scopus)     | Automática por ISSN del medio; se puede forzar manualmente |
| Líneas de investigación       | Clasificación de tópicos de OpenAlex + ajustes manuales    |
| Categorías, opciones y cargos | Catálogos internos gestionados en Administración           |

## Consideraciones generales

- Las acciones destructivas (por ejemplo, **Desvincular académico** o **Eliminar usuario**) se confirman en un diálogo y, en general, **no se pueden deshacer**.
- Los cambios en publicaciones se guardan como **ajustes manuales** que sobrescriben lo importado; siempre es posible **Restaurar originales**.
- La gestión de datos sensibles (cuentas, contraseñas, perfiles) debe hacerse con cuidado; no compartas credenciales.
