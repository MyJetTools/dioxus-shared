use std::rc::Rc;

use dioxus_utils::DataState;

pub struct EnvListState {
    items: DataState<Vec<Rc<String>>>,
    selected_env: Option<Rc<String>>,
}

impl EnvListState {
    pub fn new() -> Self {
        Self {
            items: DataState::None,
            selected_env: None,
        }
    }

    pub fn get_items(&self) -> &DataState<Vec<Rc<String>>> {
        &self.items
    }

    pub fn has_envs(&self) -> bool {
        self.items.is_loading()
    }
    pub fn get_selected_env(&self) -> Option<Rc<String>> {
        self.selected_env.clone()
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        let items: Vec<Rc<String>> = items.into_iter().map(|itm| Rc::new(itm)).collect();
        self.items = DataState::Loaded(items);
    }

    pub fn set_active_env(&mut self, selected_env: String) {
        if self.items.is_none() {
            panic!("Should net set active env before envs are loaded");
        }

        if let Some(items) = self.items.try_unwrap_as_loaded() {
            if items.len() == 0 {
                return;
            }

            let index = items
                .iter()
                .position(|itm| itm.as_str() == selected_env.as_str());

            match index {
                Some(index) => {
                    self.selected_env = Some(items[index].clone());
                }
                None => {
                    self.selected_env = items.first().cloned();
                }
            }
        }
    }
}
