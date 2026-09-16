#set document(title: "Sistema de Gestión y Métricas Académicas")
#set page(paper: "a4", margin: (x: 2.5cm, y: 2.2cm), numbering: "1")
#set text(font: "P052", size: 10.5pt, lang: "es")
#set par(justify: true, leading: 0.72em)
#set heading(numbering: "1.1.")

#show heading.where(level: 1): set text(size: 13pt, weight: "bold")
#show heading.where(level: 1): set block(above: 2em, below: 1em)

#show heading.where(level: 2): set text(size: 11.5pt, weight: "bold")
#show heading.where(level: 2): set block(above: 2em, below: 1em)

#show heading.where(level: 3): set text(size: 11.5pt, weight: "bold")
#show heading.where(level: 3): set block(above: 2em, below: 1em)

#let report-head(
  title,
  subtitle,
  name: "Luciano Ignacio Revillod Jeréz",
  email: "lrevillod2022@alu.uct.cl",
  show-line: true,
) = {
  align(center)[
    #v(-2em)
    #image("logo.png", width: 50%)
    #text(font: "P052", size: 18pt, weight: "bold")[#title]
    #v(-0.1cm)
    #text(font: "P052", size: 11pt, style: "italic", fill: luma(100))[#subtitle]
    #v(8pt)
    #name
    #linebreak()
    #email
    #linebreak()
    Estudiante de Ingeniería Civil en Informática, Universidad Católica de Temuco
    #linebreak()
    #if show-line {
      v(6pt)
      line(length: 100%)
    }
  ]
}

#report-head(
  "Sistema de Gestión y Métricas Académicas",
  "Reporte de Proyecto y Proyección futura",
  show-line: true,
)

#v(6pt)


#heading(numbering: none)[Introducción]

Este documento presenta la plataforma desarrollada para la gestión y visualización de la producción científica de la Facultad de Ingeniería de la Universidad Católica de Temuco. El proyecto surge a partir de la necesidad de contar con un sistema centralizado para organizar la información de los académicos y sus publicaciones, reemplazando parte de la gestión que anteriormente se realizaba mediante planillas Excel.

La plataforma integra información académica con datos obtenidos desde fuentes externas, principalmente ORCID y OpenAlex. A partir de estos datos se implementaron procesos para importar y actualizar publicaciones, clasificarlas según las líneas de investigación de la facultad, determinar su indexación y generar indicadores de producción científica. Se incorporó información sobre las relaciones de coautoría para analizar las colaboraciones entre académicos y generar recomendaciones de posibles colaboradores.

El sistema está operativo para la Facultad de Ingeniería y ofrece una vista pública para consultar la información y una vista administrativa para gestionarla. Las vistas dan acceso a las funcionalidades desarrolladas; el trabajo principal de la plataforma es procesar y organizar los datos.

El estado actual del sistema y las principales funcionalidades implementadas se describen a continuación. Posteriormente se presentan las líneas de desarrollo que se han identificado para continuar ampliando la plataforma.


= El sistema actual

== Contexto institucional

El sistema está pensado para una sola unidad académica: la Facultad de Ingeniería de la UCT. La estructura de la facultad es la siguiente:

- 4 departamentos: Ciencias Matemáticas y Físicas; Obras Civiles y Geología; Procesos Industriales; Ingeniería Informática.
- 10 carreras distribuidas entre esos departamentos

El diseño del modelo de datos refleja esta jerarquía institucional de forma explícita. La organización se representa con una cadena facultad → departamento → carrera, a la que se suman dos dimensiones propias del ámbito académico: el cargo laboral del académico y su categorización académica (planta permanente o adjunta, con una opción de docencia o investigación y horas asociadas).

#pagebreak()

== Capacidades del sistema

La información de los académicos y su producción científica se reúne en un mismo lugar. Las principales funcionalidades están orientadas a la gestión de los datos académicos, la importación y clasificación de publicaciones, el análisis de la producción científica y la visualización de las colaboraciones entre investigadores.

