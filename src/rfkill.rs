use std::fs;
use crate::app::AppResult;
pub fn check() -> AppResult<()> {
    if let Ok(entries) = fs::read_dir("/sys/class/rfkill/") {
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            if let Some(_file_name) = entry_path.file_name() {
                let name = fs::read_to_string(entry_path.join("type"))?;
                if name.trim() == "bluetooth" {
                    let state_path = entry_path.join("state");
                    let state = fs::read_to_string(state_path)?.trim().parse::<u8>()?;
                    match state {
                        0 => {
                            eprintln!(
                                r"
Устройство Bluetooth мягко заблокировано
Запустите следующую команду, чтобы разблокировать его
$ sudo rfkill разблокировать Bluetooth
                    "
                            );
                            std::process::exit(1);
                        }
                        2 => {
                            eprintln!("Устройство Bluetooth жестко заблокировано");
                            std::process::exit(1);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}