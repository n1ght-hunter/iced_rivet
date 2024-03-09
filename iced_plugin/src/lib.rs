use libloading::library_filename;
use std::{any::Any, collections::HashMap, ffi::OsStr, marker::PhantomData, ops::Deref, sync::Arc};

#[derive(Clone, Debug)]
pub struct MessageType(Arc<Box<dyn Any + Send + Sync>>);

impl MessageType {
    pub fn new<T: 'static + Send + Sync>(message: T) -> Self {
        Self(Arc::new(Box::new(message)))
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.0.deref().downcast_ref()
    }

    pub fn downcast<T: 'static>(self) -> Result<T, ()> {
        let value = Arc::try_unwrap(self.0).map_err(|_| ())?;
        value.downcast::<T>().map_err(|_| ()).map(|value| *value)
    }
}

pub trait Plugin: Send + Sync {
    fn view(&self) -> iced::Element<'_, MessageType>;

    fn update(&mut self, message: MessageType) -> iced::Command<MessageType>;

    fn subscription(&self) -> Option<iced::Subscription<MessageType>> {
        None
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// DO NOT OVERRIDE THIS METHOD
    ///
    /// This method is used to get the version of the plugin.
    fn version(&self) -> &'static str {
        "1.0.0"
    }
}

pub type PluginFunction = Box<dyn Plugin>;
type CreatePlugin = fn() -> PluginFunction;

pub trait PluginLoader {
    fn plugin_message(id: u16, message: MessageType) -> Self;
}

pub struct PluginHandler<T: 'static + PluginLoader> {
    plugins: HashMap<u16, (libloading::Library, Box<dyn Plugin>)>,
    message: PhantomData<T>,
    counter: u16,
}

impl<T: 'static + PluginLoader> PluginHandler<T> {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            message: PhantomData,
            counter: 0,
        }
    }

    pub fn load<P: AsRef<OsStr>>(&mut self, path: P) -> Result<(), libloading::Error> {
        let lib = unsafe { libloading::Library::new(library_filename(path))? };
        let func = unsafe { lib.get::<CreatePlugin>(b"new_plugin")? };
        let plugin = func();
        self.plugins.insert(self.counter, (lib, plugin));
        self.counter += 1;
        Ok(())
    }

    pub fn unload(&mut self, id: u16) {
        if let Some((lib, _)) = self.plugins.remove(&id) {
            drop(lib);
        }
    }

    pub fn plugin_update(&mut self, id: u16, message: MessageType) -> iced::Command<T> {
        let update = if let Some((_, plugin)) = self.plugins.get_mut(&id) {
            plugin.update(message)
        } else {
            iced::Command::none()
        };

        update.map(move |message| T::plugin_message(id, message))
    }

    pub fn plugin_view(&self, id: u16) -> Option<iced::Element<'_, T>> {
        self.plugins
            .get(&id)
            .map(|(_, plugin)| plugin.view())
            .map(move |view| view.map(move |message| T::plugin_message(id, message)))
    }

    pub fn plugin_info(&self) -> Vec<(u16, String)> {
        self.plugins
            .iter()
            .map(|(id, (_, plugin))| (id.clone(), plugin.name().to_string()))
            .collect()
    }

    pub fn drop_libs(&mut self) {
        for (_, (lib, _)) in self.plugins.drain() {
            lib.close().unwrap();
        }
    }
}

impl<T: 'static + PluginLoader> Drop for PluginHandler<T> {
    fn drop(&mut self) {
        self.drop_libs();
    }
}
