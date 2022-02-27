/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use chrono::{DateTime, Datelike, Local, Timelike, Utc};

pub trait ThemeContext {
    fn get_var(&self, var_name: &str) -> &str;
    fn from_sub_context<T: SubContext>(&mut self, contributor: &T);
    fn extend(&mut self, other: &Self);
}

///Contributing to a context will require a struct to implement this trait
pub trait SubContext {
    fn retrieve_var(&self) -> HashMap<String, String>;
}

///A context is a hashmap holding a variable name & a value for this variable.
pub struct Context {
    data: HashMap<String, String>,
}

impl ThemeContext for Context {
    fn get_var(&self, var_name: &str) -> &str {
        self.data.get(var_name).unwrap()
    }

    fn from_sub_context<T: SubContext>(&mut self, contributor: &T) {
        self.data.extend(contributor.retrieve_var());
    }

    fn extend(&mut self, other: &Self) {
        self.data.extend(other.data.clone());
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    #[cfg(test)]
    pub fn set_var(&mut self, var_name: &str, value: &str) {
        self.data.insert(var_name.to_string(), value.to_string());
    }
}

pub struct DirectoryContext {
    current_dir: String,
}

impl DirectoryContext {
    pub fn new(current_dir: String) -> Self {
        Self { current_dir }
    }
}

impl SubContext for DirectoryContext {
    fn retrieve_var(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("current_dir".to_string(), self.current_dir.clone());

        vars
    }
}

pub struct SessionContext {
    desktop_env: String,
    real_name: String,
    username: String,
    device_name: String,
    hostname: String,
    platform: String,
    distribution: String,
}

impl SessionContext {
    ///Create from current env
    pub fn new() -> Self {
        Self {
            desktop_env: whoami::desktop_env().to_string(),
            real_name: whoami::realname(),
            username: whoami::username(),
            device_name: whoami::devicename(),
            hostname: whoami::hostname(),
            platform: whoami::platform().to_string(),
            distribution: whoami::distro(),
        }
    }
}

impl SubContext for SessionContext {
    fn retrieve_var(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("desktop_env".to_string(), self.desktop_env.clone());
        vars.insert("real_name".to_string(), self.real_name.clone());
        vars.insert("username".to_string(), self.username.clone());
        vars.insert("device_name".to_string(), self.device_name.clone());
        vars.insert("hostname".to_string(), self.hostname.clone());
        vars.insert("platform".to_string(), self.platform.clone());
        vars.insert("distribution".to_string(), self.distribution.clone());

        vars
    }
}

impl SubContext for DateTime<Utc> {
    fn retrieve_var(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        let d = self.naive_utc();
        let time = d.time();
        let date = d.date();
        vars.insert("utc_ss".to_string(), time.second().to_string());
        vars.insert("utc_mm".to_string(), time.minute().to_string());
        vars.insert("utc_hh12".to_string(), time.hour12().1.to_string());
        vars.insert(
            "utc_AMPM".to_string(),
            if time.hour12().0 {
                "PM".to_string()
            } else {
                "AM".to_string()
            },
        );
        vars.insert("utc_hh24".to_string(), time.hour().to_string());
        vars.insert("utc_DD".to_string(), date.day().to_string());
        vars.insert("utc_MM".to_string(), date.month().to_string());
        vars.insert("utc_YYYY".to_string(), date.year().to_string());
        vars.insert("utc_ord".to_string(), date.ordinal().to_string());
        vars.insert("utc_wday".to_string(), date.weekday().to_string());

        vars
    }
}

impl SubContext for DateTime<Local> {
    fn retrieve_var(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        let d = self.naive_local();
        let time = d.time();
        let date = d.date();
        vars.insert("loc_ss".to_string(), time.second().to_string());
        vars.insert("loc_mm".to_string(), time.minute().to_string());
        vars.insert("loc_hh12".to_string(), time.hour12().1.to_string());
        vars.insert(
            "loc_AMPM".to_string(),
            if time.hour12().0 {
                "PM".to_string()
            } else {
                "AM".to_string()
            },
        );
        vars.insert("loc_hh24".to_string(), time.hour().to_string());
        vars.insert("loc_DD".to_string(), date.day().to_string());
        vars.insert("loc_MM".to_string(), date.month().to_string());
        vars.insert("loc_YYYY".to_string(), date.year().to_string());
        vars.insert("loc_ord".to_string(), date.ordinal().to_string());
        vars.insert("loc_wday".to_string(), date.weekday().to_string());

        vars
    }
}
