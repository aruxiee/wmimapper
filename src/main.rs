#![allow(dead_code)]
#![allow(unused_imports)]

use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};
use rayon::prelude::*;
use std::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_OperatingSystem")]
#[serde(rename_all = "PascalCase")]
struct OSInfo {
    caption: String,
    version: String,
    os_architecture: String,
    serial_number: String,
    last_boot_up_time: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_Processor")]
#[serde(rename_all = "PascalCase")]
struct CPUInfo {
    name: String,
    number_of_cores: u32,
    processor_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_LogicalDisk")]
#[serde(rename_all = "PascalCase")]
struct DiskInfo {
    device_id: String,
    size: Option<u64>,
    free_space: Option<u64>,
}

fn main() {
    let subnet = "192.168.1"; 
    let mut targets = Vec::new();

    println!("[!] performing discovery on {}.0/24...", subnet);
    println!("[!] encryption enabled.");

    for i in 1..255 {
        targets.push(format!("{}.{}", subnet, i));
    }

    targets.into_par_iter().for_each(|ip| {
        if let Err(e) = deep_dump_encrypted(&ip) {
            let err_msg = e.to_string();
            if !err_msg.contains("0x800706ba") && !err_msg.contains("0x80070005") {
                eprintln!("[-] {}: {}", ip, err_msg);
            }
        }
    });
}

fn deep_dump_encrypted(ip: &str) -> Result<(), Box<dyn Error>> {
    let com_lib = COMLibrary::new()?;
    
    let namespace = if ip == "127.0.0.1" || ip == "localhost" {
        "root\\cimv2".to_string()
    } else {
        format!("\\\\{}\\root\\cimv2", ip)
    };

    let wmi_con = WMIConnection::with_namespace_path(&namespace, com_lib.into())?;

    println!("\n[+] found active endpoint: {}", ip);

    let os_list: Vec<OSInfo> = wmi_con.query()?;
    for os in os_list {
        println!("    [os] {} | sn: {}", os.caption, os.serial_number);
        println!("         last boot: {}", os.last_boot_up_time);
    }

    let cpu_list: Vec<CPUInfo> = wmi_con.query()?;
    for cpu in cpu_list {
        println!("    [cpu] {} ({} cores) | id: {}", cpu.name.trim(), cpu.number_of_cores, cpu.processor_id);
    }

    let disk_list: Vec<DiskInfo> = wmi_con.query()?;
    for disk in disk_list {
        if let Some(s) = disk.size {
            let free = disk.free_space.unwrap_or(0) / 1024 / 1024 / 1024;
            println!("    [disk] {}: {} gb free / {} gb total", disk.device_id, free, s / 1024 / 1024 / 1024);
        }
    }

    Ok(())
}