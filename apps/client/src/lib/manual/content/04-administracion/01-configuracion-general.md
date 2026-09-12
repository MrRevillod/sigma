La pestaña **General** reúne los ajustes de la jornada completa equivalente y el envío de códigos de edición.

## Jornada Completa Equivalente (JCE)

La JCE representa el tiempo de una jornada completa. Define el **máximo de horas** que se puede asignar a la carga de un académico. El valor se usa en la creación y edición de académicos y como denominador del indicador de **productividad por JCE**.

1. En **Configuración → General**, busca la tarjeta **Jornada Completa Equivalente (JCE)**.
2. Ajusta el campo **JCE máxima** (se admite decimal, paso de 0,25; por ejemplo, `42,5`).
3. Presiona **Guardar**. Verás la confirmación **Configuración actualizada**.

Reglas:

- El valor debe ser **mayor que 0**.
- Si un académico tiene una JCE superior al máximo, el sistema no permite guardarlo (verás un error del tipo "La JCE no puede superar...") hasta corregir el valor.
- **Cambiar la JCE máxima no re-escala** los registros existentes: solo se aplica a partir de ese momento y a las validaciones.

<figure class="manual-figure">
  <img src="/manual/config-general.webp" alt="Pestaña General con la JCE máxima y el envío masivo de códigos" />
  <figcaption>Configuración general.</figcaption>
</figure>

## Envío masivo de códigos de edición

Permite habilitar a **todos** los académicos para que mantengan su propio perfil.

Al enviar estos códigos, **todos** los académicos recibirán un correo electrónico con una serie de códigos alfanuméricos que les permitirá acceder a su perfil y editarlo. Cada código es **válido una sola vez**; al usarlo, se invalida y el académico debe usar otro código de la lista.

1. Presiona **Enviar códigos a todos**.
2. En el diálogo **Enviar códigos de edición masivo**, revisa la explicación y confirma con **Enviar códigos**.
3. Al terminar verás **Códigos enviados a N académicos**.

Para enviar códigos a **un solo** académico, usa el botón **Enviar códigos de edición** en su ficha (ver [Edición delegada](/admin/manual/academicos/editar)).
