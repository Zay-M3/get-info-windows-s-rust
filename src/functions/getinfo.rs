use sysinfo::{System, Networks, Disks, Users};
use std::net::UdpSocket;

use crate::utils::interfase::*;

/// The function `get_system_os` in Rust retrieves information about the system's operating system,
/// including OS name, version, kernel version, hostname, and uptime.
/// 
/// Returns:
/// 
/// The function `get_system_os` is returning an instance of the `SistemaOperativo` struct, which
/// contains information about the system's operating system, version, kernel version, hostname, uptime
/// in seconds, and uptime in days.
pub fn get_system_os() -> SistemaOperativo {

    let uptime = System::uptime();
    let sistema_operativo = SistemaOperativo {
        os: System::name().unwrap_or_default(),
        version: System::os_version().unwrap_or_default(),
        kernel: System::kernel_version().unwrap_or_default(),
        hostname: System::host_name().unwrap_or_default(),
        uptime_segundos: uptime,
        uptime_dias: uptime / 86400,
    };

    sistema_operativo
}
/// The function `get_cpu_info` retrieves information about the CPU, including details about each core
/// such as name, frequency, usage percentage, and vendor ID.
/// 
/// Returns:
/// 
/// The `get_cpu_info` function is returning a `CpuInfo` struct that contains information about the
/// CPU(s) in the system. The struct includes the total number of CPUs and a vector of `CpuDetalle`
/// structs, each representing detailed information about an individual CPU core.

pub fn get_cpu_info()-> CpuInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut cpus_detalle = Vec::new();
    for (i, cpu) in sys.cpus().iter().enumerate() {
        cpus_detalle.push(CpuDetalle {
            id: i,
            nombre: cpu.name().to_string(),
            frecuencia_mhz: cpu.frequency(),
            uso_porcentaje: cpu.cpu_usage(),
            vendor: cpu.vendor_id().to_string(),
        });
    }

    let cpu_info = CpuInfo {
        total_cpus: sys.cpus().len(),
        cpus: cpus_detalle,
    };

    cpu_info
}
/// This Rust function retrieves memory information such as total RAM, used RAM, free RAM, RAM usage
/// percentage, total swap space, used swap space, and free swap space.
/// 
/// Returns:
/// 
/// The `get_memory_info` function is returning a struct of type `MemoriaInfo` which contains various
/// memory-related information such as total RAM in gigabytes, total RAM in bytes, used RAM in
/// gigabytes, used RAM in bytes, free RAM in gigabytes, free RAM in bytes, RAM usage percentage, total
/// swap space in gigabytes, used swap space in gigabytes, and free swap

pub fn get_memory_info() -> MemoriaInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let memoria_info = MemoriaInfo {
        ram_total_gb: sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        ram_total_bytes: sys.total_memory(),
        ram_usada_gb: sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        ram_usada_bytes: sys.used_memory(),
        ram_libre_gb: sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        ram_libre_bytes: sys.available_memory(),
        ram_uso_porcentaje: (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0,
        swap_total_gb: sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0,
        swap_usada_gb: sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0,
        swap_libre_gb: (sys.total_swap() - sys.used_swap()) as f64 / 1024.0 / 1024.0 / 1024.0,
    };

    memoria_info
}
/// The function `get_disks_info` retrieves information about disks in the system and returns it as a
/// vector of `DiscoInfo` structs.
/// 
/// Returns:
/// 
/// The function `get_disks_info` is returning a vector of `DiscoInfo` structs. Each `DiscoInfo` struct
/// contains information about a disk, such as disk number, name, mount point, file system, type, total
/// space in GB, available space in GB, used space in GB, percentage usage, and whether the disk is
/// removable.

pub fn get_disks_info() -> Vec<DiscoInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();
    let mut discos_info = Vec::new();
    for (i, disk) in disks.iter().enumerate() {
        discos_info.push(DiscoInfo {
            numero: i + 1,
            nombre: disk.name().to_string_lossy().to_string(),
            punto_montaje: disk.mount_point().display().to_string(),
            sistema_archivos: format!("{:?}", disk.file_system()),
            tipo: format!("{:?}", disk.kind()),
            espacio_total_gb: disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0,
            espacio_disponible_gb: disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0,
            espacio_usado_gb: (disk.total_space() - disk.available_space()) as f64 / 1024.0 / 1024.0 / 1024.0,
            uso_porcentaje: ((disk.total_space() - disk.available_space()) as f64 / disk.total_space() as f64) * 100.0,
            removible: disk.is_removable(),
        });
    }

    discos_info
}

