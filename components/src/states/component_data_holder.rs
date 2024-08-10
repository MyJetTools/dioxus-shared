use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub enum ComponentDataHolder<T: Clone> {
    Initialized,
    Loading,
    Loaded(Result<T, ServerFnError>),
}

impl<T: Clone> ComponentDataHolder<T> {
    pub fn new() -> Self {
        Self::Initialized
    }

    pub fn reset(&mut self) {
        *self = Self::Initialized;
    }

    pub fn set_loading(&mut self) {
        *self = Self::Loading;
    }

    pub fn set_value(&mut self, value: Result<T, ServerFnError>) {
        *self = Self::Loaded(value)
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
    pub fn unwrap_mut<'s>(&'s mut self) -> &'s mut T {
        match self {
            Self::Loaded(Ok(value)) => value,
            _ => panic!("DataModel is not loaded"),
        }
    }

    pub fn unwrap(&self) -> &T {
        match self {
            Self::Loaded(Ok(value)) => value,
            _ => panic!("DataModel is not loaded"),
        }
    }

    pub fn value_as_option_ref(&self) -> Option<&T> {
        match self {
            Self::Loaded(value) => match value {
                Ok(value) => Some(value),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn value_as_option_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Loaded(value) => match value {
                Ok(value) => Some(value),
                _ => None,
            },
            _ => None,
        }
    }
}
