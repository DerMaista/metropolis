use crate::logos;

use rand::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::Widget,
};

#[derive(Debug, Clone, PartialEq)]
pub enum VehicleType {
    Spinner,
    Shuttle,
    Police,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Weather {
    Clear,
    Rain,
    Snow,
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub color: Color,
    pub v_type: VehicleType,
    pub length: u16,
}

#[derive(Debug, Clone)]
pub struct Raindrop {
    pub x: f32,
    pub y: f32,
    pub speed_y: f32,
    pub speed_x: f32,
    pub z_index: u8,
}

#[derive(Debug, Clone)]
pub struct Splash {
    pub x: u16,
    pub y: u16,
    pub frame: u8,
}

#[derive(Debug, Clone)]
pub struct Person {
    pub x: f32,
    pub speed: f32,
    pub color: Color,
    pub id_offset: u64,
    pub is_entering: bool,
    pub entry_pause_timer: u8,
}

pub struct MetropolisCity {
    pub vehicles: Vec<Vehicle>,
    pub raindrops: Vec<Raindrop>,
    pub splashes: Vec<Splash>,
    pub people: Vec<Person>,
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub cpu_smoothed: f32,
    pub ram_smoothed: f32,
    pub frame_count: u64,
    pub window_seed: u64,
    pub chase_cooldown: u32,
    pub distro: String,
    pub weather: Weather,
    pub debug_mode: bool,
    pub top_processes: Vec<String>,
    pub disk_usage: f32,
}

impl MetropolisCity {
    pub fn new(distro: String, weather: Weather) -> Self {
        Self {
            vehicles: Vec::with_capacity(100),
            raindrops: Vec::with_capacity(250),
            splashes: Vec::with_capacity(50),
            people: Vec::with_capacity(30),
            cpu_usage: 0.0,
            ram_usage: 0.0,
            cpu_smoothed: 0.0,
            ram_smoothed: 0.0,
            frame_count: 0,
            window_seed: thread_rng().gen(),
            chase_cooldown: 0,
            distro,
            weather,
            debug_mode: false,
            top_processes: Vec::new(),
            disk_usage: 0.0,
        }
    }

    fn get_sky_lane(rng: &mut impl Rng) -> f32 {
        let lanes = vec![5.0, 8.0, 11.0, 14.0];
        lanes[rng.gen_range(0..lanes.len())]
    }