/// The function `get_networks_info` retrieves network information such as interface details and local
/// IP address in Rust.
/// 
/// Returns:
/// 
/// The `get_networks_info` function is returning an instance of the `RedesInfo` struct, which contains
/// information about network interfaces and the local IP address.

pub fn get_networks_info() -> RedesInfo {

    let networks = Networks::new_with_refreshed_list();
    let mut redes_info = Vec::new();

    for (interface_name, data) in &networks {
        redes_info.push(InterfazRed {
            nombre: interface_name.to_string(),
            mac: data.mac_address().to_string(),
            recibido_mb: data.total_received() as f64 / 1024.0 / 1024.0,
            recibido_bytes: data.total_received(),
            transmitido_mb: data.total_transmitted() as f64 / 1024.0 / 1024.0,
            transmitido_bytes: data.total_transmitted(),
            paquetes_recibidos: data.total_packets_received(),
            paquetes_transmitidos: data.total_packets_transmitted(),
            errores_recibidos: data.total_errors_on_received(),
            errores_transmitidos: data.total_errors_on_transmitted(),
        });
    }

    //ip local principal
    let mut ip_local = String::from("No disponible");
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                ip_local = addr.ip().to_string();
            }
        }
    }

    let redes_info = RedesInfo {
        interfaces: redes_info,
        ip_local_principal: ip_local,
    };

    redes_info
}
/// The function `get_users_info` retrieves information about users and their groups in Rust.
/// 
/// Returns:
/// 
/// The function `get_users_info` is returning an instance of `UsuariosInfo` struct.

pub fn get_users_info() -> UsuariosInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let users = Users::new_with_refreshed_list();
    let total_usuarios = users.len();

    let mut usuarios_detalle = Vec::new();
    for user in &users {
        let grupos: Vec<String> = user.groups().iter()
            .map(|g| g.name().to_string())
            .collect();
        usuarios_detalle.push(UsuarioDetalle {
            nombre: user.name().to_string(),
            grupos,
        });
    }

    let usuarios_info = UsuariosInfo {
        total: total_usuarios,
        usuarios: usuarios_detalle,
    };

    usuarios_info
}
/// The function `get_processes_info` retrieves information about processes, including the top 10
/// processes by CPU and memory usage.
/// 
/// Returns:
/// 
/// The `get_processes_info` function is returning a struct `ProcesosInfo` which contains information
/// about processes. The struct includes the total number of processes, the top 10 processes by CPU
/// usage, and the top 10 processes by memory usage. Each top process is represented by a
/// `ProcesoDetalle` struct which contains details such as process ID, name, CPU percentage, memory
/// usage in

pub fn get_processes_info() -> ProcesosInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes_vec: Vec<_> = sys.processes().iter().collect();
    
    // Top 10 procesos por uso de CPU
    processes_vec.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());
    let mut top_10_cpu = Vec::new();
    for (i, (pid, process)) in processes_vec.iter().take(10).enumerate() {
        top_10_cpu.push(ProcesoDetalle {
            indice: i,
            pid: pid.as_u32(),
            nombre: process.name().to_string(),
            cpu_porcentaje: process.cpu_usage(),
            memoria_mb: process.memory() as f64 / 1024.0 / 1024.0,
            memoria_virtual_mb: process.virtual_memory() as f64 / 1024.0 / 1024.0,
            disco_lectura_bytes: process.disk_usage().read_bytes,
            disco_escritura_bytes: process.disk_usage().written_bytes,
        });
    }
    
    // Top 10 procesos por uso de memoria
    processes_vec.sort_by(|a, b| b.1.memory().cmp(&a.1.memory()));
    let mut top_10_memoria = Vec::new();
    for (i, (pid, process)) in processes_vec.iter().take(10).enumerate() {
        top_10_memoria.push(ProcesoDetalle {
            indice: i,
            pid: pid.as_u32(),
            nombre: process.name().to_string(),
            cpu_porcentaje: process.cpu_usage(),
            memoria_mb: process.memory() as f64 / 1024.0 / 1024.0,
            memoria_virtual_mb: process.virtual_memory() as f64 / 1024.0 / 1024.0,
            disco_lectura_bytes: process.disk_usage().read_bytes,
            disco_escritura_bytes: process.disk_usage().written_bytes,
        });
    }
    
    let procesos_info = ProcesosInfo {
        total: sys.processes().len(),
        top_10_cpu,
        top_10_memoria,
    };

    procesos_info
}
