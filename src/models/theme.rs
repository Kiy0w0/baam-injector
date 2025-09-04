use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeType {
    Pink,
    Purple, 
    Cyberpunk,
    Matrix,
    Retro,
}

impl Default for ThemeType {
    fn default() -> Self {
        ThemeType::Purple // Current theme as default
    }
}

impl ThemeType {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeType::Pink => "🌸 Pink Puff",
            ThemeType::Purple => "💜 Purple Vibes", 
            ThemeType::Cyberpunk => "🌆 Cyberpunk",
            ThemeType::Matrix => "🔋 Matrix",
            ThemeType::Retro => "🌈 Retro Wave",
        }
    }

    pub fn all() -> Vec<ThemeType> {
        vec![
            ThemeType::Pink,
            ThemeType::Purple,
            ThemeType::Cyberpunk, 
            ThemeType::Matrix,
            ThemeType::Retro,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    // Background colors
    pub bg_primary: Color32,
    pub bg_secondary: Color32,
    pub bg_tertiary: Color32,
    
    // Panel colors
    pub panel_bg: Color32,
    pub panel_stroke: Color32,
    
    // Button colors
    pub button_bg: Color32,
    pub button_bg_hovered: Color32,
    pub button_bg_active: Color32,
    pub button_text: Color32,
    
    // Accent colors
    pub accent_primary: Color32,
    pub accent_secondary: Color32,
    pub accent_tertiary: Color32,
    
    // Text colors
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_disabled: Color32,
    
    // Special colors
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
    
    // UI properties
    pub rounding: f32,
    pub border_width: f32,
}

impl Theme {
    pub fn from_type(theme_type: ThemeType) -> Self {
        match theme_type {
            ThemeType::Pink => Self::pink_puff(),
            ThemeType::Purple => Self::purple_vibes(),
            ThemeType::Cyberpunk => Self::cyberpunk(),
            ThemeType::Matrix => Self::matrix(),
            ThemeType::Retro => Self::retro_wave(),
        }
    }
    
    fn pink_puff() -> Self {
        Self {
            bg_primary: Color32::from_rgb(255, 240, 250),
            bg_secondary: Color32::from_rgb(252, 228, 244),
            bg_tertiary: Color32::from_rgb(248, 216, 238),
            
            panel_bg: Color32::from_rgb(255, 235, 248),
            panel_stroke: Color32::from_rgb(255, 182, 217),
            
            button_bg: Color32::from_rgb(255, 192, 227),
            button_bg_hovered: Color32::from_rgb(255, 167, 214),
            button_bg_active: Color32::from_rgb(255, 142, 201),
            button_text: Color32::from_rgb(139, 69, 116),
            
            accent_primary: Color32::from_rgb(255, 105, 180),
            accent_secondary: Color32::from_rgb(255, 182, 217),
            accent_tertiary: Color32::from_rgb(255, 218, 240),
            
            text_primary: Color32::from_rgb(139, 69, 116),
            text_secondary: Color32::from_rgb(169, 99, 146),
            text_disabled: Color32::from_rgb(199, 129, 176),
            
            success: Color32::from_rgb(144, 238, 144),
            warning: Color32::from_rgb(255, 218, 185),
            error: Color32::from_rgb(255, 182, 193),
            
            rounding: 15.0,
            border_width: 2.0,
        }
    }
    
    fn purple_vibes() -> Self {
        Self {
            bg_primary: Color32::from_rgb(240, 235, 255),
            bg_secondary: Color32::from_rgb(230, 220, 255),
            bg_tertiary: Color32::from_rgb(220, 205, 255),
            
            panel_bg: Color32::from_rgb(235, 225, 255),
            panel_stroke: Color32::from_rgb(186, 164, 255),
            
            button_bg: Color32::from_rgb(186, 164, 255),
            button_bg_hovered: Color32::from_rgb(161, 134, 255),
            button_bg_active: Color32::from_rgb(136, 104, 255),
            button_text: Color32::from_rgb(75, 0, 130),
            
            accent_primary: Color32::from_rgb(138, 43, 226),
            accent_secondary: Color32::from_rgb(186, 164, 255),
            accent_tertiary: Color32::from_rgb(221, 210, 255),
            
            text_primary: Color32::from_rgb(75, 0, 130),
            text_secondary: Color32::from_rgb(105, 30, 160),
            text_disabled: Color32::from_rgb(135, 60, 190),
            
            success: Color32::from_rgb(144, 238, 144),
            warning: Color32::from_rgb(255, 218, 185),
            error: Color32::from_rgb(255, 182, 193),
            
            rounding: 20.0,
            border_width: 3.0,
        }
    }
    
    fn cyberpunk() -> Self {
        Self {
            bg_primary: Color32::from_rgb(10, 10, 15),
            bg_secondary: Color32::from_rgb(15, 15, 25),
            bg_tertiary: Color32::from_rgb(20, 20, 35),
            
            panel_bg: Color32::from_rgb(12, 12, 20),
            panel_stroke: Color32::from_rgb(0, 255, 255),
            
            button_bg: Color32::from_rgb(30, 30, 50),
            button_bg_hovered: Color32::from_rgb(40, 40, 70),
            button_bg_active: Color32::from_rgb(0, 255, 255),
            button_text: Color32::from_rgb(0, 255, 255),
            
            accent_primary: Color32::from_rgb(255, 0, 255),
            accent_secondary: Color32::from_rgb(0, 255, 255),
            accent_tertiary: Color32::from_rgb(255, 255, 0),
            
            text_primary: Color32::from_rgb(0, 255, 255),
            text_secondary: Color32::from_rgb(255, 0, 255),
            text_disabled: Color32::from_rgb(100, 100, 150),
            
            success: Color32::from_rgb(0, 255, 0),
            warning: Color32::from_rgb(255, 255, 0),
            error: Color32::from_rgb(255, 0, 0),
            
            rounding: 5.0,
            border_width: 2.0,
        }
    }
    
    fn matrix() -> Self {
        Self {
            bg_primary: Color32::from_rgb(0, 0, 0),
            bg_secondary: Color32::from_rgb(5, 10, 5),
            bg_tertiary: Color32::from_rgb(10, 20, 10),
            
            panel_bg: Color32::from_rgb(2, 8, 2),
            panel_stroke: Color32::from_rgb(0, 255, 0),
            
            button_bg: Color32::from_rgb(0, 50, 0),
            button_bg_hovered: Color32::from_rgb(0, 80, 0),
            button_bg_active: Color32::from_rgb(0, 255, 0),
            button_text: Color32::from_rgb(0, 255, 0),
            
            accent_primary: Color32::from_rgb(0, 255, 0),
            accent_secondary: Color32::from_rgb(50, 255, 50),
            accent_tertiary: Color32::from_rgb(100, 255, 100),
            
            text_primary: Color32::from_rgb(0, 255, 0),
            text_secondary: Color32::from_rgb(0, 200, 0),
            text_disabled: Color32::from_rgb(0, 100, 0),
            
            success: Color32::from_rgb(0, 255, 0),
            warning: Color32::from_rgb(255, 255, 0),
            error: Color32::from_rgb(255, 0, 0),
            
            rounding: 0.0,
            border_width: 1.0,
        }
    }
    
    fn retro_wave() -> Self {
        Self {
            bg_primary: Color32::from_rgb(25, 10, 50),
            bg_secondary: Color32::from_rgb(35, 20, 70),
            bg_tertiary: Color32::from_rgb(45, 30, 90),
            
            panel_bg: Color32::from_rgb(30, 15, 60),
            panel_stroke: Color32::from_rgb(255, 20, 147),
            
            button_bg: Color32::from_rgb(60, 30, 120),
            button_bg_hovered: Color32::from_rgb(80, 50, 140),
            button_bg_active: Color32::from_rgb(255, 20, 147),
            button_text: Color32::from_rgb(255, 255, 255),
            
            accent_primary: Color32::from_rgb(255, 20, 147),
            accent_secondary: Color32::from_rgb(0, 255, 255),
            accent_tertiary: Color32::from_rgb(255, 255, 0),
            
            text_primary: Color32::from_rgb(255, 255, 255),
            text_secondary: Color32::from_rgb(255, 20, 147),
            text_disabled: Color32::from_rgb(150, 150, 150),
            
            success: Color32::from_rgb(0, 255, 127),
            warning: Color32::from_rgb(255, 215, 0),
            error: Color32::from_rgb(255, 69, 0),
            
            rounding: 8.0,
            border_width: 2.0,
        }
    }
}