    pub fn update(&mut self, area: Rect, cpu: f32, ram: f32, disk: f32, processes: Vec<String>) {
        if area.width == 0 || area.height == 0 { return; }
        self.cpu_usage = cpu;
        self.ram_usage = ram;
        self.disk_usage = disk;
        self.top_processes = processes;
        self.cpu_smoothed = self.cpu_smoothed + (cpu - self.cpu_smoothed) * 0.05;
        self.ram_smoothed = self.ram_smoothed + (ram - self.ram_smoothed) * 0.05;
        self.frame_count = self.frame_count.wrapping_add(1);
        let mut rng = thread_rng();

        let pulse = (self.frame_count as f32 * 0.003).sin() * 0.5 + 0.5; // Density wave
        let base_targets = (cpu / 4.0).max(5.0);
        let target_vehicles = (base_targets + (base_targets * pulse)) as usize;
        
        if self.chase_cooldown > 0 { self.chase_cooldown -= 1; }

        if self.vehicles.len() < target_vehicles && self.frame_count % 3 == 0 {
            let y = Self::get_sky_lane(&mut rng);
            let roll = rng.gen_range(0.0..1.0);
            
            if roll < 0.02 && self.chase_cooldown == 0 { 
                let speed = rng.gen_range(4.5..6.5);
                self.vehicles.push(Vehicle { x: -5.0, y, speed, color: Color::Rgb(255, 85, 85), v_type: VehicleType::Spinner, length: 3 });
                self.vehicles.push(Vehicle { x: -15.0, y, speed, color: Color::White, v_type: VehicleType::Police, length: 3 });
                self.vehicles.push(Vehicle { x: -25.0, y, speed, color: Color::White, v_type: VehicleType::Police, length: 3 });
                self.vehicles.push(Vehicle { x: -35.0, y, speed, color: Color::White, v_type: VehicleType::Police, length: 3 });
                self.chase_cooldown = 1200;
            } else if roll < 0.10 {
                let length: u16 = rng.gen_range(4..12);
                
                let mega_count = self.vehicles.iter().filter(|v| v.v_type == VehicleType::Shuttle && v.length > 9).count();
                
                let disk_bonus = (self.disk_usage / 10.0) as u16; 
                let adj_length = length.saturating_add(disk_bonus).min(25);

                if adj_length <= 9 || mega_count == 0 {
                    let color = if adj_length <= 6 {
                        Color::Rgb(255, 255, 85)
                    } else if adj_length <= 9 {
                        Color::Rgb(170, 170, 170)
                    } else {
                        Color::Rgb(85, 85, 85)
                    };
                    self.vehicles.push(Vehicle { x: -5.0, y, speed: rng.gen_range(0.3..0.6), color, v_type: VehicleType::Shuttle, length: adj_length });
                }
            } else {
                let traffic_colors = vec![
                    Color::Rgb(85, 255, 255),
                    Color::Rgb(85, 255, 85),
                    Color::Rgb(255, 85, 255),
                    Color::Rgb(255, 255, 85),
                ];
                let color = traffic_colors[rng.gen_range(0..traffic_colors.len())];
                self.vehicles.push(Vehicle { x: -5.0, y, speed: rng.gen_range(0.8..2.2), color, v_type: VehicleType::Spinner, length: 3 });
            }
        }
        let speed_mod = 0.5 + (cpu / 80.0);
        self.vehicles.retain_mut(|v| { v.x += v.speed * speed_mod; v.x < (area.width as f32 + 40.0) });

        if self.people.len() < 15 && self.frame_count % 15 == 0 {
            let dir = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            let start_x = if dir > 0.0 { -2.0 } else { area.width as f32 + 2.0 };
            self.people.push(Person { x: start_x, speed: rng.gen_range(0.1..0.3) * dir, color: Color::Rgb(170, 170, 170), id_offset: rng.gen_range(0..100), is_entering: false, entry_pause_timer: 0 });
        }
        self.people.retain_mut(|p| {
            if !p.is_entering {
                p.x += p.speed;
                let mut is_main = false; let mut near_door = false;
                for (idx, xb) in (0..area.width).step_by(20).enumerate() {
                    let mut bw = 8; if idx == 1 { bw = 32; }
                    let dx = xb + (bw / 2);
                    if (p.x as u16).abs_diff(dx) < 1 { near_door = true; if idx == 1 { is_main = true; } break; }
                }
                if near_door && rng.gen_bool(0.02) {
                    let chance = if is_main { 0.4 } else { 0.1 };
                    if rng.gen_bool(chance) { p.is_entering = true; p.entry_pause_timer = 60; }
                }
            } else {
                if p.entry_pause_timer > 0 { p.entry_pause_timer -= 1; }
                else { return false; }
            }
            p.x > -5.0 && p.x < (area.width as f32 + 5.0)
        });

        if self.weather == Weather::Rain {
            if self.raindrops.len() < 100 {
                let z = if rng.gen_bool(0.3) { 1 } else { 0 };
                self.raindrops.push(Raindrop { x: rng.gen_range(0..area.width as i32 + 40) as f32, y: -2.0, speed_y: if z == 1 { rng.gen_range(1.2..1.8) } else { rng.gen_range(0.6..1.0) }, speed_x: -0.4, z_index: z });
            }
            self.raindrops.retain_mut(|r| { 
                r.y += r.speed_y; 
                r.x += r.speed_x; 
                let ground_y = area.height as f32 - 3.0;
                if r.y >= ground_y && rng.gen_bool(0.08) { 
                    self.splashes.push(Splash { x: r.x as u16, y: ground_y as u16, frame: 0 }); 
                    return false; 
                } 
                r.y < area.height as f32 && r.x > 0.0 
            });
            self.splashes.retain_mut(|s| { s.frame += 1; s.frame < 4 });
        } else if self.weather == Weather::Snow {
            if self.raindrops.len() < 150 {
                let z = if rng.gen_bool(0.3) { 1 } else { 0 };
                self.raindrops.push(Raindrop { 
                    x: rng.gen_range(0..area.width as i32 + 40) as f32, 
                    y: -2.0, 
                    speed_y: if z == 1 { rng.gen_range(0.3..0.6) } else { rng.gen_range(0.15..0.35) }, 
                    speed_x: rng.gen_range(-0.3..0.1), 
                    z_index: z 
                });
            }
            self.raindrops.retain_mut(|r| { 
                r.y += r.speed_y; 
                r.x += r.speed_x + ((self.frame_count as f32 * 0.05).sin() * 0.1); 
                r.y < area.height as f32 && r.x > 0.0 && r.x < area.width as f32 + 20.0
            });
            self.splashes.clear();
        } else {
            self.raindrops.clear();
            self.splashes.clear();
        }
    }
}

impl Widget for &MetropolisCity {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 10 || area.height < 10 { return; }

