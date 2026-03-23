use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Appearance {
    #[serde(default)]
    pub solid_background_color: String,
    #[serde(default)]
    pub global_theme: String,
    #[serde(default)]
    pub default_weather: String,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            solid_background_color: "".to_string(),
            global_theme: "default".to_string(),
            default_weather: "clear".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct MonolithConfig {
    pub custom_text: String,
    pub custom_color: String,
    pub override_distro: String,
}

impl Default for MonolithConfig {
    fn default() -> Self {
        Self {
            custom_text: "".to_string(),
            custom_color: "".to_string(),
            override_distro: "".to_string(),
        }
    }
}



#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct SimulationConfig {
    pub max_vehicles: usize,
    pub max_pedestrians: usize,
    pub vehicle_speed_multiplier: f32,
    pub weather_speed_multiplier: f32,
    pub weather_density_multiplier: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            max_vehicles: 100,
            max_pedestrians: 15,
            vehicle_speed_multiplier: 1.0,
            weather_speed_multiplier: 1.0,
            weather_density_multiplier: 1.0,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub appearance: Appearance,
    #[serde(default)]
    pub monolith: MonolithConfig,
    #[serde(default)]
    pub simulation: SimulationConfig,
}

impl Config {
    pub fn load() -> Self {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "Metropolis") {
            let path = proj_dirs.config_dir();
            let mut config_file = path.to_path_buf();
            config_file.push("config.toml");

            let mut themes_dir = path.to_path_buf();
            themes_dir.push("themes");

            if !themes_dir.exists() {
                let _ = std::fs::create_dir_all(&themes_dir);
            }

            let mut template_file = themes_dir.clone();
            template_file.push("template.toml");
            if !template_file.exists() {
                let template_content = include_str!("../assets/template.toml");
                let _ = std::fs::write(&template_file, template_content);
            }

            if config_file.exists() {
                if let Ok(content) = std::fs::read_to_string(&config_file) {
                    match toml::from_str(&content) {
                        Ok(config) => return config,
                        Err(e) => {
                            let mut err_file = path.to_path_buf();
                            err_file.push("error.log");
                            let err_msg = format!("Error parsing config.toml: {}\n", e);
                            let _ = std::fs::write(&err_file, &err_msg);
                            eprintln!("{}", err_msg);
                            std::thread::sleep(std::time::Duration::from_secs(2));
                        }
                    }
                }
            } else {
                if !path.exists() {
                    let _ = std::fs::create_dir_all(&path);
                }

                let config_content = include_str!("../assets/config_template.toml");
                let _ = fs::write(config_file, config_content);
            }
        }
        Config::default()
    }
}
