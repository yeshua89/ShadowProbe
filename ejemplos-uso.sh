#!/bin/bash
# Ejemplos de uso de ShadowProbe
# Este archivo muestra los comandos correctos para cada tarea

echo "═══════════════════════════════════════════════════════"
echo "  EJEMPLOS DE USO - ShadowProbe"
echo "═══════════════════════════════════════════════════════"
echo ""

# Ejemplo 1: Compilar el proyecto
echo "1️⃣  Compilar el proyecto (SIN entrar a shell):"
echo "   make compile"
echo ""

# Ejemplo 2: Ejecutar tests
echo "2️⃣  Ejecutar tests (SIN entrar a shell):"
echo "   make test"
echo ""

# Ejemplo 3: Usar el CLI después de compilar
echo "3️⃣  Ver ayuda del CLI:"
echo "   docker compose run --rm dev ./target/debug/shadowprobe --help"
echo ""

# Ejemplo 4: Listar scanners
echo "4️⃣  Listar scanners disponibles:"
echo "   docker compose run --rm dev ./target/debug/shadowprobe list"
echo ""

# Ejemplo 5: Escanear una URL (ejemplo)
echo "5️⃣  Escanear una URL:"
echo "   docker compose run --rm dev ./target/debug/shadowprobe scan \\"
echo "     --url https://example.com \\"
echo "     --depth 2 \\"
echo "     --output scan-results.json"
echo ""

# Ejemplo 6: Trabajo interactivo
echo "6️⃣  Trabajar de forma interactiva:"
echo "   make dev                    # Abre la shell"
echo "   # Dentro del contenedor:"
echo "   cargo build                 # Compila"
echo "   cargo test                  # Ejecuta tests"
echo "   cargo run -- --help         # Ejecuta el CLI"
echo "   exit                        # Sale del contenedor"
echo ""

# Ejemplo 7: Formatear código
echo "7️⃣  Formatear código:"
echo "   make fmt"
echo ""

# Ejemplo 8: Limpiar todo
echo "8️⃣  Limpiar contenedores y caché:"
echo "   make clean"
echo ""

echo "═══════════════════════════════════════════════════════"
echo "  ⚠️  RECUERDA:"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "❌ INCORRECTO (si estás dentro del contenedor):"
echo "   root@xxx:/app# make build   ← No funciona!"
echo ""
echo "✅ CORRECTO (opción 1 - desde tu terminal):"
echo "   $ make compile              ← Funciona!"
echo ""
echo "✅ CORRECTO (opción 2 - dentro del contenedor):"
echo "   root@xxx:/app# cargo build  ← Funciona!"
echo "   root@xxx:/app# exit         ← Sale del contenedor"
echo "   $ make compile              ← Ahora funciona!"
echo ""
echo "═══════════════════════════════════════════════════════"
