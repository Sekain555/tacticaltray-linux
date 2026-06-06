# CONTEXTO — TacticalTray Linux

## Stack tecnológico

| Tecnología | Versión | Rol |
|---|---|---|
| Rust | 1.95.0 | Lenguaje principal |
| ksni | 0.3.4 | System tray via StatusNotifierItem (KDE/freedesktop) |
| sysinfo | 0.33 | Lectura de métricas del sistema |
| image | 0.25 | Carga y procesamiento de frames PNG |
| cargo | 1.95.0 | Gestor de paquetes y build system |

**Versión actual:** `1.0.0`
**Plataforma:** Linux (KDE Plasma, Wayland/X11)
**Distribución:** AUR (`tacticaltray-linux`)

---

## Arquitectura

Aplicación de sistema tray con un único binario autónomo. Los assets (frames PNG de Nox) están embebidos en el binario mediante `include_bytes!` — no requiere archivos externos en tiempo de ejecución.

```
[main.rs]
  ├── NoxTray (struct) — implementa ksni::Tray
  │     ├── frames_light / frames_dark — frames PNG embebidos
  │     ├── frame_index — frame actual de la animación
  │     ├── cpu_usage — carga actual del CPU
  │     └── sys_info — string de métricas formateado
  │
  ├── Hilo principal — loop de animación
  │     ├── sysinfo::System — lectura de CPU
  │     ├── cpu_to_interval_ms() — velocidad según carga
  │     └── handle.update() — actualiza frame en el tray
  │
  └── ksni::TrayService — gestiona el tray via D-Bus/StatusNotifierItem
```

---

## Componentes principales

### NoxTray
Struct principal que implementa `ksni::Tray`. Contiene el estado completo de la aplicación.

| Campo | Tipo | Descripción |
|---|---|---|
| `frame_index` | `usize` | Índice del frame actual de Nox |
| `frames_light` | `Vec<Vec<u8>>` | Frames PNG modo claro embebidos |
| `frames_dark` | `Vec<Vec<u8>>` | Frames PNG modo oscuro embebidos |
| `dark_mode` | `bool` | Tema detectado del sistema |
| `cpu_usage` | `f32` | Uso actual del CPU (%) |
| `sys_info` | `String` | Métricas formateadas para el menú |

### Funciones core

| Función | Descripción |
|---|---|
| `load_frames(dark)` | Carga los 8 frames PNG con `include_bytes!` |
| `detect_dark_mode()` | Lee `~/.config/kdeglobals` para detectar el tema |
| `build_info(sys)` | Genera el string de métricas del menú |
| `cpu_to_interval_ms(cpu)` | Convierte carga CPU en intervalo de animación |
| `icon_pixmap()` | Convierte el frame actual a formato ARGB32 para ksni |

### Métricas del menú

- 🖥️ CPU: uso (%) + temperatura (°C)
- 🎮 GPU: uso (%) + temperatura (°C) — leídos desde `/sys/class/drm/`
- 🧠 RAM: usada / total (MB)
- 💾 Disco: usado / total (GB) — solo partición raíz `/`
- 🌐 Red: interfaces activas con tráfico recibido/enviado (KB)

---

## Animación

La velocidad de animación es proporcional a la carga del CPU:

```rust
let interval_ms = (350.0 - (cpu / 100.0) * 320.0).max(30.0) as u64;
```

- **CPU 0%** → 350ms entre frames (lento)
- **CPU 100%** → 30ms entre frames (rápido)

El loop principal corre en el hilo principal. `handle.update()` de ksni sincroniza los cambios de estado al tray de forma thread-safe.

---

## Assets

Los frames de Nox viven en `assets/` durante el desarrollo y se embeben en el binario en compilación con `include_bytes!`:

| Animación | Frames light | Frames dark | Estado |
|---|---|---|---|
| Run | `nox_0..7.png` | `nox_dark_0..7.png` | ✅ Disponible |
| Walk | `walk_0..7.png` | `walk_dark_0..7.png` | 🔒 Reservado (Nightfall Tactics) |
| Crouch Walk | `crouchwalk_0..9.png` | `crouchwalk_dark_0..9.png` | 🔒 Futuro |
| Climb Back | `climbback_0..3.png` | `climbback_dark_0..3.png` | 🔒 Futuro |
| Shoot 2H | `shoot2h_0..9.png` | `shoot2h_dark_0..9.png` | 🔒 Reservado (Nightfall Tactics) |

---

## Detección de tema

Lee `~/.config/kdeglobals` y detecta si contiene `BreezeDark` o `Breeze Dark`. Si no puede leer el archivo, asume tema claro.

---

## Distribución

| Canal | Estado | Comando |
|---|---|---|
| AUR | ✅ Publicado | `paru -S tacticaltray-linux` |
| GitHub Releases | ⏳ Pendiente binario precompilado | — |

El PKGBUILD descarga el tarball del tag de GitHub, compila desde fuente con `cargo build --release --locked` e instala el binario en `/usr/bin/tacticaltray-linux`.

---

## Autostart KDE

```ini
# ~/.config/autostart/tacticaltray-linux.desktop
[Desktop Entry]
Type=Application
Name=TacticalTray
Exec=tacticaltray-linux
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
```

---

## Estado del roadmap

### DONE ✅
- Ícono de Nox en el system tray de KDE Wayland
- Animación de Nox con velocidad proporcional al CPU
- Detección automática de tema claro/oscuro
- Métricas en tiempo real: CPU, GPU, RAM, temperatura, disco, red
- Frames embebidos en el binario (`include_bytes!`)
- Publicación en el AUR
- Autostart configurado en KDE

### BACKLOG
- Sistema de progresión de kilómetros y desbloqueo de animaciones
- Release de binario precompilado en GitHub (evitar compilación de 233MB)
- Soporte para animaciones adicionales (Walk, Crouch Walk, Climb Back)
- Integración con Nightfall Tactics para desbloqueo de Walk y Shoot 2H
- Widget de escritorio (Fase 2)

---

## Pendientes técnicos conocidos

- El binario se compiló sin `--locked` verificado en el PKGBUILD — confirmar que `Cargo.lock` está en el repo
- GPU readings dependen de `/sys/class/drm/card0/` — puede no funcionar en todas las configuraciones de hardware
- La detección de tema solo lee `kdeglobals` — no reacciona a cambios de tema en caliente
- El PKGBUILD descarga y compila ~233MB de dependencias Rust — mejorar con binario precompilado en v1.0.1

---

## Decisiones de arquitectura

| Decisión | Razón |
|---|---|
| `ksni` sobre `tray-icon` | `tray-icon` con libayatana falla al actualizar íconos frecuentemente en Wayland/KDE; ksni usa StatusNotifierItem nativo |
| `include_bytes!` para assets | Binario autónomo sin dependencias de archivos externos |
| Hilo principal para animación | ksni maneja D-Bus internamente; `handle.update()` sincroniza estado de forma segura |
| Lectura de GPU desde sysfs | `sysinfo` no expone GPU usage en Linux; sysfs es el estándar del kernel |
| Solo partición `/` en disco | Evitar mostrar todos los mount points del sistema que confunden al usuario |
| Filtrar interfaces de red con tráfico > 0 | Mostrar solo interfaces activas, omitir loopback y adaptadores sin actividad |

---

## Notas de desarrollo

- Rama principal: `main`
- Lenguaje del sistema operativo de desarrollo: CachyOS (Arch-based) con KDE Plasma + Wayland
- Shell: fish — los heredocs (`<< 'EOF'`) no funcionan; usar `echo '...' >` o archivos directos
- AUR repo: `ssh://aur@aur.archlinux.org/tacticaltray-linux.git`
- Para actualizar el AUR: editar PKGBUILD + regenerar `.SRCINFO` con `makepkg --printsrcinfo > .SRCINFO` + push
- Deepwiki disponible en `deepwiki.com/Sekain555/tacticaltray-linux`
