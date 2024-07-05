use serde::Deserialize;

use crate::shortcut::{shortcut, KeyBind, Shortcut};

#[derive(Debug, Clone, Deserialize)]
pub struct Keyboard {
    #[serde(default = "KeyBind::move_up")]
    pub move_up: KeyBind,
    #[serde(default = "KeyBind::move_down")]
    pub move_down: KeyBind,
    #[serde(default = "KeyBind::move_left")]
    pub move_left: KeyBind,
    #[serde(default = "KeyBind::move_right")]
    pub move_right: KeyBind,
    #[serde(default = "KeyBind::close_buffer")]
    pub close_buffer: KeyBind,
    #[serde(default = "KeyBind::maximize_buffer")]
    pub maximize_buffer: KeyBind,
    #[serde(default = "KeyBind::restore_buffer")]
    pub restore_buffer: KeyBind,
    #[serde(default = "KeyBind::cycle_next_buffer")]
    pub cycle_next_buffer: KeyBind,
    #[serde(default = "KeyBind::cycle_previous_buffer")]
    pub cycle_previous_buffer: KeyBind,
    #[serde(default = "KeyBind::leave_buffer")]
    pub leave_buffer: KeyBind,
    #[serde(default = "KeyBind::toggle_nick_list")]
    pub toggle_nick_list: KeyBind,
    #[serde(default = "KeyBind::toggle_sidebar")]
    pub toggle_sidebar: KeyBind,
    #[serde(default = "KeyBind::command_bar")]
    pub command_bar: KeyBind,
    #[serde(default = "KeyBind::reload_configuration")]
    pub reload_configuration: KeyBind,
    #[serde(default = "KeyBind::toggle_configuration_buffer")]
    pub toggle_configuration_buffer: KeyBind,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            move_up: KeyBind::move_up(),
            move_down: KeyBind::move_down(),
            move_left: KeyBind::move_left(),
            move_right: KeyBind::move_right(),
            close_buffer: KeyBind::close_buffer(),
            maximize_buffer: KeyBind::maximize_buffer(),
            restore_buffer: KeyBind::restore_buffer(),
            cycle_next_buffer: KeyBind::cycle_next_buffer(),
            cycle_previous_buffer: KeyBind::cycle_previous_buffer(),
            leave_buffer: KeyBind::leave_buffer(),
            toggle_nick_list: KeyBind::toggle_nick_list(),
            toggle_sidebar: KeyBind::toggle_sidebar(),
            command_bar: KeyBind::command_bar(),
            reload_configuration: KeyBind::reload_configuration(),
            toggle_configuration_buffer: KeyBind::toggle_configuration_buffer(),
        }
    }
}

impl Keyboard {
    pub fn shortcuts(&self) -> Vec<Shortcut> {
        use crate::shortcut::Command::*;

        vec![
            shortcut(self.move_up.clone(), MoveUp),
            shortcut(self.move_down.clone(), MoveDown),
            shortcut(self.move_left.clone(), MoveLeft),
            shortcut(self.move_right.clone(), MoveRight),
            shortcut(self.close_buffer.clone(), CloseBuffer),
            shortcut(self.maximize_buffer.clone(), MaximizeBuffer),
            shortcut(self.restore_buffer.clone(), RestoreBuffer),
            shortcut(self.cycle_next_buffer.clone(), CycleNextBuffer),
            shortcut(self.cycle_previous_buffer.clone(), CyclePreviousBuffer),
            shortcut(self.leave_buffer.clone(), LeaveBuffer),
            shortcut(self.toggle_nick_list.clone(), ToggleNicklist),
            shortcut(self.toggle_sidebar.clone(), ToggleSidebar),
            shortcut(self.command_bar.clone(), CommandBar),
            shortcut(self.reload_configuration.clone(), ReloadConfiguration),
            shortcut(self.toggle_configuration_buffer.clone(), ToggleConfigurationBuffer),
        ]
    }
}