=== Gestión de académicos

Los académicos de la facultad quedan registrados en un catálogo central. Contiene información personal, institucional y académica:

Identificación: RUT, nombres y apellidos, correo institucional y ORCID.
Datos personales: sexo, fecha de nacimiento, nacionalidad y ciudad.
Datos laborales: cargo, departamento, carrera y JCE (Jornada Completa Equivalente), expresada en horas.
Categorización académica: planta, categoría, opción de docencia o investigación, horas asociadas a la categoría y horas de descuento anual.
Grados académicos: información de los grados profesional, magíster y doctor, incluyendo institución, país y fecha de obtención.

Para evitar inconsistencias, la creación y edición de estos registros cuenta con validaciones. Se verifica el formato del RUT y ORCID, la relación entre carrera y departamento y los límites configurados para la JCE. El sistema evita registrar dos veces a un académico mediante el RUT o el ORCID.

=== Importación de datos académicos desde CSV

Para la carga inicial y actualización de académicos se implementó una importación mediante archivos CSV. Cada fila es validada de forma independiente antes de ser almacenada.

Durante este proceso se comprueba el formato de los datos y las reglas de negocio correspondientes. Las referencias a departamentos, carreras, cargos y categorías se resuelven utilizando los catálogos existentes y se valida que los datos sean consistentes entre sí.

Las filas válidas se almacenan mediante una transacción y las que presentan errores se informan por separado. Al finalizar la importación, el sistema entrega un resumen con los registros creados, actualizados y los errores encontrados en cada fila.

La misma funcionalidad permite actualizar información existente. Si el sistema encuentra un académico mediante su RUT o correo, sus datos se actualizan en lugar de crear un nuevo registro. Los grados académicos también se actualizan durante este proceso.

=== Sincronización de publicaciones

Las publicaciones de los académicos se obtienen automáticamente desde fuentes externas. ORCID se utiliza como punto de partida para identificar las obras asociadas a cada académico y OpenAlex permite completar sus metadatos.

De cada publicación se obtiene información como el título, DOI, fecha, año, idioma, estado, revista, autores y afiliaciones. Se incorpora la información de tópicos, palabras clave e ISSN.

A partir de los ORCID de los autores, el sistema distingue entre académicos de la facultad y autores externos. Esta distinción se utiliza posteriormente para representar las colaboraciones científicas.

La sincronización puede ejecutarse para un académico específico o para todos los académicos de la facultad. El proceso es idempotente, es por esto que una publicación que ya existe se actualiza en lugar de registrarse nuevamente. Al finalizar se informa el resultado del proceso, incluyendo publicaciones creadas, autores enlazados, tópicos y palabras clave asociados, además de las obras que no pudieron ser incorporadas.

Las fuentes utilizadas imponen algunas restricciones a la importación. Solo se consideran publicaciones visibles en ORCID, con DOI y que estén disponibles en OpenAlex. El alcance actual considera únicamente artículos, dejando fuera libros, capítulos y otros tipos de obra.

Las modificaciones realizadas manualmente sobre una publicación se almacenan por separado de los datos obtenidos desde las fuentes externas. Una nueva sincronización puede actualizar los metadatos originales sin perder las correcciones realizadas desde la plataforma. Si una publicación deja de aparecer en ORCID, esta se desvincula del académico correspondiente.

=== Clasificación institucional de las publicaciones

La facultad cuenta con una clasificación propia para organizar las publicaciones según sus líneas de investigación. Estas líneas complementan la clasificación temática de las publicaciones.

Las líneas de investigación institucionales se administran y se relacionan con los subcampos temáticos correspondientes. La clasificación utilizada por la plataforma sigue la estructura:

#image("openalex-hierarchy.png", width: 100%)

