//! The Menu and User Interface system.
//!
//! This module manages all the non-gameplay screens, such as the Main Menu,
//! the Pause Screen, and the Game Over screen. It handles drawing buttons and text,
//! processing mouse clicks, and taking care of saving and loading the player's progress.

use font8x8::{BASIC_FONTS, UnicodeFonts};
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window};
use std::fs;

use crate::audio::audio_handler::Audio;
use crate::game::gamestate::Game;
use crate::render::RendererData;
use crate::custom_key_bindings::{Action,Keybindings, ALL_ACTIONS};

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 450;

const COL_BG_MAIN: u32 = 0x111111;
const COL_BG_GAMEOVER: u32 = 0x330000;
const COL_BG_PAUSED: u32 = 0x002244;
const COL_TITLE_RED: u32 = 0xCC0000;
const COL_TITLE_BLUE: u32 = 0x44AAFF;
const COL_BTN_NORMAL: u32 = 0x444444;
const COL_BTN_HOVER: u32 = 0xAA0000;
const COL_TEXT_WHITE: u32 = 0xFFFFFF;
const COL_BINDINGS_TEXT: u32 = 0x39FF14;
const COL_SELECTED_ACTION: u32 = 0xAA6600;

/// Dictates the high-level routing in the `main.rs` loop.
pub enum AppState {
    StartScreen,
    Playing,
    GameOver,
    GamePaused,
    Keybind,
    Quit,
}
/// Simple layout container for button definitions.
struct ButtonDef {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    text: String,
}

