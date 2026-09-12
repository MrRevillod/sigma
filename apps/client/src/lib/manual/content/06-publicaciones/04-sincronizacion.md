La sincronización consulta ORCID y OpenAlex para traer las publicaciones de un académico.

## Desde la ficha del académico

En la pestaña **Publicaciones**, presiona **Sincronizar Publicaciones**. Verás un resumen: **`Sincronización completa: N nuevas · M desvinculadas`** (o **parcial** si hubo errores).

Si el académico no tiene ORCID, el botón aparece deshabilitado como **Sin ORCID**.

<figure class="manual-figure">
  <img src="/manual/publicaciones-sync-academico.webp" alt="Diálogo de sincronización de publicaciones" />
  <figcaption>Sincronización de publicaciones de un Académico.</figcaption>
</figure>

## De todos los académicos

En el catálogo, presiona **Sincronizar publicaciones**. Se abre un diálogo que explica que el proceso puede tardar; al confirmar verás **Sincronización iniciada. Recibirás un correo cuando finalice.**

<figure class="manual-figure">
  <img src="/manual/publicaciones-sync.webp" alt="Diálogo de sincronización de publicaciones" />
  <figcaption>Sincronización de publicaciones.</figcaption>
</figure>

## Qué importa la sincronización

- Solo publicaciones de tipo **Artículo**.
- Solo las que tienen **DOI**.
- Solo las publicadas **desde la fecha de ingreso** del académico.
- Solo las que OpenAlex puede resolver por DOI.

Si una publicación ya existe (identificada por su ID de OpenAlex), no se duplica; se actualiza y se vuelve a vincular al académico. Además, se **desvinculan** las autorías del académico que ya no aparecen en ORCID.
