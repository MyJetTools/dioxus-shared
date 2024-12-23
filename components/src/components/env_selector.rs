use std::rc::Rc;

use dioxus::prelude::*;

#[component]
pub fn EnvSelector(envs: Vec<Rc<String>>, on_change: EventHandler<String>) -> Element {
    let mut selected_env = use_signal(|| SelectedEnv::new(envs));

    let selected_env_read_access = selected_env.read();

    let items = selected_env_read_access.envs.iter().map(|itm| {
        if selected_env_read_access.as_str() == itm.as_str() {
            return rsx! {
                option { selected: true, "{itm}" }
            };
        }

        rsx! {
            option { "{itm}" }
        }
    });

    rsx! {
        div { style: "position: sticky; top: 0; background-color: var(--left-panel-color);",
            select {
                class: "form-control",

                style: "background-color: #2c2c2c;color: white;border-color: black;",
                onchange: move |e| {
                    let value = e.value();
                    selected_env.write().set_value(value.as_str());
                    on_change.call(e.value());
                },
                {items}
            }
        }
    }
}

pub struct SelectedEnv {
    envs: Vec<Rc<String>>,
    value: Rc<String>,
}

impl SelectedEnv {
    pub fn new(envs: Vec<Rc<String>>) -> Self {
        let value = dioxus_utils::js::GlobalAppSettings::get_local_storage()
            .get("selectedEnv")
            .unwrap_or_default();

        let index = envs.iter().position(|v| v.as_str() == value.as_str());

        let value = match index {
            Some(index) => envs.get(index).unwrap().clone(),

            None => {
                if envs.len() > 0 {
                    envs.get(0).unwrap().clone()
                } else {
                    Rc::new(value)
                }
            }
        };

        Self { value, envs }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn set_value(&mut self, value: &str) {
        dioxus_utils::js::GlobalAppSettings::get_local_storage().set("selectedEnv", value);

        let index = self.envs.iter().position(|v| v.as_str() == value).unwrap();

        self.value = self.envs.get(index).unwrap().clone();
    }
}