        let mut star_rng = StdRng::seed_from_u64(42); 
        for i in 0..25 {
            let x = star_rng.gen_range(0..area.width);
            let y = star_rng.gen_range(0..area.height / 2);
            let mut p_rng = StdRng::seed_from_u64(i as u64);
            let star_type = p_rng.gen_range(0..4);
            let (symbol, dim_color) = match star_type {
                0 => ('.', Color::Rgb(60, 60, 80)),
                1 => ('·', Color::Rgb(50, 50, 70)),
                2 => ('*', Color::Rgb(70, 70, 90)),
                _ => ('+', Color::Rgb(40, 40, 60)),
            };
            let pulse = ((self.frame_count as f32 * 0.1 + i as f32).sin() + 1.0) / 2.0;
            let color = if pulse > 0.85 {
                match star_type {
                    0 => Color::Rgb(200, 200, 255),
                    1 => Color::Cyan,
                    2 => Color::Rgb(255, 150, 255),
                    _ => Color::White,
                }
            } else if pulse > 0.5 { dim_color } else { Color::Rgb(30, 30, 45) };
            safe_set_char(buf, area.x + x, area.y + y, symbol, color);
        }

        let logo_asset = logos::get_logo(&self.distro);
        let mut skip_next = false;
        let b_base_color = Color::Rgb(20, 20, 35);
        let ground_y = area.height.saturating_sub(3);

        let mut bg_rng = StdRng::seed_from_u64(12345);
        let bg_color = Color::Rgb(12, 12, 22);
        for x_bg in (0..area.width).step_by(15) {
            let bw = bg_rng.gen_range(6..15) as u16;
            let bh = bg_rng.gen_range(area.height / 5..area.height / 2) as u16;
            let start_x = area.x.saturating_add(x_bg);
            let start_y = ground_y.saturating_sub(bh);

            for y_rel in 0..bh {
                for x_rel in 0..bw {
                    let dx = start_x.saturating_add(x_rel);
                    let dy = start_y.saturating_add(y_rel);
                    if dx < area.x + area.width && dy < area.y + area.height {
                        buf.get_mut(dx, dy).set_symbol(" ").set_bg(bg_color);
                    }
                }
            }
        }

