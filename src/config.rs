use macroquad::prelude::Color;

// ============================================================================
// UI COLOR CONFIGURATION
// ============================================================================

// Base UI Colors
pub const COLOR_UI_ACCENT_CYAN: Color = Color::from_rgba(0, 229, 255, 255); 
pub const COLOR_UI_PANEL_BG: Color = Color::from_rgba(20, 26, 42, 230); 
pub const COLOR_UI_PANEL_BORDER: Color = Color::from_rgba(50, 80, 130, 200); 
pub const COLOR_UI_PANEL_TITLE_BG: Color = Color::from_rgba(30, 42, 68, 255); 
pub const COLOR_UI_PANEL_TITLE_LINE: Color = Color::from_rgba(60, 100, 160, 255); 

// Button Colors
pub const COLOR_UI_BUTTON_BG: Color = Color::from_rgba(30, 40, 62, 220); 
pub const COLOR_UI_BUTTON_BORDER: Color = Color::from_rgba(60, 90, 140, 255); 
pub const COLOR_UI_BUTTON_TEXT: Color = Color::from_rgba(220, 230, 245, 255); 

// Input Colors
pub const COLOR_UI_INPUT_BG: Color = Color::from_rgba(15, 22, 36, 255); 
pub const COLOR_UI_INPUT_BORDER: Color = Color::from_rgba(60, 80, 120, 255); 
pub const COLOR_UI_INPUT_LABEL: Color = Color::from_rgba(160, 180, 210, 255); 

// Event Log Colors
pub const COLOR_LOG_DEFAULT: Color = Color::from_rgba(180, 200, 225, 255); 

// Name Tag Colors
pub const COLOR_NAME_TAG_BG: Color = Color::from_rgba(0, 0, 0, 180); 

// Other colors
pub const COLOR_ERROR: Color = Color::from_rgba(239, 83, 80, 255); 


// ============================================================================
// UI DIMENSION & LAYOUT CONFIGURATION
// ============================================================================

// Panel Layout
pub const UI_PANEL_BORDER_THICKNESS: f32 = 2.0; 
pub const UI_PANEL_TITLE_HEIGHT: f32 = 32.0; 
pub const UI_PANEL_TITLE_LINE_THICKNESS: f32 = 1.5; 
pub const UI_PANEL_TITLE_TEXT_OFFSET_X: f32 = 12.0; 
pub const UI_PANEL_TITLE_TEXT_OFFSET_Y: f32 = 22.0; 

// Button Layout
pub const UI_BUTTON_BORDER_THICKNESS: f32 = 2.0; 
pub const UI_BUTTON_TEXT_OFFSET_Y: f32 = -2.0; 
pub const UI_BUTTON_HOVER_DARKEN: f32 = 0.8; 

// Input Layout
pub const UI_INPUT_LABEL_OFFSET_Y: f32 = -6.0; 
pub const UI_INPUT_BORDER_THICKNESS: f32 = 1.5; 
pub const UI_INPUT_TEXT_OFFSET_X: f32 = 8.0; 
pub const UI_INPUT_TEXT_OFFSET_Y: f32 = 5.0; 

// Logs Layout
pub const UI_LOG_LINE_HEIGHT: f32 = 18.0; 
pub const UI_LOG_START_OFFSET_Y: f32 = 48.0; 
pub const UI_LOG_TEXT_OFFSET_X: f32 = 12.0; 
pub const UI_LOG_HEIGHT_PADDING: f32 = 55.0; 

// ============================================================================
// UI FONT SIZE CONFIGURATION
// ============================================================================

// Font size constants
pub const FONT_SIZE_TITLE: f32 = 38.0;
pub const FONT_SIZE_LARGE: f32 = 28.0;
pub const FONT_SIZE_MEDIUM: f32 = 14.0;
pub const FONT_SIZE_REGULAR: f32 = 12.0;
pub const FONT_SIZE_SMALL: f32 = 11.0;
pub const FONT_SIZE_TINY: f32 = 10.0;

// ============================================================================
// GAME WORLD COLOR CONFIGURATION
// ============================================================================

