use sysinfo::{System, SystemExt, CpuExt};
use futures::executor::block_on;

async fn fetch_gpu_info() -> Option<String> {
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let adapter = instance.enumerate_adapters(wgpu::Backends::all()).next()?;
    let info = adapter.get_info();
    Some(format!("Graphics Card: {} - Vendor: {:?}", info.name, info.vendor))
}

#[tauri::command]
async fn get_system_info() -> Result<String, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_info = sys.cpus().iter().map(|cpu| cpu.brand().to_string()).collect::<Vec<_>>().join(", ");
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    // Fetch GPU information
    let gpu_info = block_on(fetch_gpu_info()).unwrap_or_else(|| "Graphics Card: Not available".to_string());

    let info = format!(
        "CPU Info: {}\nTotal Memory: {}\nUsed Memory: {}\n{}",
        cpu_info, total_memory, used_memory, gpu_info
    );

    Ok(info)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
