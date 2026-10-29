use minifb::Key;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

// creating all possible actions
pub enum Action {
    MoveForward, MoveBackward, MoveLeft, MoveRight,
    Sprint, Jump, Crouch, RocketJump, Interact, AimUp, AimDown,TurnLeft, TurnRight,
    ToggleAim, FireBullet, FireArrow, ToggleGodMode, FlyUp,FlyDown, LockActiveEnemies,
}

// array of Actions
pub const ALL_ACTIONS: [Action; 20] = [
    Action::MoveForward, Action::MoveBackward, Action::MoveLeft, Action::MoveRight,
    Action::Sprint, Action::Jump,  Action::Crouch, Action::RocketJump, Action::Interact,
    Action::AimUp, Action::AimDown, Action::TurnLeft, Action::TurnRight,
    Action::ToggleAim,  Action::FireBullet, Action::FireArrow,
    Action::ToggleGodMode, Action::FlyUp, Action::FlyDown,
    Action::LockActiveEnemies,
    ];

// creating labels for Actions
impl Action {
    pub fn label(self) -> &'static str {
        match self {
            Self::MoveForward => "Move forward", Self::MoveBackward => "Move backward",
            Self::MoveLeft => "Move Left", Self::MoveRight => "Move Right",
            Self::Sprint => "Sprint", Self::Jump => "Jump",  Self::Crouch => "Crouch", Self::RocketJump => "Rocket jump",
            Self::Interact => "Interact",  Self::AimUp => "Aim up", Self::AimDown => "Aim down",
            Self::TurnLeft => "Turn Left", Self::TurnRight => "Turn Right",
            Self::ToggleAim => "Toggle aim mode", Self::FireBullet => "Fire bullet", Self::FireArrow => "Fire arrow",
            Self::ToggleGodMode => "Toggle God mode", Self::FlyUp => "Fly up (God mode)", Self::FlyDown => "Fly down (God mode)",
            Self::LockActiveEnemies => "Lock active enemies",
        }
    }
}

// declaring the default doom style controls to be used for custom keybindings
pub struct Keybindings {
            move_forward: String, move_backward: String, move_left: String,
            move_right: String, sprint: String, jump:String,
            crouch: String, rocket_jump: String, interact:String,
            aim_up: String, aim_down: String, turn_left: String, turn_right:String,
            toggle_aim: String, fire_bullet: String,fire_arrow: String,
            //god mode controls
            toggle_god_mode: String, fly_up: String, fly_down: String,
            lock_active_enemies: String,
}

// defining the initial keybindings (default doom style controls with default keybindings) for the game,
impl Default for Keybindings {
    fn default() -> Self {
        if crate::DEBUG { println!("Using default keybindings.");
        }
        Self {
            move_forward: "W".to_string(), move_backward: "S".to_string(), move_left: "A".to_string(),
            move_right: "D".to_string(), sprint: "LeftShift".to_string(), jump: "Space".to_string(),
            crouch: "C".to_string(), rocket_jump: "R".to_string(), interact: "F".to_string(),
            aim_up: "Up".to_string(), aim_down: "Down".to_string(), turn_left: "Left".to_string(), turn_right: "Right".to_string(),
            toggle_aim: "E".to_string(), fire_bullet: "RightCtrl".to_string(),fire_arrow: "RightShift".to_string(),
            //god mode controls
            toggle_god_mode: "G".to_string(), fly_up: "Space".to_string(), fly_down: "LeftShift".to_string(),
            lock_active_enemies: "L".to_string(),
        }
    }
}

impl Keybindings {
    //load default keybindings
    pub fn load() -> Self {
        // use default values
        Self::default()
    }

    // return keyboard key for given Action, from Keybinding
    pub fn key(&self, action: Action) -> Key {
        match key_from_name(self.name(action)) {
        Some(key) => key,
        None => default_key(action),
        }
    }

    //return Key string for a given Action
    pub fn name(&self, action: Action) -> &str {
        match action {
            Action::MoveForward => &self.move_forward, Action::MoveBackward => &self.move_backward,
            Action::MoveLeft => &self.move_left, Action::MoveRight => &self.move_right,
            Action::TurnLeft => &self.turn_left, Action::TurnRight => &self.turn_right,
            Action::Jump => &self.jump, Action::Sprint => &self.sprint, Action::Crouch => &self.crouch,
            Action::Interact => &self.interact, Action::ToggleAim => &self.toggle_aim,
            Action::FireBullet => &self.fire_bullet, Action::FireArrow => &self.fire_arrow,
            Action::RocketJump => &self.rocket_jump, Action::ToggleGodMode => &self.toggle_god_mode,
            Action::FlyUp => &self.fly_up, Action::FlyDown => &self.fly_down,
            Action::AimUp => &self.aim_up, Action::AimDown => &self.aim_down,
            Action::LockActiveEnemies => &self.lock_active_enemies,
        }
    }

    // Setting a new key value for a given action.
    pub fn set(&mut self, action: Action, key: Key) {
        //convert keyboard key to a string
        let key_str = format!("{key:?}");
        // dereferencing the String pointer for the given action, to assign new key value
        *self.name_mut(action) = key_str;
    }

