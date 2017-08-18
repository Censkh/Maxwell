use super::Plugin;
use super::PluginPass;

pub struct PluginManager {
    plugins: Vec<Box<Plugin>>
}

impl PluginManager {
    pub fn new() -> Self {
        return PluginManager {
            plugins: Vec::new(),
        };
    }

    pub fn apply_plugin(&self, mut pass: PluginPass) {
        for plugin in &self.plugins {
            plugin.handle(&mut pass);
        }
    }

    pub fn add_plugin<T>(&mut self, plugin: T) where T: Plugin + 'static {
        self.plugins.push(Box::new(plugin));
    }
}