        for (i, x_base) in (0..area.width).step_by(20).enumerate() {
            if skip_next { skip_next = false; continue; }
            let mut bw = 8 + (x_base % 7) as u16;
            let mut bh = (area.height / 3) + (x_base % 11) as u16;
            if i == 1 { bw = 32; bh = area.height.saturating_sub(8); skip_next = true; }
            if i == 3 { bw = 28; skip_next = true; }
            let ground_y = area.height.saturating_sub(3);
            let start_y = ground_y.saturating_sub(bh);
            let start_x = area.x.saturating_add(x_base);

            for y_rel in 0..bh {
                for x_rel in 0..bw {
                    let dx = start_x.saturating_add(x_rel);
                    let dy = start_y.saturating_add(y_rel);
                    if dx < area.x + area.width && dy < area.y + area.height {
                        let mut symbol = " ";
                        let mut fg = b_base_color;
                        let mut bg = b_base_color;

                        let mut is_logo_pixel = false;
                        if i == 1 && y_rel < 20 && x_rel < 32 {
                            if let Some(pixel) = &logo_asset.grid[y_rel as usize][x_rel as usize] {
                                let cell = buf.get_mut(dx, dy);
                                cell.set_char(pixel.ch);
                                cell.set_fg(pixel.color);
                                cell.set_bg(pixel.bg);
                                is_logo_pixel = true;
                            } else if self.distro.to_lowercase().contains("windows") && x_rel >= 6 && x_rel <= 26 && y_rel >= 3 && y_rel <= 14 {
                                bg = b_base_color;
                                is_logo_pixel = true;
                            } else if !logo_asset.is_compact && x_rel > 4 && x_rel < 28 {
                                bg = b_base_color;
                                is_logo_pixel = true;
                            }
                            
                            if is_logo_pixel && logo_asset.grid[y_rel as usize][x_rel as usize].is_some() {
                                continue;
                            }
                        }

                        if !is_logo_pixel {
                            if x_rel == 0 || x_rel == bw - 1 { 
                                symbol = "┃"; 
                                fg = Color::Rgb(30, 30, 50); 
                            }

                            let has_sign = i % 2 == 1 && bh > 12;
                            let is_win_row = y_rel > 2 && y_rel < bh.saturating_sub(4) && y_rel % 3 == 0;
                            let x_clearance = if has_sign { bw.saturating_sub(2) } else { bw - 1 };

                            let mut near_logo = false;
                            if i == 1 {
                                for dy_off in -1..=1 {
                                    for dx_off in -1..=1 {
                                        let check_y = (y_rel as i32 + dy_off) as usize;
                                        let check_x = (x_rel as i32 + dx_off) as usize;
                                        if check_y < 20 && check_x < 32 {
                                            if logo_asset.grid[check_y][check_x].is_some() {
                                                near_logo = true; break;
                                            }
                                        }
                                    }
                                    if near_logo { break; }
                                }
                            }

                            if !near_logo && is_win_row && x_rel > 0 && x_rel < x_clearance && (dx.wrapping_add(dy as u16)) % 4 == 0 {
                                let door_x = bw / 2;
                                if !(y_rel >= bh - 3 && x_rel >= door_x - 1 && x_rel <= door_x + 1) {
                                    symbol = "▄";
                                    let seed = (dx as u64).wrapping_mul(100).wrapping_add(dy as u64).wrapping_add(self.window_seed);
                                    let mut wr = StdRng::seed_from_u64(seed);
                                    fg = if wr.gen_bool(0.25) { Color::Rgb(255, 255, 85) } else { Color::Rgb(85, 85, 85) };
                                    bg = Color::Black;
                                }
                            }
                            
                            if y_rel >= bh - 3 {
                                let door_x = bw / 2;
                                if x_rel >= door_x - 1 && x_rel <= door_x + 1 {
                                    if y_rel == bh - 3 { 
                                        symbol = "━"; 
                                        fg = if i % 2 == 0 { Color::Cyan } else { Color::Magenta }; 
                                    } else { 
                                        symbol = "░"; 
                                        fg = Color::Rgb(30, 40, 60); 
                                    }
                                }
                                if x_rel == door_x + 2 && y_rel == bh - 2 {
                                    symbol = "·"; 
                                    fg = if self.frame_count % 20 < 10 { Color::Red } else { Color::Green };
                                }
                            }
                        }

                        buf.get_mut(dx, dy).set_symbol(symbol).set_fg(fg).set_bg(bg);
                    }
                }
            }

            if i % 2 == 1 && bh > 12 {
                let sign_text;
                let sign_color;
                if i == 1 {
                    let display_name = match self.distro.to_lowercase().as_str() {
                        "popos" | "pop_os" => "POP! OS".to_string(),
                        "endeavouros" => "ENDEAVOUR OS".to_string(),
                        "artix" => "ARTIX LINUX".to_string(),
                        "garuda" => "GARUDA LINUX".to_string(),
                        "zorin" => "ZORIN OS".to_string(),
                        "rhel" | "redhat" => "RED HAT".to_string(),
                        _ => self.distro.to_uppercase(),
                    };
                    sign_text = format!("{} CORP", display_name);
                    sign_color = Color::Rgb(85, 255, 255);
                } else {
                    let p_idx = (i / 2).saturating_sub(1) % self.top_processes.len().max(1);
                    sign_text = if self.top_processes.is_empty() {
                        "NULL".to_string()
                    } else {
                        self.top_processes[p_idx % self.top_processes.len()].clone()
                    };
                    
                    sign_color = match (i / 2) % 3 {
                        0 => Color::Rgb(255, 85, 255),
                        1 => Color::Rgb(85, 255, 85),
                        _ => Color::Rgb(255, 255, 85),
                    };
                }

                let sign_y = start_y.saturating_add(5);
                draw_neon_sign(buf, start_x + bw - 1, sign_y, &sign_text, sign_color, self.frame_count);
            }

            if i != 3 {
                let ant_x = start_x.saturating_add(2);
                if ant_x < buf.area.width {
                    let ant_y = area.y.saturating_add(start_y.saturating_sub(1));
                    if ant_y < buf.area.height {
                        match i % 3 {
                            0 => {
                                safe_set_symbol(buf, ant_x, ant_y, "┷", Color::Rgb(60, 60, 80));
                                if ant_y > area.y {
                                    safe_set_symbol(buf, ant_x, ant_y - 1, "┃", Color::Rgb(50, 50, 70));
                                    let beacon_color = if self.frame_count % 30 < 15 { Color::Red } else { Color::Rgb(60, 0, 0) };
                                    if ant_y > area.y + 1 {
                                        safe_set_symbol(buf, ant_x, ant_y - 2, "*", beacon_color);
                                    }
                                }
                            }
                            1 => {
                                safe_set_symbol(buf, ant_x, ant_y, "📡", Color::Rgb(100, 100, 120));
                            }
                            _ => {
                                safe_set_symbol(buf, ant_x, ant_y, "▝▘", Color::Rgb(40, 40, 50));
                            }
                        }
                    }
                }
            }
        }

