# Backend para el punto de venta (todavía no tengo un nombre XD)
Este es el backend que se usará para nuestro proyecto final y probablemente también se va a usar en un entorno real.

## Recomendaciones
Como este programa tal vez use mucho la línea de comandos, lo mejor es que te instales [Visual Studio Code](https://code.visualstudio.com/Download), [git](https://git-scm.com/downloads) y [winget](https://www.microsoft.com/p/app-installer/9nblggh4nns1?ocid=9nblggh4nns1_ORSEARCH_Bing&rtc=1&activetab=pivot:overviewtab), son opcionales pero encarecidamente recomendados.

### En caso de instalar VS Code
Instalar rust-analyzer en VS Code te facilitaría mucho la edición de este código y también la detección de errores antes de siquiera compilar. Instálalo presionando ```Ctrl + P``` y pegando ```ext install matklad.rust-analyzer```

## Primero que nada, ¿Cómo descargo el código xd?
Usando git puedes hacer ```git clone https://github.com/Aaron-Uriel/server-backend.git``` desde tu terminal, esto creará una carpeta llamada server-backend, en la carpeta donde sea que estés por lo que es mejor que hagas habras la terminal en Documentos.
También puedes Descargarlo desde el apartado "Code" de color verde y descargarlo como zip.

## Requisitos de compilación
* Para compilar este proyecto se necesita de el lenguaje de programación Rust el cual puedes instalar desde [aquí](https://rustup.rs/).
* Para poder crear la base de datos de forma mas automática se necesita de diesel_cli, lo podemos instalar con: ```cargo install diesel_cli --no-default-features --features mysql``` (para este paso ya debiste de haber instalado correctamente Rust, en caso de no reconocer a cargo como un programa probablemente hay algún error con la variable PATH, usualmente el instalador te lo menciona al final).
* Se necesita de mysql/mariadb el cual puedes instalar desde winget (es algo que supongo ahorita, mas tarde lo comprobaré) y el resto de dependencias serán instaladas por cargo (el gestor de paquetes de rust).

## Requisitos de ejecución
* Necesitas de tener mysql/mariadb correctamente configurado con tu usuario y contraseña ya puesto, ya con eso ahora necesitas poner tu usuario y contraseña en el archivo .env que está en esta carpeta.
* Para crear la base de datos sobre la que corre el programa necesitas teclear ```diesel setup```, debería crear una base de datos llamada "punto_de_venta" y ejecutar las migraciones (la tablas de la bd), si crea la bd vacía ejecuta ```diesel migration run```.

## Compilación
Ejecutar el siguiente comando dentro de esta carpeta debería de compilarlo y ejecutarlo sin problemas.
    ```cargo run --release```
Puede llegar a tardar mucho la primera vez pero las próximas serán mas rápidas.