// Map & Background Colors
pub const COLOR_WORLD_BG_CLEAR: Color = Color::from_rgba(15, 20, 32, 255); 
pub const COLOR_WORLD_MAP_BG: Color = Color::from_rgba(22, 30, 48, 255); 
pub const COLOR_WORLD_MAP_BORDER: Color = Color::from_rgba(64, 120, 200, 180); 
pub const COLOR_WORLD_GRID_LINE: Color = Color::from_rgba(255, 255, 255, 12); 

// Central Circle Colors
pub const COLOR_CIRCLE_BG: Color = Color::from_rgba(0, 229, 255, 20); 
pub const COLOR_CIRCLE_TEXT: Color = Color::from_rgba(0, 229, 255, 220); 

// Flag Colors
pub const COLOR_FLAG_GLOW_AVAILABLE: Color = Color::from_rgba(255, 215, 0, 40); 
pub const COLOR_FLAG_GLOW_CARRIED: Color = Color::from_rgba(255, 215, 0, 80); 
pub const COLOR_FLAG_CENTER_AVAILABLE: Color = Color::from_rgba(255, 215, 0, 255); 
pub const COLOR_FLAG_INTERACT_RADIUS: Color = Color::from_rgba(255, 215, 0, 100); 

// Player Rendering Colors
pub const COLOR_PLAYER_LOCAL_AURA: Color = Color::from_rgba(255, 255, 255, 120); 
pub const PLAYER_COLORS: [Color; 8] = [
    Color::from_rgba(239, 83, 80, 255),  
    Color::from_rgba(66, 165, 245, 255), 
    Color::from_rgba(102, 187, 106, 255),
    Color::from_rgba(171, 71, 188, 255), 
    Color::from_rgba(255, 167, 38, 255), 
    Color::from_rgba(38, 198, 218, 255), 
    Color::from_rgba(236, 64, 122, 255), 
    Color::from_rgba(212, 225, 87, 255), 
];


// ============================================================================
// GAME WORLD RENDER DIMENSIONS
// ============================================================================

// Camera
pub const CAMERA_DEFAULT_ZOOM: f32 = 0.35; 

// Map & Grid
pub const RENDER_MAP_BORDER_THICKNESS: f32 = 3.0; 
pub const RENDER_GRID_STEP: f32 = 200.0; 
pub const RENDER_GRID_LINE_THICKNESS: f32 = 1.0; 

// Central Circle
pub const RENDER_CIRCLE_BORDER_THICKNESS: f32 = 4.0; 
pub const RENDER_CIRCLE_TEXT_OFFSET_Y: f32 = 10.0; 

// Flag Metrics
pub const RENDER_FLAG_INTERACT_BORDER_THICKNESS: f32 = 1.5; 
pub const RENDER_FLAG_POLE_AVAILABLE_HEIGHT: f32 = 35.0; 
pub const RENDER_FLAG_POLE_CARRIED_HEIGHT: f32 = 40.0; 
pub const RENDER_FLAG_POLE_LINE_THICKNESS: f32 = 3.0; 
pub const RENDER_FLAG_CARRIED_BORDER_THICKNESS: f32 = 2.5; 

// Player Metrics
pub const RENDER_PLAYER_OUTLINE_THICKNESS: f32 = 2.0; 
pub const RENDER_PLAYER_DIR_LINE_THICKNESS: f32 = 3.0; 
pub const RENDER_PLAYER_DIR_DOT_RADIUS: f32 = 3.0; 
pub const RENDER_PLAYER_DIR_MULTIPLIER: f32 = 1.6; 
pub const RENDER_PLAYER_AURA_OFFSET: f32 = 6.0; 

// Player Name Tags
pub const UI_NAME_TAG_OFFSET_Y: f32 = 8.0; 
pub const UI_NAME_TAG_PADDING_X: f32 = 4.0; 
pub const UI_NAME_TAG_PADDING_Y: f32 = 12.0; 
pub const UI_NAME_TAG_HEIGHT: f32 = 16.0; 
