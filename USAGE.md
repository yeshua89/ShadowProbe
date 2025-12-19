# Guía de Uso - ShadowProbe

## 🎯 ¿Cuándo usar cada comando?

### Opción 1: Comandos Rápidos (Recomendado para tareas específicas)

Usa estos comandos cuando necesites hacer UNA tarea rápida:

```bash
make compile    # Compilar rápidamente
make test       # Ejecutar tests
make fmt        # Formatear código
make run-cli    # Ver ayuda del CLI
```

**Ventajas:**
- ✅ Rápido y directo
- ✅ Perfecto para CI/CD
- ✅ No tienes que recordar salir

**Desventajas:**
- ⏱️ Cada comando arranca/para el contenedor Docker
- ⏱️ Si vas a hacer múltiples tareas, es más lento

---

### Opción 2: Shell Interactiva (Recomendado para desarrollo prolongado)

Usa `make dev` cuando vayas a:
- 🔧 Trabajar por un buen rato (30+ minutos)
- 🐛 Debuggear algo complejo
- 🧪 Experimentar con múltiples comandos
- 📝 Hacer cambios y testear iterativamente

```bash
make dev        # Entras UNA vez al contenedor

# Ahora dentro del contenedor puedes hacer TODO esto:
cargo build
cargo test
cargo run -- --help
cargo run -- scan --url https://example.com
cargo watch -x test    # Auto-ejecuta tests al cambiar código
cargo doc --open       # Genera documentación
exit                   # Cuando termines
```

**Ventajas:**
- ✅ Contenedor arranca UNA sola vez
- ✅ Trabajas fluido sin interrupciones
- ✅ Más rápido para múltiples tareas

**Desventajas:**
- ⚠️ Tienes que recordar salir con `exit`
- ⚠️ Comandos `make` NO funcionan dentro

---

## 🚀 Flujos de Trabajo Típicos

### Flujo 1: "Solo quiero compilar y ver si funciona"
```bash
make compile
make run-cli
```

### Flujo 2: "Voy a trabajar 1 hora haciendo cambios"
```bash
make dev

# Dentro del contenedor:
cargo watch -x test    # Se ejecutan tests automáticamente
# Haces cambios en tu editor...
# Los tests se re-ejecutan solos
exit
```

### Flujo 3: "Quiero asegurarme que todo está bien antes de commit"
```bash
make ci    # Ejecuta: fmt + clippy + test
```

### Flujo 4: "Necesito testear el scanner contra una URL real"
```bash
make dev

# Dentro:
cargo run -- scan --url https://testphp.vulnweb.com --depth 2
cargo run -- scan --url https://testphp.vulnweb.com --profile stealth
exit
```

---

## 🎓 Ejemplos Prácticos

### Ejemplo 1: Primera vez usando el proyecto
```bash
# 1. Compilar
make compile

# 2. Ver ayuda
make run-cli

# 3. Listar scanners
docker compose run --rm dev ./target/debug/shadowprobe list

# 4. Ejecutar tests
make test
```

### Ejemplo 2: Desarrollo diario
```bash
# Entrar a la shell
make dev

# Trabajar con hot-reload
cargo watch -x "test --lib"

# Cuando termines
exit
```

### Ejemplo 3: Fix rápido
```bash
# Editas el código en tu editor...

# Verificar que compila
make check

# Formatear
make fmt

# Tests
make test

# Done! No necesitas entrar a la shell
```

---

## ❓ Preguntas Frecuentes

### ¿Por qué no puedo usar `make build` dentro de `make dev`?

Porque `make dev` abre un contenedor Docker, y dentro de ese contenedor no está instalado Docker (y no debería estarlo). Los comandos `make` ejecutan `docker compose`, así que solo funcionan FUERA del contenedor.

**Solución:**
- Dentro del contenedor: usa `cargo build`
- Fuera del contenedor: usa `make compile`

### ¿Cuál es la diferencia entre `make compile` y `make build`?

- `make compile`: Compila en modo DEBUG (rápido, con símbolos de debug)
- `make build`: Compila en modo RELEASE (optimizado, más lento de compilar)

Para desarrollo normal, usa `make compile`.

### ¿Puedo tener la shell abierta en una terminal y ejecutar `make test` en otra?

¡Sí! Es perfectamente válido. Por ejemplo:

**Terminal 1:**
```bash
make dev
# Te quedas aquí trabajando
```

**Terminal 2:**
```bash
make test      # Funciona normal
make compile   # Funciona normal
```

### ¿Debo siempre usar `make dev`?

No. Usa `make dev` solo cuando vayas a:
- Trabajar por un buen rato
- Ejecutar múltiples comandos secuencialmente
- Usar `cargo watch` para desarrollo iterativo

Para tareas únicas (compilar, testear, formatear), usa los comandos directos `make compile`, `make test`, etc.

---

## 📋 Resumen de Comandos

| Comando | ¿Abre Shell? | Úsalo para... |
|---------|--------------|---------------|
| `make compile` | ❌ No | Compilar rápido |
| `make test` | ❌ No | Ejecutar tests |
| `make fmt` | ❌ No | Formatear código |
| `make run-cli` | ❌ No | Ver ayuda del CLI |
| `make dev` | ✅ Sí | Trabajar largo rato |
| `make quick` | ❌ No | Compile + test rápido |
| `make ci` | ❌ No | Verificar antes de commit |

---

## 💡 Recomendación Final

**Para la mayoría de casos: usa comandos directos como `make compile`, `make test`.**

Solo usa `make dev` cuando realmente vayas a estar trabajando de forma interactiva por un buen rato.

¡Esto hace tu flujo de trabajo más eficiente! 🚀
