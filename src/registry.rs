use winreg::enums::*;
use winreg::RegKey;
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn read_registry_info() -> Value {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut info: HashMap<String, Value> = HashMap::new();

    if let Ok(software) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
        info.insert("ProductName".to_string(), json!(software.get_value::<String, _>("ProductName").unwrap_or("Unknown".to_string())));
        info.insert("CurrentVersion".to_string(), json!(software.get_value::<String, _>("CurrentVersion").unwrap_or("Unknown".to_string())));
        info.insert("EditionID".to_string(), json!(software.get_value::<String, _>("EditionID").unwrap_or("Unknown".to_string())));
        info.insert("DisplayVersion".to_string(), json!(software.get_value::<String, _>("DisplayVersion").unwrap_or("Unknown".to_string())));
        info.insert("CurrentBuild".to_string(), json!(software.get_value::<String, _>("CurrentBuild").unwrap_or("Unknown".to_string())));
        info.insert("UBR".to_string(), json!(software.get_value::<u32, _>("UBR").map(|v| v.to_string()).unwrap_or("Unknown".to_string())));
        info.insert("InstallDate".to_string(), json!(software.get_value::<u32, _>("InstallDate").map(|v| v.to_string()).unwrap_or("Unknown".to_string())));
        info.insert("RegisteredOwner".to_string(), json!(software.get_value::<String, _>("RegisteredOwner").unwrap_or("Unknown".to_string())));
    }

    if let Ok(computer) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName") {
        info.insert("ComputerName".to_string(), json!(computer.get_value::<String, _>("ComputerName").unwrap_or("Unknown".to_string())));
    }

    if let Ok(windows) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Windows") {
        info.insert("ShutdownTime".to_string(), json!(windows.get_value::<u64, _>("ShutdownTime").map(|v| v.to_string()).unwrap_or("Unknown".to_string())));
    }

    if let Ok(timezone) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\TimeZoneInformation") {
        info.insert("TimeZone".to_string(), json!(timezone.get_value::<String, _>("TimeZone").unwrap_or("Unknown".to_string())));
    }

    if info.is_empty() {
        json!({"error": "Не удалось открыть ключи реестра ОС"})
    } else {
        json!(info)
    }
}

pub fn get_users() -> Value {
    let hku = RegKey::predef(HKEY_USERS);
    let mut users_info: HashMap<String, Value> = HashMap::new();

    for sid in hku.enum_keys().filter_map(|x| x.ok()) {
        let mut user_data: HashMap<String, Value> = HashMap::new();

        if let Ok(volatile) = hku.open_subkey(&format!("{}\\Volatile Environment", sid)) {
            user_data.insert("APPDATA".to_string(), json!(volatile.get_value::<String, _>("APPDATA").unwrap_or("Unknown".to_string())));
            user_data.insert("USERNAME".to_string(), json!(volatile.get_value::<String, _>("USERNAME").unwrap_or("Unknown".to_string())));
            user_data.insert("USERDOMAIN".to_string(), json!(volatile.get_value::<String, _>("USERDOMAIN").unwrap_or("Unknown".to_string())));
            user_data.insert("LOCALAPPDATA".to_string(), json!(volatile.get_value::<String, _>("LOCALAPPDATA").unwrap_or("Unknown".to_string())));
            user_data.insert("USERPROFILE".to_string(), json!(volatile.get_value::<String, _>("USERPROFILE").unwrap_or("Unknown".to_string())));
        }

        if let Ok(env) = hku.open_subkey(&format!("{}\\Environment", sid)) {
            let mut env_values: HashMap<String, String> = HashMap::new();
            for value in env.enum_values().filter_map(|x| x.ok()) {
                env_values.insert(value.0.clone(), value.1.to_string());
            }
            user_data.insert("Environment".to_string(), json!(env_values));
        }

        if let Ok(intl) = hku.open_subkey(&format!("{}\\Control Panel\\International", sid)) {
            user_data.insert("LocaleName".to_string(), json!(intl.get_value::<String, _>("LocaleName").unwrap_or("Unknown".to_string())));
            user_data.insert("sShortDate".to_string(), json!(intl.get_value::<String, _>("sShortDate").unwrap_or("Unknown".to_string())));
            user_data.insert("sNativeDigits".to_string(), json!(intl.get_value::<String, _>("sNativeDigits").unwrap_or("Unknown".to_string())));
        }

        if let Ok(desktop) = hku.open_subkey(&format!("{}\\Control Panel\\Desktop", sid)) {
            user_data.insert("WallPaper".to_string(), json!(desktop.get_value::<String, _>("WallPaper").unwrap_or("Unknown".to_string())));
        }

        if !user_data.is_empty() {
            users_info.insert(sid, json!(user_data));
        }
    }

    if users_info.is_empty() {
        json!({"error": "Не удалось получить данные о пользователях"})
    } else {
        json!(users_info)
    }
}

