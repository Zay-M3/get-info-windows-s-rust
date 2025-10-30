
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SystemReport {
    pub sistema_operativo: SistemaOperativo,
    pub cpu: CpuInfo,
    pub memoria: MemoriaInfo,
    pub discos: Vec<DiscoInfo>,
    pub redes: RedesInfo,
    pub usuarios: UsuariosInfo,
    pub procesos: ProcesosInfo,
    pub verificacion_aplicaciones: Vec<AplicacionInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct SistemaOperativo {
    pub os: String,
    pub version: String,
    pub kernel: String,
    pub hostname: String,
    pub uptime_segundos: u64,
    pub uptime_dias: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CpuInfo {
    pub total_cpus: usize,
    pub cpus: Vec<CpuDetalle>,
}

#[derive(Serialize, Deserialize)]
pub struct CpuDetalle {
    pub id: usize,
    pub nombre: String,
    pub frecuencia_mhz: u64,
    pub uso_porcentaje: f32,
    pub vendor: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemoriaInfo {
    pub ram_total_gb: f64,
    pub ram_total_bytes: u64,
    pub ram_usada_gb: f64,
    pub ram_usada_bytes: u64,
    pub ram_libre_gb: f64,
    pub ram_libre_bytes: u64,
    pub ram_uso_porcentaje: f64,
    pub swap_total_gb: f64,
    pub swap_usada_gb: f64,
    pub swap_libre_gb: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoInfo {
    pub numero: usize,
    pub nombre: String,
    pub punto_montaje: String,
    pub sistema_archivos: String,
    pub tipo: String,
    pub espacio_total_gb: u64,
    pub espacio_disponible_gb: u64,
    pub espacio_usado_gb: u64,
    pub uso_porcentaje: u64,
    pub removible: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RedesInfo {
    pub interfaces: Vec<InterfazRed>,
    pub ip_local_principal: String,
}

#[derive(Serialize, Deserialize)]
pub struct InterfazRed {
    pub nombre: String,
    pub mac: String,
    pub recibido_mb: f64,
    pub recibido_bytes: u64,
    pub transmitido_mb: f64,
    pub transmitido_bytes: u64,
    pub paquetes_recibidos: u64,
    pub paquetes_transmitidos: u64,
    pub errores_recibidos: u64,
    pub errores_transmitidos: u64,
}

#[derive(Serialize, Deserialize)]
pub struct UsuariosInfo {
    pub total: usize,
    pub usuarios: Vec<UsuarioDetalle>,
}

#[derive(Serialize, Deserialize)]
pub struct UsuarioDetalle {
    pub nombre: String,
    pub grupos: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProcesosInfo {
    pub total: usize,
    pub top_10_cpu: Vec<ProcesoDetalle>,
    pub top_10_memoria: Vec<ProcesoDetalle>,
}

#[derive(Serialize, Deserialize)]
pub struct ProcesoDetalle {
    pub indice: usize,
    pub pid: u32,
    pub nombre: String,
    pub cpu_porcentaje: f32,
    pub memoria_mb: f64,
    pub memoria_virtual_mb: f64,
    pub disco_lectura_bytes: u64,
    pub disco_escritura_bytes: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AplicacionInfo {
    pub ruta: String,
    pub existe: bool,
    pub elementos: Option<usize>,
}

#[derive(Serialize, Debug)]
pub struct Info {
    pub id: u64,
    pub name: String,
    pub active: bool,
    pub winput: String,
}
