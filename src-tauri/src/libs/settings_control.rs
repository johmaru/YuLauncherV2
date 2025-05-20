use std::{io::Write, path::Path};
use serde::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize)]
struct Resolution {
    width: u32,
    height: u32,
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution {
            width: 1024,
            height: 768,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            language: "ja".to_string(),
            theme: "dark".to_string(),
            resolution: Resolution::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Settings {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub resolution: Resolution,
}

fn create_settings_file() -> Result<(), Box<dyn std::error::Error>> {
    let default_settings = Settings::default();
    let toml_string = toml::to_string(&default_settings)?;
    let path = settings_path()?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let mut file = std::fs::File::create(&path)?;
    file.write_all(toml_string.as_bytes())?;
    println!("設定ファイルが作成されました。{}", &path.display());
    Ok(())
}

fn settings_path() -> std::io::Result<std::path::PathBuf> {
    let mut exe_path = std::env::current_exe()?;
    exe_path.pop();
    Ok(exe_path.join("settings.toml"))
}

pub fn check_settings_file() -> Result<(), Box<dyn std::error::Error>> {
    let settings_path = settings_path()?;
    if !Path::new(&settings_path).exists() {
        println!("設定ファイルが見つかりませんでした、新しいファイルを作成します...");
        create_settings_file()?;
    } else {
        println!("ファイルが存在します。 設定ファイル場所: {}", &settings_path.display());
    }
    Ok(())
}