pub fn get_network_info() -> Value {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut info: HashMap<String, Value> = HashMap::new();

    if let Ok(tcpip) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters") {
        info.insert("DhcpIPAddress".to_string(), json!(tcpip.get_value::<String, _>("DhcpIPAddress").unwrap_or("Unknown".to_string())));
        info.insert("Domain".to_string(), json!(tcpip.get_value::<String, _>("Domain").unwrap_or("Unknown".to_string())));
    }

    if let Ok(interfaces) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces") {
        for iface in interfaces.enum_keys().filter_map(|x| x.ok()) {
            if let Ok(iface_key) = interfaces.open_subkey(&iface) {
                let mut iface_info: HashMap<String, Value> = HashMap::new();
                iface_info.insert("DhcpIPAddress".to_string(), json!(iface_key.get_value::<String, _>("DhcpIPAddress").unwrap_or("Unknown".to_string())));
                iface_info.insert("DhcpNameServer".to_string(), json!(iface_key.get_value::<String, _>("DhcpNameServer").unwrap_or("Unknown".to_string())));
                iface_info.insert("DhcpServer".to_string(), json!(iface_key.get_value::<String, _>("DhcpServer").unwrap_or("Unknown".to_string())));
                iface_info.insert("DhcpSubnetMask".to_string(), json!(iface_key.get_value::<String, _>("DhcpSubnetMask").unwrap_or("Unknown".to_string())));
                iface_info.insert("DhcpDefaultGateway".to_string(), json!(iface_key.get_value::<String, _>("DhcpDefaultGateway").unwrap_or("Unknown".to_string())));
                info.insert(format!("Interface_{}", iface), json!(iface_info));
            }
        }
    }

    if info.is_empty() {
        json!({"error": "Не удалось открыть ключ сети"})
    } else {
        json!(info)
    }
}

pub fn get_bios_info() -> Value {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut info: HashMap<String, Value> = HashMap::new();

    if let Ok(system_desc) = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System") {
        info.insert("SystemBiosVersion".to_string(), json!(system_desc.get_value::<String, _>("SystemBiosVersion").unwrap_or("Unknown".to_string())));
        info.insert("SystemBiosDate".to_string(), json!(system_desc.get_value::<String, _>("SystemBiosDate").unwrap_or("Unknown".to_string())));
        info.insert("VideoBiosVersion".to_string(), json!(system_desc.get_value::<String, _>("VideoBiosVersion").unwrap_or("Unknown".to_string())));
    }

    if let Ok(bios_desc) = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\BIOS") {
        info.insert("BIOSVendor".to_string(), json!(bios_desc.get_value::<String, _>("BIOSVendor").unwrap_or("Unknown".to_string())));
        info.insert("BIOSVersion".to_string(), json!(bios_desc.get_value::<String, _>("BIOSVersion").unwrap_or("Unknown".to_string())));
        info.insert("BIOSReleaseDate".to_string(), json!(bios_desc.get_value::<String, _>("BIOSReleaseDate").unwrap_or("Unknown".to_string())));
    }

    if let Ok(sys_info) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\SystemInformation") {
        info.insert("SystemBIOSVersion".to_string(), json!(sys_info.get_value::<String, _>("BIOSVersion").unwrap_or("Unknown".to_string())));
        info.insert("SystemBIOSReleaseDate".to_string(), json!(sys_info.get_value::<String, _>("BIOSReleaseDate").unwrap_or("Unknown".to_string())));
    }

    if info.is_empty() {
        json!({"error": "Не удалось открыть ключи BIOS"})
    } else {
        json!(info)
    }
}

