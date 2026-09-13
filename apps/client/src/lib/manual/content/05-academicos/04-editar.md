## Datos editables

Desde la ficha, presiona el ícono de lápiz en la tarjeta azul. El diálogo **Editar académico** permite modificar:

- **Datos personales**: **Nombres**, **Apellido paterno**, **Apellido materno**, **Email**, **ORCID**, **Sexo**, **Fecha de nacimiento**, **Ciudad** y **Nacionalidad**.
- **Información laboral**: **Fecha de ingreso**, **Cargo**, **Departamento** y **Carrera**.
- **Categorización académica**: **Categoría**, **Opción**, **JCE** y **Horas descuento anual**.

Reglas:

- El **Email** y el **ORCID** deben ser únicos entre los académicos.
- La **Carrera** debe pertenecer al **Departamento** seleccionado; al cambiar de departamento la carrera se limpia. Puedes dejarla en **Sin carrera**.
- La **JCE** no puede superar la jornada máxima configurada.

El **RUT** no se edita: se define al crear el académico o mediante importación CSV.

<figure class="manual-figure">
  <img src="/manual/academicos-editar.webp" alt="Diálogo Editar académico con los campos modificables" />
  <figcaption>Edición de un académico.</figcaption>
</figure>

## Grados académicos

En la pestaña **Información Académica** se listan los grados por tipo: **Profesional**, **Magíster** y **Doctor**.

1. En el tipo que falte, presiona **Agregar**.
2. Completa **Nombre**, **Universidad**, **Fecha**, **Tipo** y **País**.
3. Presiona **Guardar**.

Para modificar uno existente, usa el lápiz sobre el grado.

Reglas: un académico puede tener un grado **Profesional** y **a lo más un grado superior** (Magíster **o** Doctor). Si ya tiene uno, el otro queda deshabilitado. Al editar, el **Tipo** no se puede cambiar.

## Edición delegada

La edición delegada permite que el **académico** mantenga sus datos sin entregarle acceso al panel.

### Enviar códigos (individual)

1. En la ficha, presiona **Enviar códigos de edición**.
2. Confirma con **Enviar código**.

Se envían por correo **códigos de 8 caracteres**, de un solo uso. Con ellos el académico puede editar su perfil y sus publicaciones. Cada código es **personal** y **temporal**: al usarlo, se invalida.

### Solicitar un enlace de edición (lado del académico)

Desde el perfil público del académico, la acción **Solicitar edición de perfil** pide el **Código de autorización** (8 caracteres). Si es válido, el sistema envía por correo un **enlace temporal** que:

- Es **personal** y de **un solo uso**: al guardar el perfil, el enlace deja de servir.
- Permite además **sincronizar publicaciones** y **ajustar publicaciones** (título, resumen, DOI, año, línea, indexación, autores y afiliaciones).

Un código inválido o expirado muestra **Código inválido. Verifica el código enviado a tu correo.**

> El envío masivo a todos los académicos se hace desde **Administración → [Configuración general](/admin/manual/administracion/configuracion-general)**.
