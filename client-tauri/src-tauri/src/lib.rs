#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Lancer le serveur Next.js uniquement en production
    #[cfg(not(debug_assertions))]
    {
        let server_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("client/.next/standalone/server.js");

        std::process::Command::new("node")
            .arg(&server_path)
            .env("PORT", "3000")
            .env("HOSTNAME", "127.0.0.1")
            .spawn()
            .expect("Failed to start Next.js server");

        // Attendre que le serveur soit prêt
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
