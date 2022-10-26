# SGSSI RUST MINER

Este proyecto es una versión en Rust de [SGSSI_minero](https://github.com/UnaiFernandez/SGSSI_minero). El objetivo del minero en es conseguir que el resumen del último bloque tenga la mayor cantidad de ceros al principio del resumen sha256 del bloque. Mediante este programa, se calcula el resumen con la cantidad de ceros establecida en el segundo parametro como minimo.
## Instrucciones de uso

### Instalando lo necesario

Instalar la herramienta rustup, para gestionar las versiones de Rust y otras herramientas asociadas.

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

(Para instalar la ultima version: [Install Rust](https://www.rust-lang.org/tools/install))

Para la creación de aplicaciones y su ejecución se usara el gestor de paquetes de Rust *Cargo*. Cargo viene preintalado si al instalar se han usado los instaladores oficiales. 


### Usando cargo para compilar y ejecutar el proyecto

**Para compilar el programa usa:**

    cargo build

**Para ejecutarlo usa:**

    cargo run -- CBXX N

El primer parametro es el nombre del bloque (CB04 en este caso, porque es el unico disponible en el repositorio) y el segundo parametro es el numero de ceros que se quieren obteber como minimo al principio del resumen.

**Para hacer clean ejecuta:**

    cargo clean

De esta manera se eliminan todos los archivos no necesarios, generados al compilar el código.


![Ejemplo de ejecución](images/minero_img.png "Ejemplo de ejecución")


## Integridad de los archivos

**Cargo.toml:** 1df36f3b30540dc1064b0846e50eadb6108b8aaf13dce8a5c6b8d81415ebd128 

**main.rs:** ba3ac7cf6099b6597a86c4677560dceba0c0c9560500bf4ea8639a216117af13 
