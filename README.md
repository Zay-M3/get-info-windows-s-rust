# Extractor de Información del Sistema con Rust

Este es un ejecutable en Rust que saca toda la información relevante del sistema operativo en formato JSON. Nada de interfaces gráficas ni complicaciones, solo un .exe que corre, escupe el JSON y listo.

## Requisitos

- Windows 
- Rust instalado 
- Visual Studio Build Tools 

## Inicio Rápido

```powershell
# Compilar
cargo build --release

# Ejecutar
.\target\release\test-rust.exe

# O si estás en desarrollo
cargo run
```

## Qué Información Extrae

El ejecutable saca todo esto y lo devuelve en JSON:

- **Sistema Operativo**: Nombre, versión, kernel, hostname, uptime
- **CPU**: Todos los núcleos con frecuencia, uso y vendor
- **Memoria**: RAM total, usada, libre, SWAP (todo en GB y bytes)
- **Discos**: Todos los discos con espacio, uso, sistema de archivos, tipo
- **Redes**: Interfaces, MAC, tráfico, paquetes, errores, IP local
- **Usuarios**: Lista completa con sus grupos
- **Procesos**: Top 10 por CPU y top 10 por memoria con detalles
- **Aplicaciones**: Verificación de rutas personalizadas (ej: C:\SiesaFiscal)

## Uso Típico

### Guardar en archivo

```powershell
.\test-rust.exe > info-sistema.json
```

### Procesarlo con PowerShell

```powershell
$info = .\test-rust.exe | ConvertFrom-Json
$info.sistema_operativo.hostname
$info.cpu.total_cpus
```

### Integrarlo con Python

```python
import subprocess
import json

result = subprocess.run(['test-rust.exe'], capture_output=True, text=True)
data = json.loads(result.stdout)

print(f"Hostname: {data['sistema_operativo']['hostname']}")
print(f"RAM usada: {data['memoria']['ram_usada_gb']:.2f} GB")
```

### Monitoreo automatizado

```powershell
# Ejecutar cada hora y guardar con timestamp
while ($true) {
    $fecha = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
    .\test-rust.exe > "logs\sistema_$fecha.json"
    Start-Sleep -Seconds 3600
}
```

## Compilación

### Opción 1: Compilar localmente

```powershell
# Debug (más rápido de compilar)
cargo build

# Release (optimizado)
cargo build --release
```

### Opción 2: Descargar el .exe

Si no quieres compilar, descarga el ejecutable desde releases (cuando esté disponible).

## Problemas Comunes

### Error de linker (link.exe)

Si ves `error: linking with link.exe failed: exit code: 1181`:

1. Instala Visual Studio Build Tools
2. Marca la opción "C++ build tools" durante la instalación
3. Reinicia la terminal

### El JSON sale cortado en la consola

Redirige la salida a un archivo:

```powershell
.\test-rust.exe > salida.json
```

### No detecta algunas interfaces de red

Es normal. Solo muestra las interfaces activas que el sistema reporta.

## Personalización

Para agregar rutas personalizadas a verificar, edita `main.rs`:

```rust
let paths_to_check = vec![
    "C:\\TuApp",
    "D:\\OtraCarpeta",
];
```

Luego recompila.

## Dependencias

El proyecto usa solo:

- `sysinfo`: Para extraer info del sistema
- `serde` y `serde_json`: Para serializar a JSON

## Performance

El ejecutable es ligero:

- Tarda ~1-2 segundos en ejecutar
- Usa ~20MB de RAM mientras corre
- El JSON resultante pesa ~5-15KB dependiendo de cuántos procesos tengas

## Estructura del Proyecto

```
test-rust/
├── src/
│   ├── main.rs              # Lógica principal
│   └── utils/
│       └── interfase.rs     # Definiciones de estructuras
├── Cargo.toml               # Dependencias
└── target/
    └── release/
        └── test-rust.exe    # Ejecutable compilado
```

## Para Qué Sirve Esto

- Monitoreo de servidores sin instalar agentes pesados
- Auditorías rápidas del sistema
- Integración con scripts de automatización
- Logs históricos del estado del servidor
- Detección de cambios en el sistema
- Base para dashboards personalizados

## Aportes

Si encuentras bugs o quieres agregar más información al JSON, los PRs son bienvenidos. No soy el mejor programador de Rust del mundo, solo quería que funcionara y que fuera útil.

## Disclaimer

Esto no está pensado para producción crítica ni para reemplazar herramientas enterprise de monitoreo. Es para tener información rápida del sistema sin complicarse.