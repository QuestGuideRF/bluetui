use crate::app::AppResult;
use anyhow::Context;
use bluer::Address;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};
pub fn read_favorite_devices_from_disk() -> AppResult<Vec<Address>> {
    let data_dir = dirs::data_dir()
        .context("не удалось найти data_dir")?
        .join("bluetui");
    let file =
        File::open(data_dir.join("favorites.txt")).context("невозможно открыть файл избранного")?;
    let lines = BufReader::new(file).lines();
    lines
        .map(|line| {
            let line = line?;
            let addr = Address::from_str(&line)?;
            Ok(addr)
        })
        .collect()
}
pub fn save_favorite_devices_to_disk(favorite_devices: &[Address]) -> AppResult<()> {
    let data_dir = dirs::data_dir()
        .context("не удалось найти data_dir")?
        .join("bluetui");
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)
            .context("невозможно создать родительский каталог(ы) для файла избранного")?;
    }
    let file_path = data_dir.join("favorites.txt");
    let mut contents =
        String::with_capacity(favorite_devices.len() * (Address::any().to_string() + "\n").len());
    for device in favorite_devices {
        contents.push_str(&device.to_string());
        contents.push('\n');
    }
    std::fs::write(file_path, contents).context("ошибка при записи файла избранного")?;
    Ok(())
}