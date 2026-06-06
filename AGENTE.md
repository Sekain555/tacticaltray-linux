# AGENTE — TacticalTray Linux

> Guía para agentes autónomos de IA que trabajen en este proyecto.
> Lee este archivo completo antes de modificar cualquier código.

---

## Qué es este proyecto

TacticalTray Linux es una aplicación de system tray para Linux que muestra a **Nox** — personaje del videojuego *Nightfall Tactics* — animado en la barra de tareas de KDE. La velocidad de animación refleja la carga del CPU. El menú contextual muestra métricas del sistema en tiempo real.

Es el port Linux de [TacticalTray Windows](https://github.com/Sekain555/tacticaltray), construido desde cero en Rust.

---

## Stack

| Tecnología | Versión | Rol |
|---|---|---|
| Rust | 1.95+ | Lenguaje principal |
| ksni | 0.3.4 | System tray via StatusNotifierItem |
| sysinfo | 0.33 | Métricas del sistema |
| image | 0.25 | Carga de frames PNG |

---

## Estructura del proyecto

```
tacticaltray-linux/
  src/
    main.rs          ← TODO el código vive aquí (proyecto pequeño)
  assets/
    nox_0..7.png         ← frames modo claro
    nox_dark_0..7.png    ← frames modo oscuro
    icon.png             ← ícono fallback
    walk_*.png           ← animaciones futuras (no usar aún)
    crouchwalk_*.png     ← animaciones futuras (no usar aún)
    climbback_*.png      ← animaciones futuras (no usar aún)
    shoot2h_*.png        ← animaciones futuras (no usar aún)
  Cargo.toml
  Cargo.lock           ← NO ignorar — necesario para AUR
  PKGBUILD             ← NO aquí — vive en aur-tacticaltray/
```

---

## Reglas críticas

### No tocar sin autorización del desarrollador

- Los frames de `walk_*` y `shoot2h_*` están reservados para integración con Nightfall Tactics. **No activarlos ni referenciarlos en el código visible al usuario.**
- El sistema de progresión de kilómetros está planificado pero no implementado. No implementarlo sin diseño previo aprobado.
- No cambiar la API del menú contextual sin consultar — el formato de las métricas es una decisión de UX del desarrollador.

### Rust y ownership

- `ksni::TrayMethods` debe estar importado para usar `.spawn()` en el struct
- `handle.update()` es la única forma segura de modificar el estado del tray desde fuera del hilo de ksni
- Los frames se cargan una sola vez al inicio con `include_bytes!` — no releer archivos en tiempo de ejecución
- `System` de sysinfo **no implementa `Clone`** — no intentar clonarlo

### AUR

- El repositorio AUR es **separado** del repositorio de código: `ssh://aur@aur.archlinux.org/tacticaltray-linux.git`
- El `PKGBUILD` y `.SRCINFO` viven en `~/Proyectos/SOFTWARE/aur-tacticaltray/`
- Después de cualquier cambio al PKGBUILD: `makepkg --printsrcinfo > .SRCINFO`
- El sha256 en el PKGBUILD debe ser **solo el hash**, sin nombre de archivo

### Shell

El desarrollador usa **fish**. Los heredocs (`<< 'EOF'`) no funcionan en fish. Usar `echo '...' >` o escribir archivos directamente.

---

## Cómo compilar y probar

```bash
# Compilar en debug (rápido)
cargo build

# Ejecutar
cargo run

# Compilar en release (lento, optimizado)
cargo build --release

# Ejecutar el binario release directamente
./target/release/tacticaltray-linux
```

---

## Cómo actualizar el AUR

```bash
# 1. Crear tag en GitHub
git tag v1.X.0
git push origin v1.X.0

# 2. Obtener sha256
curl -L "https://github.com/Sekain555/tacticaltray-linux/archive/refs/tags/v1.X.0.tar.gz" -o /tmp/tt.tar.gz
sha256sum /tmp/tt.tar.gz

# 3. Actualizar PKGBUILD (pkgver y sha256sums)
# 4. Regenerar .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# 5. Push al AUR
git add PKGBUILD .SRCINFO
git commit -m "chore: release v1.X.0"
git push origin master

# 6. Verificar instalación
rm -rf ~/.cache/paru/clone/tacticaltray-linux
paru -S tacticaltray-linux
```

---

## Backlog actual (en orden de prioridad)

1. **Binario precompilado en GitHub Releases** — evitar que los usuarios compilen 233MB de dependencias
2. **Sistema de progresión de kilómetros** — contador acumulativo que desbloquea animaciones
3. **Soporte Crouch Walk y Climb Back** — activar una vez implementado el sistema de progresión
4. **Integración Nightfall Tactics** — desbloqueo de Walk y Shoot 2H al instalar/completar el juego
5. **Widget de escritorio** — ventana flotante con Nox y métricas (Fase 2)

---

## Contexto del desarrollador

- **Nombre:** Victor 'Sekain' Sepúlveda
- **GitHub:** Sekain555
- **Sistema:** CachyOS (Arch-based) con KDE Plasma 6 + Wayland
- **Shell:** fish
- **Editor:** VSCodium + rust-analyzer
- **Nivel Rust:** principiante — aprendiendo con este proyecto
- **Metodología:** Kanban simplificado, una feature a la vez, CONTEXTO.md siempre actualizado

---

## Archivos de referencia

| Archivo | Propósito |
|---|---|
| `CONTEXTO.md` | Estado técnico actual del proyecto |
| `METODOLOGIA.md` | Flujo de trabajo y convenciones |
| `AGENTE.md` | Este archivo — guía para agentes IA |
| `README.md` | Documentación pública del proyecto |
| `Cargo.toml` | Dependencias y metadata del proyecto |
