//! Profile configuration and management.

use serde::{Deserialize, Serialize};
use crate::protocol::{
    buttons::ButtonConfig,
    constants::PollingRate,
    power::PowerConfig,
    sensor::SensorConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub sensor: SensorConfig,
    pub power: PowerConfig,
    pub polling_rate: PollingRate,
    pub buttons: ButtonConfig,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            id: "profile_1".into(),
            name: "Profile 1".into(),
            sensor: SensorConfig::default(),
            power: PowerConfig::default(),
            polling_rate: PollingRate::Hz1000,
            buttons: ButtonConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileManager {
    pub profiles: Vec<Profile>,
    pub active_index: usize,
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self {
            profiles: vec![
                Profile {
                    id: "profile_1".into(),
                    name: "Profile 1".into(),
                    ..Default::default()
                },
                Profile {
                    id: "profile_2".into(),
                    name: "Profile 2 (Gaming)".into(),
                    ..Default::default()
                },
            ],
            active_index: 0,
        }
    }
}

impl ProfileManager {
    pub fn active_profile(&self) -> &Profile {
        &self.profiles[self.active_index.min(self.profiles.len().saturating_sub(1))]
    }

    pub fn active_profile_mut(&mut self) -> &mut Profile {
        let idx = self.active_index.min(self.profiles.len().saturating_sub(1));
        &mut self.profiles[idx]
    }

    pub fn add_profile(&mut self, name: String) -> usize {
        let id = format!("profile_{}", self.profiles.len() + 1);
        let profile = Profile {
            id,
            name,
            ..Default::default()
        };
        self.profiles.push(profile);
        self.active_index = self.profiles.len() - 1;
        self.active_index
    }

    pub fn delete_profile(&mut self, index: usize) -> bool {
        if self.profiles.len() <= 1 || index >= self.profiles.len() {
            return false;
        }
        self.profiles.remove(index);
        if self.active_index >= self.profiles.len() {
            self.active_index = self.profiles.len() - 1;
        }
        true
    }

    pub fn rename_profile(&mut self, index: usize, new_name: String) {
        if let Some(p) = self.profiles.get_mut(index) {
            p.name = new_name;
        }
    }

    pub fn reset_active(&mut self) {
        let name = self.active_profile().name.clone();
        let id = self.active_profile().id.clone();
        *self.active_profile_mut() = Profile {
            id,
            name,
            ..Default::default()
        };
    }
}
