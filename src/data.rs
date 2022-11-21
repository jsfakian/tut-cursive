use std::collections::HashMap;

pub const NETWORKING: &str = "Networking";
pub const SUBNET: &str = "Subnet";
pub const GATEWAY: &str = "Gateway";
pub const DNS: &str = "DNS";
pub const INSTALL_SERVER: &str = "Eve_install_server";
pub const INSTALL_DISK: &str = "Eve_install_disk";
pub const PERSIST_DISK: &str = "Eve_persist_disk";
pub const SOFT_SERIAL: &str = "Eve_soft_serial";
pub const REBOOT_AFTER_INSTALL: &str = "Eve_reboot_after_install";
pub const PAUSE_AFTER_INSTALL: &str = "Eve_pause_after_install";
pub const PAUSE_BEFORE_INSTALL: &str = "Eve_pause_before_install";
pub const ROOT: &str = "Root";
pub const FIND_BOOT: &str = "Find_boot";
pub const CONSOLE: &str = "Console";
pub const CONFIG: &str = "General Config";
pub const FS: &str = "FS";
pub const BUTTONS: &str = "Buttons";
pub const RAID: &str = "RAID";
pub const NIC: &str = "NIC";
pub const OVERVIEW: &str = "Overview";

const LABELS: [&str; 16] = [FS, RAID, NIC, SUBNET, GATEWAY, DNS, INSTALL_SERVER, INSTALL_DISK, PERSIST_DISK, SOFT_SERIAL, REBOOT_AFTER_INSTALL, PAUSE_AFTER_INSTALL, PAUSE_BEFORE_INSTALL, ROOT, FIND_BOOT, CONSOLE];

const VALUES: [&str; 16] = ["0", "0", "0", "", "", "", "", "####", "####", "", "", "","", "", "", ""];

#[derive(Debug, Clone)]
pub struct Field {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub map: HashMap<String, String>
}

impl Data {
    pub fn new() -> Self {
        let mut data = Self { 
            map: HashMap::new(),
        };
        for (i, label) in LABELS.iter().enumerate() {
            data.map.insert(label.to_string(), VALUES[i].to_string());
        }
        data
    }
}