        for x_lamp in (5..area.width).step_by(10) {
            let mut inside = false; let mut s_skip = false;
            for (idx, xb) in (0..area.width).step_by(20).enumerate() {
                if s_skip { s_skip = false; continue; }
                let mut bw = 8 + (xb % 7) as u16;
                if idx == 1 { bw = 32; s_skip = true; }
                if idx == 3 { bw = 28; s_skip = true; }
                if x_lamp >= xb && x_lamp < xb + bw { inside = true; break; }
            }
            if !inside {
                let lx = area.x + x_lamp; 
                let ground_y = area.y + area.height - 3;
                let bulb_c = if (self.frame_count + lx as u64) % 40 < 2 { Color::Rgb(100, 100, 50) } else { Color::Rgb(255, 255, 150) };
                safe_set_symbol(buf, lx, ground_y, "┃", Color::Rgb(40, 40, 50));
                safe_set_symbol(buf, lx, ground_y.saturating_sub(1), "┃", Color::Rgb(40, 40, 50));
                safe_set_symbol(buf, lx, ground_y.saturating_sub(2), "┃", Color::Rgb(40, 40, 50));
                safe_set_string(buf, lx.saturating_sub(1), ground_y.saturating_sub(3), "(O)", bulb_c);
            }
        }

        if self.weather == Weather::Rain {
            for r in &self.raindrops {
                let rx = area.x + r.x as u16; let ry = area.y + r.y as u16;
                let sym = if r.z_index == 1 { "|" } else { ":" };
                let color = if r.z_index == 1 { Color::Rgb(0, 180, 180) } else { Color::Rgb(0, 60, 60) };
                safe_set_symbol(buf, rx, ry, sym, color);
            }
        } else if self.weather == Weather::Snow {
            for r in &self.raindrops {
                let rx = area.x + r.x as u16; let ry = area.y + r.y as u16;
                let sym = match (self.frame_count + (r.x as u64)) % 30 {
                    0..=10 => "*",
                    11..=20 => "·",
                    _ => "❄",
                };
                let color = if r.z_index == 1 { Color::White } else { Color::Rgb(120, 120, 140) };
                safe_set_symbol(buf, rx, ry, sym, color);
            }

            let ground_y = area.y + area.height - 3;
            for rx in 0..area.width {
                let dx = area.x + rx;
                let sym = if (dx as u64 + self.frame_count / 100) % 7 == 0 { "▆" } else { "█" };
                safe_set_symbol(buf, dx, ground_y + 1, sym, Color::White);
                safe_set_symbol(buf, dx, ground_y + 2, "█", Color::White);
            }
        }

