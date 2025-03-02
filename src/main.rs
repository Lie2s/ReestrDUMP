mod registry;

use eframe::egui;
use registry::{read_registry_info, get_users, get_network_info, get_bios_info, get_hardware_info, dump_registry_branch};
use serde_json::Value;
use std::env;

struct RegistryApp {
    os_info: String,
    users_info: String,
    network_info: String,
    bios_info: String,
    hardware_info: String,
    registry_path: String, 
    registry_dump: String, 
}

impl Default for RegistryApp {
    fn default() -> Self {
        Self {
            os_info: "Нажмите кнопку для загрузки данных...".to_string(),
            users_info: "".to_string(),
            network_info: "".to_string(),
            bios_info: "".to_string(),
            hardware_info: "".to_string(),
            registry_path: String::new(),
            registry_dump: "".to_string(),
        }
    }
}

impl eframe::App for RegistryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Анализатор реестра Windows");

                        if ui.button("Получить информацию о системе").clicked() {
                            self.os_info = serde_json::to_string_pretty(&read_registry_info())
                                .unwrap_or_else(|e| format!("Ошибка: {}", e));
                        }
                        ui.label(&self.os_info);

                        if ui.button("Получить пользователей").clicked() {
                            self.users_info = serde_json::to_string_pretty(&get_users())
                                .unwrap_or_else(|e| format!("Ошибка: {}", e));
                        }
                        ui.label(&self.users_info);

                        if ui.button("Получить информацию о сети").clicked() {
                            self.network_info = serde_json::to_string_pretty(&get_network_info())
                                .unwrap_or_else(|e| format!("Ошибка: {}", e));
                        }
                        ui.label(&self.network_info);

                        if ui.button("Получить информацию о BIOS").clicked() {
                            self.bios_info = serde_json::to_string_pretty(&get_bios_info())
                                .unwrap_or_else(|e| format!("Ошибка: {}", e));
                        }
                        ui.label(&self.bios_info);

                        if ui.button("Получить информацию об оборудовании").clicked() {
                            self.hardware_info = serde_json::to_string_pretty(&get_hardware_info())
                                .unwrap_or_else(|e| format!("Ошибка: {}", e));
                        }
                        ui.label(&self.hardware_info);

                        // Поле ввода и кнопка для дампа ветки реестра
                        ui.horizontal(|ui| {
                            ui.label("Путь реестра (например, HKEY_LOCAL_MACHINE\\SOFTWARE):");
                            ui.text_edit_singleline(&mut self.registry_path);
                        });
                        if ui.button("Дамп ветки реестра").clicked() {
                            self.registry_dump = serde_json::to_string_pretty(&dump_registry_branch(&self.registry_path))
                                .unwrap_or_else(|e| format!("Ошибка: {}", e));
                        }
                        ui.label(&self.registry_dump);
                    });
                });
        });
    }
}

// Функция для запуска консольного режима
fn run_console_mode(args: Vec<String>) {
    if args.len() < 2 {
        println!("Использование: reestr <команда> [аргументы]");
        println!("Доступные команды:");
        println!("  os       - Информация о системе");
        println!("  users    - Информация о пользователях");
        println!("  network  - Информация о сети");
        println!("  bios     - Информация о BIOS");
        println!("  hardware - Информация об оборудовании");
        println!("  dump <путь> - Дамп указанной ветки реестра (например, HKEY_LOCAL_MACHINE\\SOFTWARE)");
        return;
    }

    match args[1].as_str() {
        "os" => {
            let result = serde_json::to_string_pretty(&read_registry_info())
                .unwrap_or_else(|e| format!("Ошибка: {}", e));
            println!("{}", result);
        }
        "users" => {
            let result = serde_json::to_string_pretty(&get_users())
                .unwrap_or_else(|e| format!("Ошибка: {}", e));
            println!("{}", result);
        }
        "network" => {
            let result = serde_json::to_string_pretty(&get_network_info())
                .unwrap_or_else(|e| format!("Ошибка: {}", e));
            println!("{}", result);
        }
        "bios" => {
            let result = serde_json::to_string_pretty(&get_bios_info())
                .unwrap_or_else(|e| format!("Ошибка: {}", e));
            println!("{}", result);
        }
        "hardware" => {
            let result = serde_json::to_string_pretty(&get_hardware_info())
                .unwrap_or_else(|e| format!("Ошибка: {}", e));
            println!("{}", result);
        }
        "dump" => {
            if args.len() < 3 {
                println!("Укажите путь реестра, например: reestr dump HKEY_LOCAL_MACHINE\\SOFTWARE");
            } else {
                let path = &args[2];
                let result = serde_json::to_string_pretty(&dump_registry_branch(path))
                    .unwrap_or_else(|e| format!("Ошибка: {}", e));
                println!("{}", result);
            }
        }
        _ => println!("Неизвестная команда. Используйте: os, users, network, bios, hardware, dump"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        run_console_mode(args);
    } else {
        let options = eframe::NativeOptions::default();
        if let Err(e) = eframe::run_native(
            "Registry Analyzer",
            options,
            Box::new(|_cc| Box::new(RegistryApp::default())),
        ) {
            eprintln!("Ошибка запуска GUI: {}", e);
        }
    }
}