/// Holds the minimal state required for the IMGUI.
/// Since `minifb` natively handles keyboard debouncing via `KeyRepeat::No`,
/// we only need to manually track the mouse state for edge detection (clicks).
pub struct Menu {
    was_mouse_down: bool,
    key_capture: Option<Action>,
    binding_message: String,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            was_mouse_down: false,
            key_capture: None,
            binding_message: String::new(),
        }
    }

    /// Renders the main menu and handles user interactions.
    pub fn update_and_draw_start_menu(
        &mut self,
        window: &mut Window,
        buffer: &mut [u32],
        game: &mut Game,
        renderer_data: &RendererData,
        audio: &mut Audio,
    ) -> AppState {
        // Clear the buffer from the previous frame
        buffer.fill(COL_BG_MAIN);

        // Fetch user inputs
        let (mx, my, clicked) = self.get_mouse_state(window);
        let click1 = window.is_key_pressed(Key::Key1, KeyRepeat::No);
        let click2 = window.is_key_pressed(Key::Key2, KeyRepeat::No);
        let click3 = window.is_key_pressed(Key::Key3, KeyRepeat::No);
        let click4 = window.is_key_pressed(Key::Key4, KeyRepeat::No);

        // Render titles
        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "DOOMSQUAD",
            50,
            6,
            COL_TITLE_RED,
        );
        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "MAIN MENU",
            105,
            2,
            COL_TEXT_WHITE,
        );

        // Define button layouts
        let btn_start = ButtonDef {
            x: 230,
            y: 150,
            w: 340,
            h: 40,
            text: "[1] NEW GAME".to_string(),
        };
        let btn_load = ButtonDef {
            x: 230,
            y: 200,
            w: 340,
            h: 40,
            text: "[2] LOAD MAP".to_string(),
        };

        let audio_text = if !audio.is_muted {
            "[3] SOUND: ON"
        } else {
            "[3] SOUND: OFF"
        };
        let btn_audio = ButtonDef {
            x: 230,
            y: 250,
            w: 340,
            h: 40,
            text: audio_text.to_string(),
        };

        // a new option for configuring key bindings on main menu
        let btn_keybind = ButtonDef {
            x: 230,
            y: 300,
            w: 340,
            h: 40,
            text: "[4] CUSTOM KEYBINDING".to_string(),
        };

        let btn_quit = ButtonDef {
            x: 230,
            y: 350,
            w: 340,
            h: 40,
            text: "[ESC] QUIT GAME".to_string(),
        };

        // Interaction processing
        if self.draw_flat_button(buffer, &btn_start, mx, my) && clicked || click1 {
            *game = Game::new_game(renderer_data).unwrap(); // ! this unwrap is intentionally accepted
            game.map_swap(renderer_data, 1); // Jump straight to map 1 (map 0 is reserved for testing)
            return AppState::Playing;
        }

        #[allow(clippy::collapsible_if)]
        if self.draw_flat_button(buffer, &btn_load, mx, my) && clicked || click2 {
            // Read saved map index from disk and safely parse it
            if let Ok(content) = fs::read_to_string("savegame.txt")
                && let Ok(index) = content.trim().parse::<usize>()
            {
                *game = Game::new_game(renderer_data).unwrap(); // ! this unwrap is intentionally accepted
                game.map_swap(renderer_data, index);
                return AppState::Playing;
            }
        }

        if self.draw_flat_button(buffer, &btn_audio, mx, my) && clicked || click3 {
            audio.set_muted(!audio.is_muted);
            return AppState::StartScreen;
        }

        let click4 = window.is_key_pressed(Key::Key4, KeyRepeat::No);
        if self.draw_flat_button(buffer, &btn_keybind, mx, my) && clicked || click4 {
            self.key_capture = None;
            self.binding_message.clear();
            return AppState::Keybind;
        }


        if self.draw_flat_button(buffer, &btn_quit, mx, my) && clicked {
            return AppState::Quit;
        }

        // Keep looping in the menu until an action changes the state
        AppState::StartScreen
    }

    // new function to handle new keybindings
    //updated the function to draw the actions and related keybuttons
    //also possible to change keybinding
    pub fn display_and_update_keybinding(
        &mut self,
        window: &mut Window,
        buffer: &mut [u32],
        keybinding: &mut Keybindings,
    ) -> AppState {
        buffer.fill(COL_BG_MAIN);
        Self::draw_text_centered(buffer, SCREEN_WIDTH, SCREEN_HEIGHT, "CONFIGURE KEY BINDINGS", 18, 3, COL_TITLE_BLUE);
        Self::draw_text(buffer, SCREEN_WIDTH, SCREEN_HEIGHT,
            "Choose a binding -> press a key to select. ESC = cancel or back.", 70, 58, 1, COL_TEXT_WHITE);

        let (mx, my, clicked) = self.get_mouse_state(window);
        let keys_pressed = window.get_keys_pressed(KeyRepeat::No);
        if let Some(action) = self.key_capture {
            if keys_pressed.contains(&Key::Escape) {
                self.key_capture = None;
                self.binding_message = "Key assignment cancelled.".to_string();
            } else if let Some(key) = keys_pressed.first().copied() {
                //set new keybinding for new action
                    keybinding.set(action, key);
                    self.key_capture = None;
                }
        }

        // draw key bindings for all actions
        for (index, action) in ALL_ACTIONS.iter().copied().enumerate() {
            // 10 rows in one column
            let (x, y) = if index < 10 {
                (24usize, 105usize + index * 29)
            } else {
                (414usize, 105usize + (index - 10) * 29)
            };
            // Actions Labels to be displayed
            let label = action.label();
            Self::draw_text(buffer, SCREEN_WIDTH, SCREEN_HEIGHT, label, x, y + 7, 1, COL_TEXT_WHITE);
            let bx = if index < 10 { 250 } else { 640 };
            let by = y;
            let bw = 125;
            let bh = 25;
            let selected = self.key_capture == Some(action);
            //showing button colors based on hover/selection
            let hover = mx >= bx && mx <= bx + bw && my >= by && my <= by + bh;
            let color = if selected { COL_SELECTED_ACTION } else if hover { COL_BTN_HOVER } else { COL_BTN_NORMAL };
            for row in by..(by + bh) {
                for col in bx..(bx + bw) {
                    if row < SCREEN_HEIGHT && col < SCREEN_WIDTH { buffer[row * SCREEN_WIDTH + col] = color; }
                }
            }
            let key_label = if selected { "PRESS KEY..." } else { keybinding.name(action) };
            Self::draw_text(buffer, SCREEN_WIDTH, SCREEN_HEIGHT, key_label, bx + 5, by + 8, 1, COL_TEXT_WHITE);
            if clicked && hover {
                self.key_capture = Some(action);
                self.binding_message = format!("Press a key for {}", action.label());
            }
        }

        if !self.binding_message.is_empty() {
            Self::draw_text(buffer, SCREEN_WIDTH, SCREEN_HEIGHT, &self.binding_message, 24, 397, 1, COL_BINDINGS_TEXT);
        }
        //creating the back button
        let back = ButtonDef { x: 640, y: 410, w: 135, h: 28, text: "BACK".to_string() };
        if self.draw_flat_button(buffer, &back, mx, my) && clicked {
            self.key_capture = None;
            return { AppState::StartScreen };
        }
        if keys_pressed.contains(&Key::Escape) && self.key_capture.is_none() {
            return { AppState::StartScreen };
        }
        AppState::Keybind
    }



    /// Renders the death screen and allows the user to save their current level progress.
    pub fn update_and_draw_game_over(
        &mut self,
        window: &mut Window,
        buffer: &mut [u32],
        game: &mut Game,
    ) -> AppState {
        buffer.fill(COL_BG_GAMEOVER);

        let (mx, my, clicked) = self.get_mouse_state(window);
        let click1 = window.is_key_pressed(Key::Key1, KeyRepeat::No);

        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "YOU DIED",
            80,
            6,
            COL_TITLE_RED,
        );

        let btn_save = ButtonDef {
            x: 230,
            y: 220,
            w: 340,
            h: 60,
            text: "[1] SAVE MAP INDEX".to_string(),
        };
        let btn_menu = ButtonDef {
            x: 230,
            y: 300,
            w: 340,
            h: 60,
            text: "[ESC] MAIN MENU".to_string(),
        };

        if self.draw_flat_button(buffer, &btn_save, mx, my) && clicked || click1 {
            // Persist the current level index to disk so it can be loaded later
            let _ = fs::write("savegame.txt", game.map_index.to_string());
            return AppState::StartScreen;
        }

        if self.draw_flat_button(buffer, &btn_menu, mx, my) && clicked {
            return AppState::StartScreen;
        }

        AppState::GameOver
    }

    /// Renders the pause menu (triggered by pressing ESC during gameplay).
    pub fn update_and_draw_game_paused(
        &mut self,
        window: &mut Window,
        buffer: &mut [u32],
        game: &mut Game,
        audio: &mut Audio,
    ) -> AppState {
        buffer.fill(COL_BG_PAUSED);

        let (mx, my, clicked) = self.get_mouse_state(window);

        let click1 = window.is_key_pressed(Key::Key1, KeyRepeat::No);
        let click2 = window.is_key_pressed(Key::Key2, KeyRepeat::No);
        let click3 = window.is_key_pressed(Key::Key3, KeyRepeat::No);

        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "GAME PAUSED",
            60,
            6,
            COL_TITLE_BLUE,
        );

        let audio_text = if !audio.is_muted {
            "[3] SOUND: ON"
        } else {
            "[3] SOUND: OFF"
        };

        let btn_resume = ButtonDef {
            x: 230,
            y: 150,
            w: 340,
            h: 50,
            text: "[1] RESUME GAME".to_string(),
        };
        let btn_save = ButtonDef {
            x: 230,
            y: 210,
            w: 340,
            h: 50,
            text: "[2] SAVE & EXIT".to_string(),
        };
        let btn_audio = ButtonDef {
            x: 230,
            y: 270,
            w: 340,
            h: 50,
            text: audio_text.to_string(),
        };
        let btn_menu = ButtonDef {
            x: 230,
            y: 330,
            w: 340,
            h: 50,
            text: "[ESC] MAIN MENU".to_string(),
        };

        if self.draw_flat_button(buffer, &btn_resume, mx, my) && clicked || click1 {
            // Simply return to the Playing state without touching the game object to resume seamlessly
            return AppState::Playing;
        }

        if self.draw_flat_button(buffer, &btn_save, mx, my) && clicked || click2 {
            let _ = std::fs::write("savegame.txt", game.map_index.to_string());
            return AppState::StartScreen;
        }

        if self.draw_flat_button(buffer, &btn_audio, mx, my) && clicked || click3 {
            audio.set_muted(!audio.is_muted);
            return AppState::GamePaused;
        }

        if self.draw_flat_button(buffer, &btn_menu, mx, my) && clicked {
            return AppState::StartScreen;
        }

        AppState::GamePaused
    }

    /// Fetches mouse coordinates and performs edge detection (debouncing) for left-clicks.
    fn get_mouse_state(&mut self, window: &Window) -> (usize, usize, bool) {
        // Clamp prevents out-of-bounds coordinates if the mouse leaves the window frame
        let mouse_pos = window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
        let is_mouse_down = window.get_mouse_down(MouseButton::Left);

        let clicked = is_mouse_down && !self.was_mouse_down;
        self.was_mouse_down = is_mouse_down;

        (mouse_pos.0 as usize, mouse_pos.1 as usize, clicked)
    }

    /// Draws a rectangular button and its text onto the buffer.
    /// Returns `true` if the mouse cursor is currently hovering over it.
    fn draw_flat_button(&self, buffer: &mut [u32], btn: &ButtonDef, mx: usize, my: usize) -> bool {
        // Simple check for hover detection
        let hover = mx >= btn.x && mx <= (btn.x + btn.w) && my >= btn.y && my <= (btn.y + btn.h);
        let bg_color = if hover { COL_BTN_HOVER } else { COL_BTN_NORMAL };

        // Rasterize the rectangle into the 1D pixel buffer
        for row in btn.y..(btn.y + btn.h) {
            for col in btn.x..(btn.x + btn.w) {
                // Safety check to prevent panic from out-of-bounds memory writes
                if row < SCREEN_HEIGHT && col < SCREEN_WIDTH {
                    buffer[row * SCREEN_WIDTH + col] = bg_color;
                }
            }
        }

        // Center the text horizontally and vertically inside the button bounds
        let text_width = btn.text.len() * 8 * 2;
        let text_x = btn.x + btn.w.saturating_sub(text_width) / 2;
        let text_y = btn.y + btn.h.saturating_sub(16) / 2;

        Self::draw_text(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &btn.text,
            text_x,
            text_y,
            2,
            COL_TEXT_WHITE,
        );

        hover
    }

    /// Helper to align a given text string exactly in the horizontal center of the screen.
    fn draw_text_centered(
        buffer: &mut [u32],
        screen_width: usize,
        screen_height: usize,
        text: &str,
        y: usize,
        scale: usize,
        color: u32,
    ) {
        // Each character in the font is 8 pixels wide, multiplied by the scaling factor
        let text_width = text.len() * 8 * scale;
        let x = screen_width.saturating_sub(text_width) / 2;

        Self::draw_text(
            buffer,
            screen_width,
            screen_height,
            text,
            x,
            y,
            scale,
            color,
        );
    }

    /// Software text rasterizer using the `font8x8` library.
    /// It translates strings into raw pixel data and pushes them to the 1D buffer.
    #[allow(clippy::too_many_arguments)]
    fn draw_text(
        buffer: &mut [u32],
        screen_width: usize,
        screen_height: usize,
        text: &str,
        start_x: usize,
        start_y: usize,
        scale: usize,
        color: u32,
    ) {
        let mut cursor_x = start_x;

        for c in text.chars() {
            if let Some(glyph) = BASIC_FONTS.get(c) {
                #[allow(clippy::needless_range_loop)]
                for y in 0..8 {
                    for x in 0..8 {
                        // Bitwise check: if the specific bit in the glyph byte is a 1, a pixel needs to be drawn
                        if (glyph[y] & (1 << x)) != 0 {
                            // Nearest-neighbor scaling (draws larger 2D blocks for each active pixel based on the `scale` factor)
                            for dy in 0..scale {
                                for dx in 0..scale {
                                    let px = cursor_x + x * scale + dx;
                                    let py = start_y + y * scale + dy;

                                    if px < screen_width && py < screen_height {
                                        // Flatten 2D coordinate into a 1D array index
                                        buffer[py * screen_width + px] = color;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Advance the cursor by the width of the character to prepare for the next letter
            cursor_x += 8 * scale;
        }
    }
}