        let mb_x = area.x + 60;
        let mut mb_skip_next = false;
        let mut mb_tower_h = 0;
        for (idx, xb) in (0..area.width).step_by(20).enumerate() {
            if mb_skip_next { mb_skip_next = false; continue; }
            if idx == 1 { mb_skip_next = true; }
            if idx == 3 { 
                mb_tower_h = (area.height / 3) + (xb % 11) as u16; 
                break; 
            }
        }
        if mb_tower_h > 0 {
            let mb_y = ground_y.saturating_sub(mb_tower_h);
            draw_roof_megaboard(buf, mb_x + 1, mb_y, self.cpu_smoothed, self.ram_smoothed, self.frame_count);
        }

        for p in &self.people {
            if p.x < 0.0 { continue; }
            let ground_y = area.y + area.height - 3;
            let px = area.x + p.x as u16; 
            let py_l = ground_y; 
            let py_h = py_l.saturating_sub(1);
            if px < area.x + area.width {
                let mut building_bg = None; let mut s_skip = false;
                for (idx, xb) in (0..area.width).step_by(20).enumerate() {
                    if s_skip { s_skip = false; continue; }
                    let mut bw = 8 + (xb % 7) as u16;
                    if idx == 1 { bw = 32; s_skip = true; }
                    if idx == 3 { bw = 28; s_skip = true; }
                    let tower_h = if idx == 1 { area.height - 8 } else if idx == 3 { area.height - 6 } else { (area.height / 3) + (xb % 11) as u16 };
                    if px >= xb && px < xb + bw && py_l >= ground_y.saturating_sub(tower_h) { building_bg = Some(b_base_color); break; }
                }
                let gait = if p.is_entering && p.entry_pause_timer > 0 { 1 } else { ((self.frame_count + p.id_offset) / 4) % 3 };
                let leg_char = match gait { 0 => 'Λ', 1 => '|', _ => 'λ' };
                safe_set_char_with_bg(buf, px, py_h, 'o', p.color, building_bg.unwrap_or(Color::Reset));
                safe_set_char_with_bg(buf, px, py_l, leg_char, p.color, building_bg.unwrap_or(Color::Reset));
            }
        }

        for v in &self.vehicles {
            if v.x < -15.0 { continue; }
            let vx_f = area.x as f32 + v.x; let vy = area.y as u16 + v.y as u16;
            
            let (body, tail_color) = match v.v_type { 
                VehicleType::Spinner => (vec!['◢', '■', '◣'], Some(Color::Rgb(255, 85, 85))), // EGA Light Red
                VehicleType::Shuttle => {
                    let mut b = Vec::new();
                    b.push('▓');
                    for _ in 0..v.length.saturating_sub(2) { b.push('█'); }
                    b.push('▶');
                    (b, Some(Color::Rgb(255, 170, 0)))
                },
                VehicleType::Police => (vec!['◤', '█', '◥'], None),
            };

            for (off, ch) in body.iter().enumerate() {
                let dx = (vx_f + off as f32) as u16;
                if dx >= area.x + area.width || vy >= area.y + area.height { continue; }
                let mut cell_bg = Color::Reset;
                if dx < area.x + area.width && vy < area.y + area.height {
                    cell_bg = buf.get(dx, vy).bg;
                }
                
                let final_fg = if v.v_type == VehicleType::Police { 
                    match off {
                        0 => Color::White,
                        1 => Color::Rgb(60, 60, 70),
                        _ => Color::White,
                    }
                } else { v.color };

                safe_set_char_with_bg(buf, dx, vy, *ch, final_fg, cell_bg);
            }

            if v.v_type == VehicleType::Police && vy > area.y {
                let sy = vy - 1; let flash = (self.frame_count / 2) % 2 == 0;
                for (sx_f, base_color, is_on) in vec![(vx_f, Color::Rgb(85, 85, 255), flash), (vx_f + 2.0, Color::Rgb(255, 85, 85), !flash)] {
                    let sx = sx_f as u16; if sx >= area.x + area.width { continue; }
                    let mut s_bg = Color::Reset;
                    if sx < area.x + area.width && sy < area.y + area.height {
                        s_bg = buf.get(sx, sy).bg;
                    }
                    safe_set_char_with_bg(buf, sx, sy, '═', if is_on { base_color } else { Color::Rgb(40, 40, 60) }, s_bg);
                }
            }

            if let Some(t_color) = tail_color {
                let tx_f = v.x - 1.0;
                if tx_f >= 0.0 {
                    let tx = (area.x as f32 + tx_f) as u16;
                    if tx < area.x + area.width {
                        let mut t_bg = Color::Reset;
                        if tx < area.x + area.width && vy < area.y + area.height {
                            t_bg = buf.get(tx, vy).bg;
                        }

                        if v.v_type == VehicleType::Shuttle {
                            safe_set_char_with_bg(buf, tx, vy, ':', t_color, t_bg);
                            if tx >= area.x + 1 {
                                safe_set_char_with_bg(buf, tx - 1, vy, '·', t_color, t_bg);
                            }
                        } else {
                            safe_set_char_with_bg(buf, tx, vy, '·', t_color, t_bg);
                            if v.v_type == VehicleType::Spinner {
                                let t2x_f = v.x - 2.0;
                                if t2x_f >= 0.0 {
                                    let t2x = (area.x as f32 + t2x_f) as u16;
                                    if t2x < area.x + area.width { safe_set_char_with_bg(buf, t2x, vy, '·', Color::Rgb(85, 255, 255), t_bg); }
                                }
                            }
                        }
                    }
                }
            }
        }

