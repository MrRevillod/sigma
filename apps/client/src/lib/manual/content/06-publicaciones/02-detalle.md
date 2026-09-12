El detalle muestra:

- Tipo de publicación, estado (**Aceptado** / **Publicado**) y fecha.
- Título y enlaces **Ver DOI** y **Ver en OpenAlex**.
- **Abstract**, con la marca **(editado)** si fue modificado manualmente.
- **Autores**, con insignias **Correspondiente** y **Externo**, enlace al perfil y afiliaciones.
- **Publicado en**: medio e **Indexación** (o **Sin indexar**).
- **Línea de investigación**.
- **Topics** y **Keywords** detectados por OpenAlex, con su puntuación.

<figure class="manual-figure">
  <img src="/manual/publicaciones-detalle.webp" alt="Detalle de una publicación con sus secciones y enlaces externos" />
  <figcaption>Detalle de una publicación.</figcaption>
</figure>

## Autores

Cada autor puede mostrar las siguientes insignias:

- **Correspondiente**: el autor responsable de la comunicación de la publicación (solo uno).
- **Externo**: un autor que **no** está registrado como académico en la plataforma.

Un autor es **local** cuando su ORCID corresponde a un académico registrado; en ese caso muestra el enlace **Perfil →** a su ficha. Los autores **externos** no tienen ficha: solo se muestra su nombre, su ORCID y sus afiliaciones.

## Topics y taxonomía

OpenAlex clasifica el conocimiento en cuatro niveles:

- **Dominio** → **Campo** → **Subcampo** → **Tema**.

Cada **topic** de la publicación tiene una **puntuación** y, al desplegarlo, se muestra a qué dominio, campo y subcampo pertenece. Las **keywords** son palabras clave extraídas de la publicación.

## Línea de investigación

La línea se asigna automáticamente desde el **subcampo del topic con mayor puntuación**. Si la publicación no tiene topics reconocidos, queda **sin asignar**. Desde el editor se puede forzar una línea manual, que **tiene prioridad** sobre la asignación automática.
