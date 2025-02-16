use eframe::egui::{
    self, CentralPanel, CollapsingHeader, Color32, Context, ScrollArea, SidePanel, TextEdit,
    TopBottomPanel, Visuals,
};
use eframe::NativeOptions;
use rfd::FileDialog;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, PartialEq)]
enum BitsMode {
    BITS16,
    BITS32,
    BITS64,
}

impl Default for BitsMode {
    fn default() -> Self {
        BitsMode::BITS64
    }
}

impl BitsMode {
    fn bits_mode_str(&self) -> &'static str {
        match self {
            BitsMode::BITS16 => "16",
            BitsMode::BITS32 => "32",
            BitsMode::BITS64 => "64",
        }
    }
}

#[derive(Default)]
struct App {
    asm_files: Vec<String>,
    output_folder: Option<String>,
    log: Arc<Mutex<String>>,
    progress: Arc<Mutex<f32>>,
    dark_mode: bool,
    hex_preview: Arc<Mutex<String>>,
    latest_hex_file: Arc<Mutex<Option<String>>>,
    bits_mode: BitsMode,
    auto_insert_bits: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(if self.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        });
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("🚀asm2hex");
                ui.separator();
                if ui.button("🌗 Toggle Theme").clicked() {
                    self.dark_mode = !self.dark_mode;
                }
                ui.menu_button("File", |ui| {
                    if ui.button("Clear Logs").clicked() {
                        let mut lg = self.log.lock().unwrap();
                        lg.clear();
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("View Documentation").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("About").clicked() {
                        ui.close_menu();
                    }
                });
            });
        });
        SidePanel::left("side_panel")
            .resizable(true)
            .min_width(250.0)
            .show(ctx, |ui| {
                ui.label("A professional, multi-arch converter.");
                ui.separator();
                CollapsingHeader::new("Bits Mode Settings")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("Choose bits for your assembly:");
                        if ui
                            .radio_value(&mut self.bits_mode, BitsMode::BITS16, "16-bit")
                            .clicked()
                        {}
                        if ui
                            .radio_value(&mut self.bits_mode, BitsMode::BITS32, "32-bit")
                            .clicked()
                        {}
                        if ui
                            .radio_value(&mut self.bits_mode, BitsMode::BITS64, "64-bit")
                            .clicked()
                        {}
                        ui.checkbox(&mut self.auto_insert_bits, "Auto-insert [bits X]");
                    });
                ui.separator();
                CollapsingHeader::new("File Selection")
                    .default_open(true)
                    .show(ui, |ui| {
                        if ui.button("📂 Add .asm Files").clicked() {
                            if let Some(paths) = FileDialog::new()
                                .add_filter("Assembly Files", &["asm"])
                                .pick_files()
                            {
                                for path in paths {
                                    self.asm_files.push(path.display().to_string());
                                }
                            }
                        }
                        ui.separator();
                        if self.asm_files.is_empty() {
                            ui.label("No files selected.");
                        } else {
                            ui.label("Selected files:");
                            ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                                for f in self.asm_files.clone() {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("📄 {}", f));
                                        if ui.button("❌").clicked() {
                                            self.asm_files.retain(|x| x != &f);
                                        }
                                    });
                                }
                            });
                        }
                    });
                ui.separator();
                CollapsingHeader::new("Output Folder")
                    .default_open(true)
                    .show(ui, |ui| {
                        if ui.button("📁 Choose Folder").clicked() {
                            if let Some(folder) = FileDialog::new().pick_folder() {
                                self.output_folder = Some(folder.display().to_string());
                            }
                        }
                        if let Some(ref folder) = self.output_folder {
                            ui.horizontal(|ui| {
                                ui.label(format!("📂 {}", folder));
                                if ui.button("📋").clicked() {
                                    ui.ctx().output_mut(|o| o.copied_text = folder.clone());
                                }
                            });
                        } else {
                            ui.label("No output folder selected.");
                        }
                    });
                ui.separator();
                if ui.button("⚡ Convert to HEX").clicked() {
                    if !self.asm_files.is_empty() {
                        let files = self.asm_files.clone();
                        let out_folder = self
                            .output_folder
                            .clone()
                            .unwrap_or_else(|| ".".to_string());
                        let log_ref = Arc::clone(&self.log);
                        let progress_ref = Arc::clone(&self.progress);
                        let hex_preview_ref = Arc::clone(&self.hex_preview);
                        let latest_file_ref = Arc::clone(&self.latest_hex_file);
                        let bits_mode = self.bits_mode;
                        let auto_insert = self.auto_insert_bits;
                        thread::spawn(move || {
                            let total = files.len() as f32;
                            let mut done = 0.0;
                            for file_path in files {
                                let file_stem = Path::new(&file_path)
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("output");
                                let bin_file = format!("{}/{}.bin", out_folder, file_stem);
                                let hex_file = format!("{}/{}.hex", out_folder, file_stem);
                                convert_asm_file(
                                    &file_path,
                                    &bin_file,
                                    &hex_file,
                                    bits_mode,
                                    auto_insert,
                                    Arc::clone(&log_ref),
                                    Arc::clone(&hex_preview_ref),
                                    Arc::clone(&latest_file_ref),
                                );
                                done += 1.0;
                                *progress_ref.lock().unwrap() = done / total;
                            }
                            *progress_ref.lock().unwrap() = 1.0;
                        });
                    }
                }
                let progress_val = *self.progress.lock().unwrap();
                if progress_val > 0.0 && progress_val < 1.0 {
                    ui.separator();
                    ui.label("⏳ Converting...");
                    ui.add(egui::ProgressBar::new(progress_val).desired_width(200.0));
                }
                ui.separator();
                CollapsingHeader::new("Logs (color-coded)")
                    .default_open(true)
                    .show(ui, |ui| {
                        let log_txt = self.log.lock().unwrap().clone();
                        ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                            display_colored_logs(ui, &log_txt);
                        });
                    });
            });
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("📜 Full HEX Preview (Entire File)");
            ui.separator();
            if let Some(ref hex_file) = *self.latest_hex_file.lock().unwrap() {
                if let Some(preview) = read_full_hex(hex_file) {
                    let mut content = self.hex_preview.lock().unwrap();
                    *content = preview;
                }
            }
            let txt = self.hex_preview.lock().unwrap().clone();
            ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut txt.clone())
                        .desired_rows(25)
                        .desired_width(f32::INFINITY),
                );
            });
        });
        ctx.request_repaint();
    }
}

