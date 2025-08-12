use ratatui::style::Color;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TokyoNight;

impl TokyoNight {
    pub const BG: Color = Color::Rgb(26, 27, 38);           // #1a1b26
    pub const BG_DARK: Color = Color::Rgb(22, 22, 30);      // #16161e
    pub const BG_HIGHLIGHT: Color = Color::Rgb(41, 46, 66); // #292e42
    pub const TERMINAL_BLACK: Color = Color::Rgb(65, 72, 104); // #414868
    pub const FG: Color = Color::Rgb(195, 202, 222);        // #c3cade
    pub const FG_DARK: Color = Color::Rgb(169, 177, 214);   // #a9b1d6
    pub const FG_GUTTER: Color = Color::Rgb(59, 66, 97);    // #3b4261
    pub const DARK3: Color = Color::Rgb(84, 92, 126);       // #545c7e
    pub const COMMENT: Color = Color::Rgb(86, 95, 137);     // #565f89
    pub const DARK5: Color = Color::Rgb(115, 124, 153);     // #737aa2
    
    pub const BLUE: Color = Color::Rgb(125, 207, 255);      // #7dcfff
    pub const CYAN: Color = Color::Rgb(122, 220, 254);      // #7adcfe
    pub const BLUE1: Color = Color::Rgb(45, 149, 225);      // #2d95e1
    pub const BLUE2: Color = Color::Rgb(52, 152, 219);      // #3498db
    pub const BLUE5: Color = Color::Rgb(137, 221, 255);     // #89ddff
    pub const BLUE6: Color = Color::Rgb(180, 249, 248);     // #b4f9f8
    pub const BLUE7: Color = Color::Rgb(57, 97, 171);       // #3961ab
    
    pub const MAGENTA: Color = Color::Rgb(187, 154, 247);   // #bb9af7
    pub const MAGENTA2: Color = Color::Rgb(255, 146, 208);  // #ff92d0
    pub const PURPLE: Color = Color::Rgb(157, 124, 216);    // #9d7cd8
    
    pub const ORANGE: Color = Color::Rgb(255, 158, 100);    // #ff9e64
    pub const YELLOW: Color = Color::Rgb(224, 175, 104);    // #e0af68
    
    pub const GREEN: Color = Color::Rgb(115, 218, 202);     // #73daca
    pub const GREEN1: Color = Color::Rgb(158, 206, 106);    // #9ece6a
    pub const GREEN2: Color = Color::Rgb(65, 166, 181);     // #41a6b5
    pub const TEAL: Color = Color::Rgb(26, 188, 156);       // #1abc9c
    
    pub const RED: Color = Color::Rgb(247, 118, 142);       // #f7768e
    pub const RED1: Color = Color::Rgb(219, 75, 75);        // #db4b4b
}

pub struct Sparkle {
    particles: Vec<Particle>,
    last_update: u128,
}

#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    lifetime: f32,
    char: char,
    color: Color,
}

impl Sparkle {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            last_update: 0,
        }
    }
    
    pub fn update(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        if now - self.last_update > 100 {
            self.last_update = now;
            
            // Add new sparkles randomly
            if self.particles.len() < 15 && rand() < 0.3 {
                self.particles.push(Particle {
                    x: rand() * 100.0,
                    y: rand() * 30.0,
                    lifetime: 1.0,
                    char: SPARKLE_CHARS[(rand() * SPARKLE_CHARS.len() as f32) as usize],
                    color: SPARKLE_COLORS[(rand() * SPARKLE_COLORS.len() as f32) as usize],
                });
            }
            
            // Update existing particles
            self.particles.retain_mut(|p| {
                p.lifetime -= 0.1;
                p.y -= 0.5;
                p.lifetime > 0.0
            });
        }
    }
    
    pub fn get_sparkles(&self) -> Vec<(u16, u16, char, Color)> {
        self.particles
            .iter()
            .map(|p| {
                (
                    p.x as u16,
                    p.y as u16,
                    p.char,
                    fade_color(p.color, p.lifetime),
                )
            })
            .collect()
    }
}

const SPARKLE_CHARS: &[char] = &['✨', '⭐', '✦', '✧', '⋆', '･', '°', '∘', '⊹', '✵', '✶', '✷', '✸', '✹'];
const SPARKLE_COLORS: &[Color] = &[
    TokyoNight::CYAN,
    TokyoNight::BLUE,
    TokyoNight::MAGENTA,
    TokyoNight::PURPLE,
    TokyoNight::BLUE5,
];

fn fade_color(color: Color, alpha: f32) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 * alpha) as u8,
            (g as f32 * alpha) as u8,
            (b as f32 * alpha) as u8,
        ),
        _ => color,
    }
}

fn rand() -> f32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    ((now % 1000) as f32) / 1000.0
}

pub fn gradient_text(text: &str, start: Color, end: Color) -> Vec<(char, Color)> {
    let len = text.chars().count();
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            let t = i as f32 / len.max(1) as f32;
            let color = interpolate_color(start, end, t);
            (c, color)
        })
        .collect()
}

fn interpolate_color(start: Color, end: Color, t: f32) -> Color {
    match (start, end) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            Color::Rgb(
                lerp(r1 as f32, r2 as f32, t) as u8,
                lerp(g1 as f32, g2 as f32, t) as u8,
                lerp(b1 as f32, b2 as f32, t) as u8,
            )
        }
        _ => start,
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

pub fn get_status_color(is_ok: bool) -> Color {
    if is_ok {
        TokyoNight::GREEN
    } else {
        TokyoNight::RED
    }
}

pub fn get_level_color(level: &str) -> Color {
    match level.to_lowercase().as_str() {
        "info" => TokyoNight::BLUE,
        "warning" | "warn" => TokyoNight::YELLOW,
        "error" | "err" => TokyoNight::RED,
        "debug" => TokyoNight::COMMENT,
        _ => TokyoNight::FG,
    }
}

pub fn rainbow_border(tick: usize) -> Color {
    let colors = vec![
        TokyoNight::RED,
        TokyoNight::ORANGE,
        TokyoNight::YELLOW,
        TokyoNight::GREEN,
        TokyoNight::CYAN,
        TokyoNight::BLUE,
        TokyoNight::PURPLE,
        TokyoNight::MAGENTA,
    ];
    colors[tick % colors.len()]
}

pub fn pulse_color(base: Color, tick: usize) -> Color {
    let intensity = ((tick as f32 * 0.1).sin() + 1.0) / 2.0;
    match base {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 + (255.0 - r as f32) * intensity * 0.3) as u8,
            (g as f32 + (255.0 - g as f32) * intensity * 0.3) as u8,
            (b as f32 + (255.0 - b as f32) * intensity * 0.3) as u8,
        ),
        _ => base,
    }
}