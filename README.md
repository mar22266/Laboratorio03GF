# LABORATORIO 3 - Gráficos por Computadora

## Descripción
Este proyecto de Rust utiliza `minifb` y `nalgebra_glm` para renderizar un modelo 3D de una nave en tiempo real. La aplicación permite interactuar con la nave mediante el teclado para moverla, escalarla y rotarla en un espacio tridimensional.

## Funcionalidades
- **Renderizado de Modelos 3D:** Carga y renderiza modelos 3D desde archivos `.obj`.
- **Interacción en Tiempo Real:** Controla la posición, escala y rotación de la nave en tiempo real.
- **Control de Entrada:** Utiliza el teclado para modificar los parámetros de la nave visualizada.

## Controles
- **Movimiento:**
  - `W`: Mover hacia arriba.
  - `S`: Mover hacia abajo.
  - `A`: Mover hacia la izquierda.
  - `D`: Mover hacia la derecha.
- **Escala:**
  - `R`: Aumentar la escala.
  - `F`: Disminuir la escala.
- **Rotación:**
  - `Q` y `E`: Rotar sobre el eje X.
  - `Z` y `C`: Rotar sobre el eje Y.
  - `U` y `I`: Rotar sobre el eje Z.
- **General:**
  - `Escape`: Cerrar la aplicación.

## Configuración del Proyecto
1. Asegúrate de tener Rust y `cargo` instalados.
2. Clona este repositorio en tu máquina local.
3. Navega al directorio del proyecto y ejecuta `cargo run` para iniciar la aplicación.
4. Utiliza los controles mencionados para interactuar con la nave.

## Tecnologías Utilizadas
- Rust
- `minifb`: Para la gestión de la ventana y los eventos de entrada.
- `nalgebra_glm`: Para las operaciones matemáticas relacionadas con gráficos.
