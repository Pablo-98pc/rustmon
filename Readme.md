![alt text](image.png)

Título: RustMon 🦀

Descripción: Monitor de sistema ligero escrito en Rust con interfaz TUI.

Features:

    -Monitorización de CPU/RAM en tiempo real.

    -Gráficos históricos (Sparklines).

    -Filtrado de procesos en vivo.

    -Gestión de procesos (Kill command).


🛠️ Instalación y Uso:

Asegúrate de tener Rust y Cargo instalados en tu sistema.

1.Clona el repositorio:git clone https://github.com/Pablo-98pc/rustmon.git

2.Ejecuta el programa (recomendado usar --release para mayor rendimiento y precisión en los datos):cargo run --release

⌨️ Controles y Atajos:

RustMon está diseñado para ser controlado íntegramente con el teclado. Aquí tienes la lista de comandos disponibles:

    -Tecla q -> Salir de la aplicación.

    -↑ / ↓Navegar (Scroll) por la lista de procesos.

    -Tecla k Kill: Abre el menú para matar el proceso seleccionado.

    -Esc Cancelar / Limpiar: Cierra popups o borra el filtro de búsqueda.

    -(Escribir)Buscador: Escribe cualquier letra para filtrar procesos por nombre en tiempo real.

    -BackspaceBorrar: Elimina el último carácter del filtro de búsqueda.