        if self.weather == Weather::Rain {
            let ground_y = area.y + area.height - 3;
            for ry in (ground_y + 1)..(area.y + area.height) {
                let dist = ry - ground_y;
                let sy = ground_y.saturating_sub(dist);
                let ripple = ((self.frame_count as f32 * 0.2 + ry as f32 * 0.5).sin() * 1.2) as i16;
                
                for rx in 0..area.width {
                    let target_x = area.x + rx;
                    let source_x = (area.x as i16 + rx as i16 + ripple).max(area.x as i16).min((area.x + area.width - 1) as i16) as u16;
                    
                    let source_cell = buf.get(source_x, sy).clone();
                    if source_cell.symbol() != " " || source_cell.fg != Color::Reset {
                        let target_cell = buf.get_mut(target_x, ry);
                        
                        let dim_fg = darken_color(source_cell.fg);
                        let dim_bg = darken_color(source_cell.bg);
                        
                        let sym = if dist == 1 { "█" } else if dist == 2 { "▓" } else { "▒" };
                        target_cell.set_symbol(sym).set_fg(dim_fg).set_bg(dim_bg);
                    }
                }
            }
        }

        if self.debug_mode {
            let dx = area.x + 2;
            let dy = area.y + 2;
            let dg_color = Color::Rgb(85, 255, 85);
            safe_set_string(buf, dx, dy,     "--- DIAGNOSTICS ---", dg_color);
            safe_set_string(buf, dx, dy + 1, &format!("FRM:  {:08}", self.frame_count), Color::White);
            safe_set_string(buf, dx, dy + 2, &format!("WTR:  {:?}", self.weather), Color::White);
            safe_set_string(buf, dx, dy + 3, &format!("CSH:  {:04}", self.chase_cooldown), Color::White);
            safe_set_string(buf, dx, dy + 4, &format!("SEED: {:016X}", self.window_seed), Color::White);
            safe_set_string(buf, dx, dy + 5, &format!("VHC:  {:03}", self.vehicles.len()), Color::White);
            safe_set_string(buf, dx, dy + 6, "-------------------", dg_color);
        }
    }
}

fn darken_color(c: Color) -> Color {
    match c {
        Color::Rgb(r, g, b) => Color::Rgb(r / 3, g / 3, b / 3),
        Color::Red => Color::Rgb(100, 0, 0),
        Color::Green => Color::Rgb(0, 100, 0),
        Color::Blue => Color::Rgb(0, 0, 100),
        Color::Yellow => Color::Rgb(100, 100, 0),
        Color::Cyan => Color::Rgb(0, 100, 100),
        Color::Magenta => Color::Rgb(100, 0, 100),
        Color::White => Color::Rgb(100, 100, 100),
        _ => Color::Rgb(10, 10, 20),
    }
}