pub fn get_hardware_info() -> Value {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut info: HashMap<String, Value> = HashMap::new();

    if let Ok(sys_info) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\SystemInformation") {
        info.insert("SystemManufacturer".to_string(), json!(sys_info.get_value::<String, _>("SystemManufacturer").unwrap_or("Unknown".to_string())));
        info.insert("SystemProductName".to_string(), json!(sys_info.get_value::<String, _>("SystemProductName").unwrap_or("Unknown".to_string())));
    }

    if let Ok(gpu) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0000") {
        info.insert("DriverDesc".to_string(), json!(gpu.get_value::<String, _>("DriverDesc").unwrap_or("Unknown".to_string())));
    }

    if let Ok(cpu_root) = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor") {
        for cpu in cpu_root.enum_keys().filter_map(|x| x.ok()) {
            if let Ok(cpu_key) = cpu_root.open_subkey(&cpu) {
                let mut cpu_info: HashMap<String, Value> = HashMap::new();
                cpu_info.insert("ProcessorNameString".to_string(), json!(cpu_key.get_value::<String, _>("ProcessorNameString").unwrap_or("Unknown".to_string())));
                cpu_info.insert("Identifier".to_string(), json!(cpu_key.get_value::<String, _>("Identifier").unwrap_or("Unknown".to_string())));
                cpu_info.insert("VendorIdentifier".to_string(), json!(cpu_key.get_value::<String, _>("VendorIdentifier").unwrap_or("Unknown".to_string())));
                cpu_info.insert("~MHz".to_string(), json!(cpu_key.get_value::<u32, _>("~MHz").map(|v| v.to_string()).unwrap_or("Unknown".to_string())));
                info.insert(format!("CPU_{}", cpu), json!(cpu_info));
            }
        }
    }

    if let Ok(scsi_root) = hklm.open_subkey("HARDWARE\\DEVICEMAP\\Scsi") {
        for scsi in scsi_root.enum_keys().filter_map(|x| x.ok()) {
            if let Ok(scsi_key) = scsi_root.open_subkey(&scsi) {
                for port in scsi_key.enum_keys().filter_map(|x| x.ok()) {
                    if let Ok(port_key) = scsi_key.open_subkey(&port) {
                        let mut scsi_info: HashMap<String, Value> = HashMap::new();
                        scsi_info.insert("Identifier".to_string(), json!(port_key.get_value::<String, _>("Identifier").unwrap_or("Unknown".to_string())));
                        scsi_info.insert("SerialNumber".to_string(), json!(port_key.get_value::<String, _>("SerialNumber").unwrap_or("Unknown".to_string())));
                        info.insert(format!("SCSI_{}_{}", scsi, port), json!(scsi_info));
                    }
                }
            }
        }
    }

    if info.is_empty() {
        json!({"error": "Не удалось открыть ключи оборудования"})
    } else {
        json!(info)
    }
}
pub fn dump_registry_branch(path: &str) -> Value {
    let root_key = match path.split('\\').next() {
        Some("HKEY_LOCAL_MACHINE") => RegKey::predef(HKEY_LOCAL_MACHINE),
        Some("HKEY_USERS") => RegKey::predef(HKEY_USERS),
        Some("HKEY_CURRENT_USER") => RegKey::predef(HKEY_CURRENT_USER),
        Some("HKEY_CLASSES_ROOT") => RegKey::predef(HKEY_CLASSES_ROOT),
        Some("HKEY_CURRENT_CONFIG") => RegKey::predef(HKEY_CURRENT_CONFIG),
        _ => return json!({"error": "Неверный корневой ключ реестра. Используйте HKEY_LOCAL_MACHINE, HKEY_USERS и т.д."}),
    };

    let sub_path = path.split_once('\\').map(|(_, sub)| sub).unwrap_or("");
    if sub_path.is_empty() {
        return json!({"error": "Укажите путь ветки после корневого ключа"});
    }

    match root_key.open_subkey_with_flags(sub_path, KEY_READ) {
        Ok(key) => dump_key_recursive(&key, sub_path),
        Err(e) => json!({"error": format!("Не удалось открыть ветку '{}': {}", path, e)}),
    }
}

fn dump_key_recursive(key: &RegKey, path: &str) -> Value {
    let mut result: HashMap<String, Value> = HashMap::new();

    let mut values: HashMap<String, Value> = HashMap::new();
    for value in key.enum_values().filter_map(|x| x.ok()) {
        values.insert(value.0.clone(), json!(value.1.to_string()));
    }
    if !values.is_empty() {
        result.insert("values".to_string(), json!(values));
    }

    let mut subkeys: HashMap<String, Value> = HashMap::new();
    for subkey_name in key.enum_keys().filter_map(|x| x.ok()) {
        if let Ok(subkey) = key.open_subkey_with_flags(&subkey_name, KEY_READ) {
            let subkey_path = format!("{}\\{}", path, subkey_name);
            subkeys.insert(subkey_name, dump_key_recursive(&subkey, &subkey_path));
        }
    }
    if !subkeys.is_empty() {
        result.insert("subkeys".to_string(), json!(subkeys));
    }

    if result.is_empty() {
        json!({"info": "Ветка пуста или недоступна"})
    } else {
        json!(result)
    }
}