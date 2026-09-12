La ficha reúne toda la información de un académico. Tiene una **tarjeta azul lateral** y **cuatro pestañas**.

## Tarjeta lateral

Muestra los datos básicos (nacionalidad, ciudad de residencia, fecha de nacimiento, sexo y, si existe, el enlace a su ORCID) y tres acciones:

- **Editar** (lápiz): abre el diálogo para modificar sus datos.
- **Enviar códigos de edición**: habilita al académico para editar su propio perfil. Ver [Edición delegada](/admin/manual/academicos/editar).
- **Desvincular académico**: retira al académico de la universidad. Ver [Desvincular](/admin/manual/academicos/desvincular).

<figure class="manual-figure">
  <img src="/manual/academicos-detalle-informacion.webp" alt="Ficha de académico en la pestaña Información Académica" />
  <figcaption>Ficha del académico.</figcaption>
</figure>

## Pestañas

### Información Académica

Muestra su información laboral (departamento, carrera, ingreso, cargo y JCE), su categorización (planta, categoría, opción, horas y descuento anual) y sus **grados académicos**. Para agregar o editar un grado, ver [Grados académicos](/admin/manual/academicos/editar).

### Publicaciones

Lista las publicaciones del académico, con filtros por línea, indexación y rango de años. Si el académico tiene ORCID, permite **Sincronizar Publicaciones**. Ver [Publicaciones](/admin/manual/publicaciones/catalogo).

### Estadísticas

Muestra el desempeño del académico. Arriba hay un **Rango anual de publicación** y, debajo, tres bloques desplegables:

- **Líneas de investigación**: distribución de sus publicaciones por línea, con barras y un gráfico de radar.
- **Tendencia anual de publicaciones**: evolución por año, según la indexación (WoS / Scopus).
- **Impacto y Desempeño**: compara sus publicaciones con las de su **facultad**, **departamento** y **línea** mediante gráficos de anillo.

<figure class="manual-figure">
  <img src="/manual/academicos-estadisticas.webp" alt="Pestaña Estadísticas de la ficha del académico" />
  <figcaption>Estadísticas del académico.</figcaption>
</figure>

### Colaboraciones

Muestra la **red de colaboración** del académico. El botón de información de la vista abre esta misma explicación.

**Nodos**

- **Este académico**: el foco de la vista.
- **Coautor**: alguien con quien comparte publicaciones; el tamaño del nodo refleja cuántas tiene.
- **Posible colaboración**: un académico sin publicaciones en común que comparte intereses de investigación.

**Conexiones**

- **Coautoría** (azul): publicaciones compartidas; el grosor indica cuántas.
- **Posible colaboración** (verde): intereses en común; el grosor indica cuántas publicaciones cumplen el mínimo de coincidencias.
- Con **Peso en aristas** activado, se muestra el número exacto sobre cada conexión.

**En qué se basan las coincidencias**

Dos académicos coinciden cuando comparten **topics** o **keywords** con el porcentaje mínimo requerido:

- **topic**: área temática de OpenAlex asignada a una publicación, agrupada en subcampo y línea de investigación.
- **keyword**: palabra clave extraída de la publicación.

**Ajustes del grafo**

- **Colaboraciones**: muestra u oculta las coautorías.
- **Sugerencias**: muestra u oculta los posibles colaboradores.
- **Peso en aristas**: muestra el número de publicaciones sobre cada conexión.
- **Separación**: distancia entre los nodos.
- **Zoom**: acerca o aleja la vista.
- **Porcentaje de coincidencia ≥**: porcentaje mínimo para que un topic o keyword cuente como coincidencia.
- **Coincidencias por publicación ≥**: mínimo de topics o keywords compartidos en una publicación para que cuente; el peso de una sugerencia es el número de publicaciones que cumplen esta condición.

Haz clic en una conexión para ver las **publicaciones compartidas** o los temas en común.

<figure class="manual-figure">
  <img src="/manual/academicos-colaboraciones.webp" alt="Pestaña Colaboraciones con la red de coautorías" />
  <figcaption>Red de colaboración.</figcaption>
</figure>
