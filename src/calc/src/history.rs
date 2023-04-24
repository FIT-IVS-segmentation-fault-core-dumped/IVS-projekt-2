use druid::{Data, WindowId};
use serde::{Deserialize, Serialize};

/// Contains data about all calculations as well as essential informations
/// about status of the History window
#[derive(Serialize, Deserialize, Clone)]
pub struct History {
    /// calculation history
    pub data: Vec<(String, String)>,
    /// indicates whether history recording is enabled or not
    record_history: bool,

    #[serde(default, skip)]
    is_opened: bool,
    #[serde(default, skip)]
    window_id: Vec<WindowId>,
    #[serde(default, skip)]
    pub confiming_deletition: bool,
}

impl Data for History {
    fn same(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Default for History {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            record_history: true,
            is_opened: false,
            window_id: Vec::new(),
            confiming_deletition: false,
        }
    }
}

impl History {
    // Disable or enable history recording
    pub fn toggle_recording(&mut self) {
        self.record_history = !self.record_history;
    }

    /// Information about whether history is enabled or not
    pub fn recording(&self) -> bool {
        self.record_history
    }

    /// Is history window opened?
    pub fn is_opened(&self) -> bool {
        self.is_opened
    }

    /// Perform essential setups when opening the history window
    pub fn open_history(&mut self, win_id: WindowId) {
        self.window_id.push(win_id);
        self.is_opened = true;
    }

    /// Restore history initial settings
    pub fn close_history(&mut self) {
        self.window_id.pop();
        self.is_opened = false;
    }

    /// Return id of the history window.
    pub fn get_win_id(&self) -> Option<&WindowId> {
        self.window_id.get(0)
    }

    /// Get history data
    pub fn get_data(&self) -> &Vec<(String, String)> {
        &self.data
    }

    /// Clear all history data
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