    fn name_mut(&mut self, action: Action) -> &mut String {
        match action {
            Action::MoveForward => &mut self.move_forward, Action::MoveBackward => &mut self.move_backward,
            Action::MoveLeft => &mut self.move_left, Action::MoveRight => &mut self.move_right,
            Action::TurnLeft => &mut self.turn_left, Action::TurnRight => &mut self.turn_right,
            Action::Jump => &mut self.jump, Action::Sprint => &mut self.sprint, Action::Crouch => &mut self.crouch,
            Action::Interact => &mut self.interact, Action::ToggleAim => &mut self.toggle_aim,
            Action::FireBullet => &mut self.fire_bullet, Action::FireArrow => &mut self.fire_arrow,
            Action::RocketJump => &mut self.rocket_jump, Action::ToggleGodMode => &mut self.toggle_god_mode,
            Action::FlyUp => &mut self.fly_up, Action::FlyDown => &mut self.fly_down,
            Action::AimUp => &mut self.aim_up, Action::AimDown => &mut self.aim_down,
            Action::LockActiveEnemies => &mut self.lock_active_enemies,
        }
    }
}

fn default_key(action: Action) -> Key {
    key_from_name(match action {
        Action::MoveForward => "W", Action::MoveBackward => "S", Action::MoveLeft => "A",
        Action::MoveRight => "D", Action::TurnLeft => "Left", Action::TurnRight => "Right",
        Action::Jump | Action::FlyUp => "Space", Action::Sprint | Action::FlyDown => "LeftShift",
        Action::Crouch => "C", Action::Interact => "F", Action::ToggleAim => "E",
        Action::FireBullet => "RightCtrl", Action::FireArrow => "RightShift", Action::RocketJump => "R",
        Action::ToggleGodMode => "G", Action::AimUp => "Up", Action::AimDown => "Down",
        Action::LockActiveEnemies => "L", Action::FlyUp => "Space", Action::FlyDown => "LeftShift",
    }).unwrap()
}

// Convert minifb's debug-formatted key names back into Key values.
pub fn key_from_name(s: &str) -> Option<Key> {
    Some(match s {
        "A"=>Key::A,"B"=>Key::B,"C"=>Key::C,"D"=>Key::D,"E"=>Key::E,"F"=>Key::F,"G"=>Key::G,
        "H"=>Key::H,"I"=>Key::I,"J"=>Key::J,"K"=>Key::K,"L"=>Key::L,"M"=>Key::M,"N"=>Key::N,
        "O"=>Key::O,"P"=>Key::P,"Q"=>Key::Q,"R"=>Key::R,"S"=>Key::S,"T"=>Key::T,"U"=>Key::U,
        "V"=>Key::V,"W"=>Key::W,"X"=>Key::X,"Y"=>Key::Y,"Z"=>Key::Z,
        "Key0"=>Key::Key0,"Key1"=>Key::Key1,"Key2"=>Key::Key2,"Key3"=>Key::Key3,"Key4"=>Key::Key4,
        "Key5"=>Key::Key5,"Key6"=>Key::Key6,"Key7"=>Key::Key7,"Key8"=>Key::Key8,"Key9"=>Key::Key9,
        "F1"=>Key::F1,"F2"=>Key::F2,"F3"=>Key::F3,"F4"=>Key::F4,"F5"=>Key::F5,"F6"=>Key::F6,
        "F7"=>Key::F7,"F8"=>Key::F8,"F9"=>Key::F9,"F10"=>Key::F10,"F11"=>Key::F11,"F12"=>Key::F12,
        "Escape"=>Key::Escape,"Insert"=>Key::Insert,"Delete"=>Key::Delete,"Home"=>Key::Home,"End"=>Key::End,
        "PageUp"=>Key::PageUp,"PageDown"=>Key::PageDown,"Left"=>Key::Left,"Right"=>Key::Right,
        "Up"=>Key::Up,"Down"=>Key::Down,"Backspace"=>Key::Backspace,"Enter"=>Key::Enter,"Space"=>Key::Space,
        "NumPad0"=>Key::NumPad0,"NumPad1"=>Key::NumPad1,"NumPad2"=>Key::NumPad2,"NumPad3"=>Key::NumPad3,
        "NumPad4"=>Key::NumPad4,"NumPad5"=>Key::NumPad5,"NumPad6"=>Key::NumPad6,"NumPad7"=>Key::NumPad7,
        "NumPad8"=>Key::NumPad8,"NumPad9"=>Key::NumPad9,"NumLock"=>Key::NumLock,"CapsLock"=>Key::CapsLock,
        "ScrollLock"=>Key::ScrollLock,"RightShift"=>Key::RightShift,"LeftShift"=>Key::LeftShift,
        "RightCtrl"=>Key::RightCtrl,"LeftCtrl"=>Key::LeftCtrl,"RightAlt"=>Key::RightAlt,"LeftAlt"=>Key::LeftAlt,
        "LeftSuper"=>Key::LeftSuper,"RightSuper"=>Key::RightSuper,
        "NumPadDot"=>Key::NumPadDot,"NumPadSlash"=>Key::NumPadSlash,"NumPadAsterisk"=>Key::NumPadAsterisk,
        "NumPadMinus"=>Key::NumPadMinus,"NumPadPlus"=>Key::NumPadPlus,"NumPadEnter"=>Key::NumPadEnter,
        "Apostrophe"=>Key::Apostrophe,"Comma"=>Key::Comma,"Period"=>Key::Period,"Slash"=>Key::Slash,
        "Semicolon"=>Key::Semicolon,"Equal"=>Key::Equal,"LeftBracket"=>Key::LeftBracket,"Backslash"=>Key::Backslash,
        "RightBracket"=>Key::RightBracket,"Minus"=>Key::Minus,"Backquote"=>Key::Backquote,
        "F13"=>Key::F13,"F14"=>Key::F14,"F15"=>Key::F15,"Tab"=>Key::Tab,"Pause"=>Key::Pause,
        "Menu"=>Key::Menu,"Unknown"=>Key::Unknown,
        _ => return None,
    })
}