fn convert_asm_file(
    asm_path: &str,
    bin_path: &str,
    hex_path: &str,
    bits_mode: BitsMode,
    auto_insert: bool,
    log_ref: Arc<Mutex<String>>,
    hex_preview: Arc<Mutex<String>>,
    latest_file: Arc<Mutex<Option<String>>>,
) {
    {
        let mut l = log_ref.lock().unwrap();
        l.clear();
        l.push_str(&format!("🔍 Processing {}\n", asm_path));
    }
    if auto_insert {
        maybe_insert_bits(asm_path, bits_mode, &log_ref);
    }
    let out_asm = Command::new("nasm")
        .args(["-f", "bin", asm_path, "-o", bin_path])
        .output();
    if let Ok(output) = out_asm {
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            let mut l = log_ref.lock().unwrap();
            l.push_str(&format!("❌ NASM Error:\n{}\n", err));
            return;
        }
    } else {
        let mut l = log_ref.lock().unwrap();
        l.push_str("❌ Error: Could not run nasm.\n");
        return;
    }
    let oc = Command::new("objcopy")
        .args(["-I", "binary", "-O", "ihex", bin_path, hex_path])
        .output();
    if let Ok(output) = oc {
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            let mut l = log_ref.lock().unwrap();
            l.push_str(&format!("❌ objcopy Error:\n{}\n", err));
            return;
        }
    } else {
        let mut l = log_ref.lock().unwrap();
        l.push_str("❌ Error: Could not run objcopy.\n");
        return;
    }
    {
        let mut l = log_ref.lock().unwrap();
        l.push_str(&format!("✅ Success! HEX saved at {}\n", hex_path));
    }
    {
        let mut lf = latest_file.lock().unwrap();
        *lf = Some(hex_path.to_string());
    }
    if let Some(preview) = read_full_hex(hex_path) {
        let mut hp = hex_preview.lock().unwrap();
        *hp = preview;
    }
}

fn maybe_insert_bits(asm_path: &str, bits_mode: BitsMode, log_ref: &Arc<Mutex<String>>) {
    let file_data = match std::fs::read_to_string(asm_path) {
        Ok(s) => s,
        Err(e) => {
            let mut l = log_ref.lock().unwrap();
            l.push_str(&format!("⚠️ Could not read {}: {}\n", asm_path, e));
            return;
        }
    };
    if file_data.contains("[bits") || file_data.contains("bits ") {
        return;
    }
    let bits_str = bits_mode.bits_mode_str();
    let new_data = format!("[bits {}]\n{}", bits_str, file_data);
    if let Err(e) = std::fs::write(asm_path, new_data) {
        let mut l = log_ref.lock().unwrap();
        l.push_str(&format!("⚠️ Could not insert [bits {}]: {}\n", bits_str, e));
    } else {
        let mut l = log_ref.lock().unwrap();
        l.push_str(&format!(
            "ℹ️ Inserted [bits {}] into {}\n",
            bits_str, asm_path
        ));
    }
}

fn read_full_hex(hex_path: &str) -> Option<String> {
    let f = File::open(hex_path).ok()?;
    let rdr = io::BufReader::new(f);
    let lines = rdr
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>()
        .join("\n");
    Some(lines)
}

fn display_colored_logs(ui: &mut egui::Ui, log_text: &str) {
    for line in log_text.lines() {
        let mut color = Color32::WHITE;
        if line.contains("❌") || line.contains("Error") {
            color = Color32::RED;
        } else if line.contains("✅") || line.contains("Success") {
            color = Color32::GREEN;
        } else if line.contains("⚠️") {
            color = Color32::YELLOW;
        }
        ui.colored_label(color, line);
    }
}

fn main() -> Result<(), eframe::Error> {
    let app = App {
        bits_mode: BitsMode::BITS64,
        auto_insert_bits: true,
        ..Default::default()
    };
    eframe::run_native(
        "ASM → HEX (Flat Binary) - Ultimate Professional UI",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(app)),
    )
}
