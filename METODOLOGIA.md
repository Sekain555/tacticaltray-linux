# Metodología de Desarrollo — TacticalTray Linux

> Documento específico para TacticalTray Linux, complementario a la metodología general de Sekain.
> Refleja las particularidades de desarrollar en Rust para Linux desde un entorno CachyOS/KDE.
> Última actualización: junio 2026.

---

## 1. Gestión de tareas

TacticalTray Linux es un proyecto personal de alcance acotado. No usa Trello activamente — el backlog vive en el `CONTEXTO.md` y los issues de GitHub.

Para features nuevas o bugs:
- Issues de GitHub como unidad de trabajo
- Una rama `feature/` por issue
- Merge directo a `main` con squash

---

## 2. Flujo de desarrollo — Git

### Estructura de ramas

```
feature/nombre-descriptivo → main
```

No existe rama `dev` — dado el tamaño del proyecto, `main` es la rama de integración.

### Ciclo por feature

```bash
git checkout main
git pull origin main
git checkout -b feature/nombre-descriptivo

# desarrollar...

git add archivo1 archivo2
git commit -m "feat: descripción corta"
git push origin feature/nombre-descriptivo
# Pull Request → Squash & Merge a main
```

### Convención de commits

| Prefijo | Uso |
|---|---|
| `feat:` | Nueva funcionalidad |
| `fix:` | Corrección de bug |
| `chore:` | Configuración, documentación, AUR |

---

## 3. Particularidades de Rust

### Compilación

```bash
cargo build          # debug — rápido, sin optimizaciones
cargo build --release # release — lento, binario optimizado
cargo run            # compila y ejecuta en debug
```

### Dependencias

Las dependencias se declaran en `Cargo.toml`. `Cargo.lock` debe commitearse — es necesario para `cargo build --release --locked` en el PKGBUILD del AUR.

### Embeber assets

Los frames PNG de Nox se embeben en el binario con `include_bytes!`:

```rust
include_bytes!("../assets/nox_0.png").to_vec()
```

Esto garantiza que el binario sea autónomo y no dependa de archivos externos en tiempo de ejecución.

---

## 4. Publicación en el AUR

### Estructura del repositorio AUR

El AUR tiene su propio repositorio Git separado del código fuente:

```
~/Proyectos/SOFTWARE/aur-tacticaltray/
  PKGBUILD
  .SRCINFO
```

### Flujo de actualización

```bash
# 1. Actualizar versión en Cargo.toml
# 2. Compilar y verificar que funciona
cargo build --release

# 3. Crear tag en GitHub
git tag v1.X.0
git push origin v1.X.0

# 4. Obtener nuevo sha256
curl -L "https://github.com/Sekain555/tacticaltray-linux/archive/refs/tags/v1.X.0.tar.gz" -o /tmp/tt.tar.gz
sha256sum /tmp/tt.tar.gz

# 5. Actualizar PKGBUILD
# - pkgver=1.X.0
# - sha256sums=('nuevo_hash')

# 6. Regenerar .SRCINFO
cd ~/Proyectos/SOFTWARE/aur-tacticaltray
makepkg --printsrcinfo > .SRCINFO

# 7. Push al AUR
git add PKGBUILD .SRCINFO
git commit -m "chore: release v1.X.0"
git push origin master

# 8. Limpiar caché y verificar instalación
rm -rf ~/.cache/paru/clone/tacticaltray-linux
paru -S tacticaltray-linux
```

### Notas importantes del AUR

- El repositorio AUR usa la rama `master`, no `main`
- El sha256 debe ser solo el hash, sin el nombre del archivo
- `makepkg --printsrcinfo` debe ejecutarse después de cada cambio al PKGBUILD
- Limpiar el caché de paru antes de probar una nueva versión

---

## 5. Entorno de desarrollo — CachyOS

### Shell

CachyOS usa **fish** como shell por defecto. Diferencias importantes:

| Bash/Zsh | Fish |
|---|---|
| `source ~/.cargo/env` | `fish_add_path ~/.cargo/bin` |
| `export PATH="$HOME/.cargo/bin:$PATH"` | `fish_add_path ~/.cargo/bin` |
| `<< 'EOF'` heredoc | No soportado — usar `echo '...' >` |
| `VAR=value comando` | `env VAR=value comando` |

### Dependencias del sistema

```bash
sudo pacman -S libayatana-appindicator xdotool
```

### Herramientas

- **Editor:** VSCodium con extensión `rust-analyzer`
- **AUR helper:** paru
- **Portapapeles Wayland:** `wl-copy` / `wl-paste` (paquete `wl-clipboard`)

---

## 6. Decisiones de diseño — Rust

El desarrollador toma las decisiones de arquitectura y diseño. El asistente puede proponer alternativas, pero la decisión final es del desarrollador.

Ante cambios que afecten la API pública del tray (menú, íconos, comportamiento), consultar antes de implementar.

---

## 7. Principios transversales

- **Binario autónomo** — sin dependencias de archivos externos en tiempo de ejecución
- **`Cargo.lock` siempre commiteado** — garantiza builds reproducibles en el AUR
- **Probar instalación desde el AUR** antes de declarar una release estable
- **Un tag = una release** — no reusar tags existentes
- **CONTEXTO.md siempre actualizado** al cerrar cada feature

---

*Este documento se actualiza cuando la metodología o el entorno de desarrollo cambia.*
