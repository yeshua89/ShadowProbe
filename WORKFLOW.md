# 🎯 ShadowProbe - Guía Visual de Flujo de Trabajo

## ¿Qué comando uso?

```
┌─────────────────────────────────────────────────────────────┐
│  ¿VAS A HACER UNA SOLA TAREA?                               │
│  (compilar, testear, formatear)                             │
└─────────────────────────────────────────────────────────────┘
                            │
                           SÍ
                            │
                            ▼
        ┌───────────────────────────────────────┐
        │  USA COMANDOS DIRECTOS                │
        │  ────────────────────────              │
        │  make compile    ← Compilar           │
        │  make test       ← Tests              │
        │  make fmt        ← Formatear          │
        │  make run-cli    ← Ver CLI            │
        │  make quick      ← Compile + test     │
        └───────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  ¿VAS A TRABAJAR POR 30+ MINUTOS?                           │
│  (múltiples cambios, debugging, experimentar)               │
└─────────────────────────────────────────────────────────────┘
                            │
                           SÍ
                            │
                            ▼
        ┌───────────────────────────────────────┐
        │  USA SHELL INTERACTIVA                │
        │  ────────────────────────              │
        │  make dev                             │
        │                                       │
        │  Dentro del contenedor:               │
        │  - cargo build                        │
        │  - cargo test                         │
        │  - cargo watch -x test                │
        │  - cargo run -- scan --url ...        │
        │                                       │
        │  exit  ← Cuando termines              │
        └───────────────────────────────────────┘
```

---

## 🔥 Función de `make dev`

`make dev` **SÍ tiene sentido** para:

### ✅ Casos de Uso Válidos:

1. **Desarrollo iterativo con hot-reload**
   ```bash
   make dev
   cargo watch -x test    # Auto-ejecuta tests al cambiar código
   ```

2. **Debugging complejo**
   ```bash
   make dev
   cargo run -- scan --url https://target.com --verbose
   # Ves errores, ajustas, vuelves a ejecutar
   cargo run -- scan --url https://target.com --depth 3
   ```

3. **Experimentación**
   ```bash
   make dev
   cargo run -- --help
   cargo run -- list
   cargo run -- scan --url ...
   # Pruebas diferentes comandos sin salir/entrar
   ```

4. **Trabajo prolongado** (múltiples tareas consecutivas)
   ```bash
   make dev
   cargo build
   cargo test
   cargo fmt
   cargo doc --open
   # Todo sin tener que esperar que Docker arranque cada vez
   ```

### ❌ NO uses `make dev` para:

- ❌ Solo compilar (usa `make compile`)
- ❌ Solo ejecutar tests (usa `make test`)
- ❌ Solo formatear (usa `make fmt`)
- ❌ Tareas únicas en general

---

## 📊 Comparación de Rendimiento

### Escenario: Necesitas compilar + testear + formatear

**Opción 1: Comandos directos**
```bash
make compile    # Docker arranca → compila → para (10s)
make test       # Docker arranca → tests → para (15s)
make fmt        # Docker arranca → formatea → para (5s)
# TOTAL: ~30 segundos
```

**Opción 2: Shell interactiva**
```bash
make dev        # Docker arranca UNA vez (3s)
cargo build     # Compila (7s)
cargo test      # Tests (12s)
cargo fmt       # Formatea (2s)
exit            # Para (1s)
# TOTAL: ~25 segundos
```

**Conclusión:** Para múltiples tareas, `make dev` es más eficiente.

---

## 🎬 Ejemplos Reales

### Ejemplo 1: "Fix rápido de un bug"
```bash
# Editaste el código...
make test       # Verificar que funciona
make fmt        # Formatear
# Done! (sin necesidad de make dev)
```

### Ejemplo 2: "Nueva feature completa"
```bash
make dev

# Desarrollo iterativo con hot-reload:
cargo watch -x "test --lib"

# En tu editor haces cambios...
# Los tests se ejecutan automáticamente

# Cuando terminas:
exit
```

### Ejemplo 3: "Testing contra URLs reales"
```bash
make dev

# Pruebas diferentes configuraciones:
cargo run -- scan --url https://test1.com
cargo run -- scan --url https://test1.com --depth 5
cargo run -- scan --url https://test1.com --profile stealth
cargo run -- scan --url https://test2.com

exit
```

---

## 🤔 Entonces, ¿cuál es la función de `make dev`?

### Respuesta:

`make dev` **NO es para compilar/testear tareas únicas**.

`make dev` **ES para sesiones de desarrollo interactivo prolongado**.

**Analogía:**
- `make compile`, `make test` → Como pedir un Uber para cada lugar
- `make dev` → Como rentar un auto por el día cuando vas a varios lugares

---

## 📝 Regla Simple

```
┌──────────────────────────────────────────────┐
│  1 tarea  →  make [comando]                  │
│  2+ tareas continuas  →  make dev            │
└──────────────────────────────────────────────┘
```

---

## 🚀 Comandos Recomendados por Situación

| Situación | Comando Recomendado | Por qué |
|-----------|---------------------|---------|
| Primera compilación | `make compile` | Rápido y simple |
| Ejecutar tests | `make test` | Directo |
| Formatear código | `make fmt` | Sin overhead |
| Ver ayuda del CLI | `make run-cli` | Conveniencia |
| Desarrollo activo (1hr+) | `make dev` + `cargo watch` | Eficiencia |
| Debugging complejo | `make dev` | Interactividad |
| Testing múltiples URLs | `make dev` | Evita re-arrancar Docker |
| CI/CD | `make ci` | Todo en uno |
| Quick check antes de commit | `make quick` | Compile + test |

---

## 💡 Conclusión

`make dev` **tiene mucho sentido** cuando:
- 🔧 Vas a trabajar por un rato largo
- 🧪 Necesitas ejecutar múltiples comandos
- 🐛 Estás debugging algo complejo
- ⚡ Quieres usar `cargo watch` para hot-reload

**NO uses `make dev` para tareas únicas** → usa comandos directos que son más rápidos.

¡Ambos enfoques son válidos dependiendo de tu caso de uso! 🎯