Los subcampos pueden ser asignados visualmente a las líneas de investigación mediante la administración del sistema. A partir de esta relación, las publicaciones pueden quedar asociadas a una línea institucional.

La asignación de la línea de investigación se realiza de forma automática. En el caso que exista una clasificación indicada manualmente, esta tiene prioridad. En los demás casos se utiliza la clasificación temática disponible y el tópico de mayor relevancia para realizar la inferencia. Si no existe una coincidencia, la publicación queda como "Sin asignar".

=== Indexación en WoS y Scopus

Para cada publicación, la plataforma determina si la revista se encuentra indexada en Web of Science (WoS) o Scopus. Para esto se utilizan tablas de ISSN que permiten relacionar cada revista con su correspondiente indexación.

La información se incorpora a las publicaciones y también a las estadísticas de la plataforma. Esto permite filtrar y comparar la producción según su indexación.

La clasificación presenta una limitación en relación a la disponibilidad de información actualizada. Para realizarla se utilizaron bases de datos públicas de años anteriores y búsquedas manuales, debido a que las fuentes oficiales y actualizadas de estas indexaciones requieren acceso a información de pago.

=== Estadísticas y dashboards

La producción científica se analiza en distintas vistas según el nivel de la organización. A nivel de facultad se muestran indicadores como el total de publicaciones, distribución por año, publicaciones WoS y Scopus, distribución por departamento y línea de investigación y ranking de académicos.

A nivel de departamento y línea de investigación se presentan resúmenes de producción, tendencias y rankings. Los perfiles de los académicos incluyen su distribución por líneas de investigación, línea dominante, evolución anual y contribución relativa dentro de su facultad, departamento y línea.

Las vistas pueden filtrarse por rango de años, departamento e indexación. Así mismo, se incorporan indicadores de productividad relacionados con la JCE, que permiten comparar la producción considerando las horas de jornada de los académicos incluidos en el análisis.

El indicador de productividad se calcula relacionando las publicaciones de cada año con la jornada en horas de los académicos considerados. Se entregan resultados para la producción total, WoS y Scopus. El análisis puede filtrarse por grado académico y utilizar distintos grupos de académicos como denominador.

En el caso de las líneas de investigación, el cálculo considera la línea dominante de cada académico y muestra la jornada acumulada y la cantidad de académicos incluidos en cada resultado.

=== Red de colaboraciones y recomendaciones

Las colaboraciones científicas se representan mediante un grafo. Los nodos corresponden a académicos de la facultad y las relaciones representan coautorías entre ellos. El peso de cada relación depende de la cantidad de publicaciones compartidas.

A partir de esta red se generan recomendaciones de posibles colaboradores. Para esto se buscan académicos que no formen parte de la red directa del investigador consultado y que tengan afinidad con sus áreas de trabajo.

La afinidad se calcula utilizando la información temática disponible en las publicaciones, considerando tópicos, palabras clave y líneas de investigación. El umbral utilizado para determinar una coincidencia puede ser configurado en la plataforma.

=== Vistas públicas y administrativas

El sistema tiene dos espacios principales. La vista pública permite consultar un directorio de los académicos de la facultad y acceder a sus perfiles sin iniciar sesión. Estos perfiles incluyen información sobre publicaciones, estadísticas y red de colaboración.

La vista administrativa permite gestionar los datos utilizados por la plataforma. Desde ella se pueden administrar académicos, categorías, opciones, cargos, líneas de investigación, publicaciones y usuarios.

Las funciones administrativas están protegidas mediante inicio de sesión con correo y contraseña.

=== Autoservicio de perfil

Los académicos pueden actualizar parte de su información mediante un mecanismo de edición controlado por correo electrónico. El administrador puede generar códigos de edición de un solo uso y enviarlos de forma individual o masiva.

El académico utiliza este código para solicitar un enlace de edición, que se envía a su correo y tiene un tiempo de vigencia limitado. El enlace queda asociado al estado actual del registro para evitar que pueda utilizarse sobre una versión anterior de los datos.