fn safe_set_char(buf: &mut Buffer, x: u16, y: u16, ch: char, fg: Color) {
    if x < buf.area.width && y < buf.area.height { buf.get_mut(x, y).set_char(ch).set_fg(fg); }
}
fn safe_set_char_with_bg(buf: &mut Buffer, x: u16, y: u16, ch: char, fg: Color, bg: Color) {
    if x < buf.area.width && y < buf.area.height { buf.get_mut(x, y).set_char(ch).set_fg(fg).set_bg(bg); }
}
fn safe_set_symbol(buf: &mut Buffer, x: u16, y: u16, sym: &str, fg: Color) {
    if x < buf.area.width && y < buf.area.height { buf.get_mut(x, y).set_symbol(sym).set_fg(fg); }
}
fn safe_set_string(buf: &mut Buffer, x: u16, y: u16, s: &str, fg: Color) {
    if y < buf.area.height {
        for (i, ch) in s.chars().enumerate() {
            let dx = x.saturating_add(i as u16);
            if dx < buf.area.width { buf.get_mut(dx, y).set_char(ch).set_fg(fg); }
        }
    }
}
fn draw_roof_megaboard(buf: &mut Buffer, x: u16, y: u16, cpu: f32, ram: f32, frame: u64) {
    let accent = Color::Rgb(85, 255, 255);
    let dim_accent = Color::Rgb(0, 0, 170);
    
    let strut_pulse = ((frame as f32 * 0.05).sin() + 1.0) * 0.5;
    let strut_color = if strut_pulse > 0.8 { Color::Rgb(170, 170, 170) } else { Color::Rgb(85, 85, 85) };
    safe_set_symbol(buf, x + 4, y.saturating_sub(1), "╨", strut_color);
    safe_set_symbol(buf, x + 20, y.saturating_sub(1), "╨", strut_color);
    
    let board_y = y.saturating_sub(6);
    let width = 26;
    let height = 5;

    for dx in x..x+width {
        safe_set_symbol(buf, dx, board_y, "┄", Color::Rgb(170, 170, 170));
        safe_set_symbol(buf, dx, board_y + height - 1, "┄", Color::Rgb(170, 170, 170));
    }
    safe_set_symbol(buf, x, board_y, "⌜", Color::Rgb(170, 170, 170));
    safe_set_symbol(buf, x + width - 1, board_y, "⌝", Color::Rgb(170, 170, 170));
    safe_set_symbol(buf, x, board_y + height - 1, "⌞", Color::Rgb(170, 170, 170));
    safe_set_symbol(buf, x + width - 1, board_y + height - 1, "⌟", Color::Rgb(170, 170, 170));

    safe_set_string(buf, x + 2, board_y + 1, "CPU", Color::Rgb(170, 170, 170));
    let cpu_bars = (cpu / 6.6).min(15.0) as usize;
    for i in 0..15 {
        let bx = x + 7 + i as u16;
        let char = if i < cpu_bars { "❘" } else { " " };
        let color = if i < cpu_bars { accent } else { dim_accent };
        safe_set_symbol(buf, bx, board_y + 1, char, color);
    }
    safe_set_string(buf, x + 23, board_y + 1, &format!("{:>2.0}", cpu), Color::White);

    safe_set_string(buf, x + 2, board_y + 3, "MEM", Color::Rgb(170, 170, 170));
    let ram_bars = (ram / 6.6).min(15.0) as usize;
    for i in 0..15 {
        let bx = x + 7 + i as u16;
        let char = if i < ram_bars { "❘" } else { " " };
        let color = if i < ram_bars { Color::Rgb(255, 85, 255) } else { Color::Rgb(170, 0, 170) };
        safe_set_symbol(buf, bx, board_y + 3, char, color);
    }
    safe_set_string(buf, x + 23, board_y + 3, &format!("{:>2.0}", ram), Color::White);
}

fn draw_neon_sign(buf: &mut Buffer, x: u16, y: u16, text: &str, color: Color, frame: u64) {
    for (i, ch) in text.chars().enumerate() {
        let dy = y.saturating_add(i as u16);
        if dy < buf.area.height {
            let seed = (frame / 2).wrapping_add(i as u64).wrapping_add(x as u64);
            let mut rng = StdRng::seed_from_u64(seed);
            let final_color = if rng.gen_bool(0.95) { 
                color 
            } else { 
                Color::Rgb(30, 30, 45) 
            };
            
            let cell = buf.get_mut(x, dy);
            cell.set_char(ch).set_fg(final_color);
        }
    }
}


