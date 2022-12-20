#[tauri::command]
pub fn panic_cmd() {
    panic!()
}