Desde este espacio se pueden actualizar datos personales como nombres, ORCID, sexo, fecha de nacimiento, nacionalidad y ciudad.

El académico puede corregir información de las publicaciones en las que participa. Entre los datos que puede modificar se encuentran el título, resumen, DOI, año, estado, autor correspondiente, afiliaciones y línea de investigación. La plataforma combina de este modo la sincronización automática con la revisión de los propios académicos.

= Proyección a futuro

Como parte de la continuidad del proyecto, se identificaron diversas funcionalidades que podrían incorporarse en futuras etapas de desarrollo. Estas permitirían ampliar el alcance actual de la plataforma y responder a nuevas necesidades relacionadas con la gestión, visualización y análisis de la investigación desarrollada en la facultad.

== Incorporación de colaboradores externos e internacionalización

Una futura ampliación consistiría en incorporar al sistema a coautores pertenecientes a otras facultades, universidades e instituciones extranjeras. Para cada colaborador externo, se podría almacenar información obtenida desde las fuentes utilizadas durante la sincronización, como su país de origen y su institución de afiliación.

La incorporación de estos investigadores permitiría extender la red de colaboración actualmente disponible. Asimismo, las recomendaciones de posibles colaboradores podrían considerar tanto a investigadores internos como externos, de acuerdo con su afinidad temática. Para ello, se podrían utilizar las métricas ya implementadas en el sistema, basadas en la coincidencia de tópicos, palabras clave y líneas de investigación.

Esta información permitiría identificar las instituciones extranjeras con las que existen vínculos de colaboración y reconocer investigadores cuyas áreas de trabajo sean afines a las de la facultad. Estos antecedentes podrían servir de apoyo para evaluar posibles convenios, pasantías y nuevas colaboraciones académicas internacionales.

== Catálogo de revistas y análisis de costos de publicación

Otra funcionalidad futura consiste en integrar un catálogo de revistas a partir de las publicaciones existentes en la plataforma. Este catálogo podría incorporar información relevante de cada revista, como el costo asociado al *Article Processing Charge* (APC).

La disponibilidad de estos datos permitiría estimar presupuestos de publicación y analizar alternativas de revistas dentro de una misma línea de investigación o nivel de indexación. Por ejemplo, sería posible comparar los costos estimados de publicación entre distintas revistas y apoyar la toma de decisiones de los investigadores.

Para incorporar información sobre cuartiles e indexación, sería necesaria una coordinación con la Biblioteca de la universidad, debido a que estos datos se encuentran disponibles mediante la base de datos Web of Science de Clarivate, cuya suscripción es institucional.

== Incorporación de proyectos de investigación

Se propone incorporar un módulo de proyectos de investigación, comenzando por aquellos adjudicados a través de ANID. Esta implementación requeriría coordinación con la Vicerrectoría de Investigación, unidad que administra información sobre proyectos adjudicados y en ejecución.

Actualmente, el análisis de la producción científica se basa principalmente en las publicaciones y los académicos que participan en ellas. Sin embargo, los proyectos de investigación relacionados con dichas publicaciones no forman parte del modelo actual, por lo que no es posible establecer una relación directa entre una publicación y el proyecto que financió su desarrollo.

La incorporación de esta información permitiría incluir tanto proyectos financiados por ANID como proyectos en curso financiados por otras fuentes o sin financiamiento ANID. De este modo, la plataforma podría ofrecer una visión más completa de la actividad investigativa de la facultad.

== Incorporación de profesores part-time y postdoctorandos

Finalmente, se propone incorporar al sistema a profesores part-time y postdoctorandos vinculados a la facultad. Esto permitiría representar de manera más completa a los investigadores que participan en la producción científica y en las redes de colaboración, aun cuando no cuenten con una vinculación académica de jornada completa.
