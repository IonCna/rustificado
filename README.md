# 🪓 RUSTCADO - Juego del Ahorcado en Rust

**RUSTCADO** es una implementación simple del clásico juego del ahorcado, desarrollada enteramente en Rust como ejercicio de aprendizaje de lógica, estructuras y manejo de estados.

## 🧠 Descripción

Este proyecto consiste en una aplicación de terminal en Rust que permite al jugador adivinar palabras con un número limitado de intentos. El código está dividido en dos módulos principales:

- `main.rs`: Orquesta el flujo principal del juego, define palabras y controla la interacción con el usuario.
- `hanged.rs`: Implementa la estructura `State`, que encapsula el estado del juego (intentos, palabra oculta, letras adivinadas, etc.).

## 📂 Estructura del proyecto

```
.
├── main.rs        # Punto de entrada, contiene el loop principal
├── hanged.rs      # Módulo con la lógica del estado del juego
```

## ⚙️ Características

- Manejo de estado inmutable/mutable con `State`.
- Entrada de usuario por consola.
- Control de intentos máximos.
- Comparación de letras.
- Validación de juego completo.
- Palabras aleatorias seleccionadas de una lista estática.

## 📝 Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (versión estable)

## ▶️ Cómo ejecutar

```bash
cargo run
```

El juego comenzará en la terminal y te pedirá que adivines la palabra letra por letra.

## 📌 Ejemplo de palabras

```rust
static WORDS: &[&str] = &[
    "zanahoria",
    "manzana",
    "pera",
    "sandia",
    "huevos",
    "al",
    "backero",
];
```

## 🚧 Posibles mejoras

- Interfaz gráfica con `egui` o `tui`.
- Dificultad ajustable.
- Persistencia de puntajes.
- Soporte para palabras desde archivo externo.

## 🧑‍💻 Autor

Desarrollado como parte del aprendizaje del lenguaje Rust.
