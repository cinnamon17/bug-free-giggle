# BootLoader para procesadores RISC-V

Implementación de kernel para procesadores RISC-V

## Instalación de las herramientas

Además de tener Rust instalado, son necesarias las herramientas de emulación y compilación

~~~
sudo apt install qemu-system-misc gcc-riscv64-unknown-elf
~~~

Componentes de Rust para RISC-V

~~~
rustup target add riscv64gc-unknown-none-elf
~~~

## Ejecución

Dentro del archivo .cargo/config.toml dejé las instrucciones necesarias para fácil ejecución con

~~~
cargo run
~~~

## Mapa de memoría de qemu (virt)

~~~
Dirección	Dispositivo	¿Para qué sirve?
0x10000000	UART		Imprimir texto en tu terminal.
0x02000000	CLINT		Hacer que el tiempo pase (Timers).
0x0C000000	PLIC		Manejar botones, red y eventos.
0x80000000	RAM		    variables y código de ejecución.
~~~

Nota: Este kernel arranca en modo Privilege, saltándose OpenSBI mediante el flag -bios none.

