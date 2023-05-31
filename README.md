# Desktop application
<h2>Proyecto final para curso introductorio de Rust - Polkadot Hub</h2>
Este proyecto es una aplicación de Rust creada con FLTK para la interfaz de usuario y SQLite para la gestión de bases de datos. La aplicación permite a los usuarios insertar, visualizar, borrar y editar datos de propiedades inmobiliarias.

<h3>Comenzando</h3>

Estas instrucciones proporcionarán una copia del proyecto en funcionamiento en tu máquina local para fines de desarrollo y pruebas.

<h3>Prerrequisitos</h3>
Necesitarás Rust y SQLite instalados en tu máquina. Consulta la página oficial de Rust y la página oficial de SQLite para obtener instrucciones sobre cómo instalarlos.

<h3>Instalación</h3>
Sigue estos pasos para instalar y ejecutar el proyecto:

<code>
# Clona el repositorio
git clone https://github.com/ail3ngrimaldi/final-proyect.git
# Entra en el directorio del proyecto
cd final-proyect
# Construye el proyecto
cargo build
# Ejecuta el proyecto
cargo run
</code>

Una vez ejecutado este comando, debería abrirse la aplicación desktop, donde podrás cargar datos desde la pantalla inicial, y también desde allí ver los datos que ya están cargados, borrar y editar datos (utilizando como identificadores la Calle y el Numero de la Entidad vivienda.)

<h3>Construido con</h3>

<ul>
   <li> Rust - El lenguaje de programación usado.</li>
   <li>  SQLite - Gestor de bases de datos.</li>
   <li> FLTK - Biblioteca de GUI.</li>
</ul>
