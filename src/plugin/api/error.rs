use std::sync::Arc;

use plugin::api::modules::PluginId;

#[derive(Debug, PartialEq)]
pub enum PluginManagerError {
    ModuleNotFound(Arc<str>),
    PluginNotFound(PluginId),
    PluginNotStarted(PluginId),
    PluginAlreadyStarted(PluginId),
    CustomError(Arc<str>)
}

pub struct ErrorLog {
    errors: Vec<PluginManagerError>,
    warnings: Vec<PluginManagerError>
}

impl ErrorLog {
    pub fn add_error(&mut self, error: PluginManagerError) {
        if(self.errors.contains(&error)) {
            return;
        }

        self.errors.push(error);
    }

    pub fn add_warning(&mut self, error: PluginManagerError) {
        if(self.warnings.contains(&error)) {
            return;
        }

        self.warnings.push(error);
    }

    pub fn get_errors(&self) -> &Vec<PluginManagerError> {
        return &self.errors;
    }

    pub fn get_warnings(&self) -> &Vec<PluginManagerError> {
        return &self.warnings;
    }

    pub fn is_failed(&self) -> bool {
        return !self.errors.is_empty();
    }

    pub fn is_completed(&self) -> bool {
        return !self.is_failed();
    }

    pub fn is_no_alerts(&self) -> bool {
        return self.errors.is_empty() && self.warnings.is_empty();
    }

    pub fn new() -> ErrorLog {
        return ErrorLog{
            errors: Vec::new(),
            warnings: Vec::new()
        }
    }
}
