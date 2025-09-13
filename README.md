# cli-tool-dir-managment

Herramienta CLI sencilla para la gestión de directorios desde la terminal. Permite crear, listar, modificar, eliminar y organizar directorios de manera eficiente.
en dessarrollo
para Linux.

## Características

- Crear y eliminar directorios fácilmente.
- Listar el contenido de directorios.
- Modificar y renombrar carpetas.
- Organización rápida desde la línea de comandos.

## Instalación

Clona el repositorio y compila el proyecto:

```bash
git clone https://github.com/insixdev/cli-tool-dir-managment.git
cd cli-tool-dir-managment
cargo build
```
O una mejor manera para que se instale de fomra global (Recomendada):
```bash
cargo install --path .
```

## Uso

Ejecuta la herramienta desde la terminal:

```bash
cd target/debug/
./<comando> [opciones]
```
### portabilidad
siendo un binario
puedes exportar el bin a los local bin 

```bash
sudo cp target/debug/cliToolCarpet /usr/local/bin/
```

Comandos disponibles:
- `create <nombre>`: Crea un nuevo directorio.
- `delete <nombre>`: Elimina un directorio. soon
- `list` Lista contenido de un directorio. soon
- `move <origen> <destino>`: Mueve/renombra un directorio. soon

## Licencia

MIT

---
Desarrollado por [insixdev](https://github.com/insixdev)
