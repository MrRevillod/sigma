El botón **Importar CSV** permite cargar o actualizar académicos de forma masiva. Se abre el selector de archivos y se procesa el documento.

Reglas de procesamiento:

- La primera fila debe contener los **encabezados exactos** (en mayúsculas y con los mismos nombres).
- Las filas sin contenido en la primera columna se omiten.
- Un académico existente se busca por **RUT** y, si no coincide, por **email**.
- Si existe, se **actualiza** y se **reemplazan sus grados** por los del archivo.
- Si no existe, se **crea**.
- Se valida que los nombres de departamento, carrera, cargo, categoría y país existan; los valores que no calcen generan error de fila.
- La planta de la categoría debe coincidir con la columna **PLANTA**, y las horas de la opción con **HRS DD CATEGORIA/OPCION** cuando correspondan.
- La JCE no puede superar el máximo configurado.

Al terminar se muestra **`N importados · M actualizados · K con errores`**. Si hubo errores, se abre el diálogo **Resultado de la importación**, que lista cada **Fila** con sus motivos. Las filas con error **no se procesan**; corrige el archivo y vuelve a intentarlo.

<details class="manual-disclosure">
<summary>Ver columnas y valores aceptados</summary>

El archivo tiene una fila de **encabezados** y, debajo, una fila por académico. **No aplica** indica que el campo es de texto libre.

**Datos personales y de contacto**

| Columna                | Valores aceptados                           |
| ---------------------- | ------------------------------------------- |
| `RUT`                  | No aplica                                   |
| `NOMBRES`              | No aplica                                   |
| `APELLIDO PATERNO`     | No aplica                                   |
| `APELLIDO MATERNO`     | No aplica                                   |
| `CORREO`               | No aplica                                   |
| `ORCID`                | No aplica                                   |
| `SEXO`                 | `H` (Masculino), `M` (Femenino), `O` (Otro) |
| `FECHA DE NACIMIENTO`  | Fecha `AAAA-MM-DD`                          |
| `PAIS DE NACIONALIDAD` | Código o nombre del país                    |
| `CIUDAD`               | No aplica                                   |

**Datos laborales**

| Columna                   | Valores aceptados           |
| ------------------------- | --------------------------- |
| `FECHA DE INGRESO`        | Fecha `AAAA-MM-DD`          |
| `CARGO`                   | No aplica                   |
| `DEPARTAMENTO`            | No aplica                   |
| `CARRERA`                 | No aplica                   |
| `PLANTA`                  | `Adjunta`, `Permanente`     |
| `CATEGORIA`               | No aplica                   |
| `OPCION`                  | `Docencia`, `Investigación` |
| `JCE`                     | Número                      |
| `HRS DD CATEGORIA/OPCION` | Número                      |
| `HRS DD DESC PROM ANUAL`  | Número                      |

**Grados (opcional)**

| Columna              | Valores aceptados        |
| -------------------- | ------------------------ |
| `TITULO PROFESIONAL` | No aplica                |
| `UNIVERSIDAD (I)`    | No aplica                |
| `FECHA (I)`          | Fecha `AAAA-MM-DD`       |
| `PAIS (I)`           | Código o nombre del país |
| `GRADO ACADEMICO`    | No aplica                |
| `UNIVERSIDAD (II)`   | No aplica                |
| `FECHA (II)`         | Fecha `AAAA-MM-DD`       |
| `PAIS (II)`          | Código o nombre del país |
| `TIPO DE GRADO`      | `Magister`, `Doctor`     |

</details>
