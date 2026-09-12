Presiona **Crear académico**. El formulario **Nuevo académico** tiene los siguientes campos y reglas de validación:

| Campo                                                   | Reglas                                                    |
| ------------------------------------------------------- | --------------------------------------------------------- |
| **RUT**                                                 | Formato `XXXXXXXX-X`                                      |
| **Email**                                               | Debe ser un correo válido                                 |
| **Sexo**                                                | **Masculino (H)**, **Femenino (M)** u **Otro (O)**        |
| **Nombres**, **Apellido paterno**, **Apellido materno** | Entre 1 y 255 caracteres                                  |
| **Fecha de nacimiento**                                 | Formato `DD/MM/AAAA` (Día, Mes, Año)                      |
| **Fecha de ingreso UCT**                                | Formato `DD/MM/AAAA` (Día, Mes, Año)                      |
| **Departamento**                                        | Obligatorio                                               |
| **Cargo**                                               | Obligatorio (lista de cargos laborales)                   |
| **Carrera**                                             | Opcional                                                  |
| **Nacionalidad**                                        | Por defecto Chile                                         |
| **Categoría**                                           | Obligatoria                                               |
| **Opción**                                              | Se habilita al elegir categoría; incluye sus horas        |
| **JCE**                                                 | Entre 0 y la **JCE máxima** configurada, paso 0,25        |
| **Horas descuento anual**                               | Mayor o igual a 0                                         |
| **ORCID**                                               | Opcional; formato `https://orcid.org/0000-0000-0000-0000` |
| **Ciudad**                                              | Entre 1 y 255 caracteres                                  |

Usa **Limpiar Datos** para limpiar el formulario y **Crear** para guardar. Verás **Académico creado** y la tabla se actualizará.

> El sistema no permite RUT ni ORCID duplicados, ni una JCE superior al máximo configurado.

<figure class="manual-figure">
  <img src="/manual/academicos-crear.webp" alt="Formulario de creación de académico con todos sus campos" />
  <figcaption>Creación de académico.</figcaption>
</figure>
