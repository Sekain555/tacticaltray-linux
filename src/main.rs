use sysinfo::{System, Disks, Networks};
use ksni::blocking::TrayMethods;

struct NoxTray {
    frame_index: usize,
    frames_light: Vec<Vec<u8>>,
    frames_dark: Vec<Vec<u8>>,
    dark_mode: bool,
    cpu_usage: f32,
    sys_info: String,
}

impl NoxTray {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let cpu_usage = sys.global_cpu_usage();
        let sys_info = Self::build_info(&sys);
        let frames_light = Self::load_frames(false);
        let frames_dark = Self::load_frames(true);
        let dark_mode = Self::detect_dark_mode();

        NoxTray {
            frame_index: 0,
            frames_light,
            frames_dark,
            dark_mode,
            cpu_usage,
            sys_info,
        }
    }

    fn load_frames(dark: bool) -> Vec<Vec<u8>> {
        (0..8).map(|i| {
            let path = if dark {
                format!("assets/nox_dark_{}.png", i)
            } else {
                format!("assets/nox_{}.png", i)
            };
            std::fs::read(&path)
                .unwrap_or_else(|_| std::fs::read("assets/icon.png").unwrap())
        }).collect()
    }

    fn detect_dark_mode() -> bool {
        std::fs::read_to_string(
            format!("{}/.config/kdeglobals", std::env::var("HOME").unwrap_or_default())
        )
        .map(|c| c.contains("BreezeDark") || c.contains("Breeze Dark"))
        .unwrap_or(false)
    }

    fn build_info(sys: &System) -> String {
        let cpu = sys.global_cpu_usage();
        let total_ram = sys.total_memory() / 1024 / 1024;
        let used_ram = sys.used_memory() / 1024 / 1024;

        let cpu_temp = {
            let components = sysinfo::Components::new_with_refreshed_list();
            components.iter()
                .find(|c| {
                    let label = c.label().to_lowercase();
                    label.contains("cpu") || label.contains("core") || label.contains("package")
                })
                .and_then(|c| c.temperature())
                .map(|t| format!("{:.1}°C", t))
                .unwrap_or_else(|| "N/A".to_string())
        };

        let gpu_usage = std::fs::read_to_string("/sys/class/drm/card0/device/gpu_busy_percent")
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .map(|v| format!("{}%", v))
            .unwrap_or_else(|| "N/A".to_string());

        let gpu_temp = std::fs::read_to_string("/sys/class/drm/card0/device/hwmon/hwmon0/temp1_input")
            .ok()
            .and_then(|s| s.trim().parse::<f32>().ok())
            .map(|v| format!("{:.1}°C", v / 1000.0))
            .unwrap_or_else(|| "N/A".to_string());

        let disks = Disks::new_with_refreshed_list();
        let disk_info = disks.iter()
            .find(|d| d.mount_point().to_string_lossy() == "/")
            .map(|d| {
                let total = d.total_space() / 1024 / 1024 / 1024;
                let available = d.available_space() / 1024 / 1024 / 1024;
                format!("💾 Disco: {} GB / {} GB", total - available, total)
            })
            .unwrap_or_else(|| "💾 Disco: N/A".to_string());

        let networks = Networks::new_with_refreshed_list();
        let net_info: String = networks.iter()
            .filter(|(name, data)| !name.starts_with("lo") && data.total_received() > 0)
            .map(|(name, data)| {
                let rx = data.total_received() / 1024;
                let tx = data.total_transmitted() / 1024;
                format!("🌐 {}: ↓{} KB ↑{} KB", name, rx, tx)
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "🖥  CPU: {:.1}% | 🌡 {}\n🎮 GPU: {} | 🌡 {}\n🧠 RAM: {} MB / {} MB\n{}\n{}",
            cpu, cpu_temp, gpu_usage, gpu_temp, used_ram, total_ram, disk_info, net_info
        )
    }

    fn current_frame_pixmap(&self) -> Vec<ksni::Icon> {
        let data = if self.dark_mode {
            &self.frames_dark[self.frame_index]
        } else {
            &self.frames_light[self.frame_index]
        };

        let img = image::load_from_memory(data).unwrap().to_rgba8();
        let (width, height) = img.dimensions();
        let argb: Vec<u8> = img.chunks(4)
            .flat_map(|px| [px[3], px[0], px[1], px[2]])
            .collect();

        vec![ksni::Icon { width: width as i32, height: height as i32, data: argb }]
    }
}

impl ksni::Tray for NoxTray {
    fn id(&self) -> String {
        "tacticaltray".to_string()
    }

    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        self.current_frame_pixmap()
    }

    fn title(&self) -> String {
        format!("TacticalTray | CPU: {:.1}%", self.cpu_usage)
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: self.sys_info.clone(),
                enabled: false,
                ..Default::default()
            }.into(),
            MenuItem::Separator,
            StandardItem {
                label: "Quit".to_string(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }.into(),
        ]
    }
}

fn main() {
    let handle = NoxTray::new().spawn().unwrap();

    println!("TacticalTray corriendo en el tray!");

    let mut sys = System::new_all();
    let mut frame_index = 0usize;

    loop {
        sys.refresh_cpu_all();
        let cpu = sys.global_cpu_usage();
        let interval_ms = (350.0 - (cpu / 100.0) * 320.0).max(30.0) as u64;

        frame_index = (frame_index + 1) % 8;

        // Cada 10 frames (~1 segundo) actualizamos métricas completas
        let update_metrics = frame_index == 0;
        if update_metrics {
            sys.refresh_all();
        }

        let info = if update_metrics {
            NoxTray::build_info(&sys)
        } else {
            String::new()
        };

        let next_frame = frame_index;
        let cpu_snap = cpu;

        handle.update(|tray: &mut NoxTray| {
            tray.frame_index = next_frame;
            tray.cpu_usage = cpu_snap;
            if !info.is_empty() {
                tray.sys_info = info.clone();
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(interval_ms));
    }
}