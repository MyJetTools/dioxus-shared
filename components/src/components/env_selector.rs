use std::rc::Rc;

use dioxus::prelude::*;

#[component]
pub fn EnvSelector(envs: Vec<Rc<String>>, on_change: EventHandler<String>) -> Element {
    let mut selected_env = use_signal(|| None::<Rc<String>>);

    let selected_env_value = selected_env.read().clone();
    let my_eval = if selected_env_value.is_none() {
        let my_eval = eval(
            r#"
            const env = localStorage.getItem("selectedEnv");
            if (env) {
                dioxus.send(env);
            }else{
                dioxus.send("");
            }
            "#,
        );

        let mut eval_owned = my_eval.to_owned();
        spawn(async move {
            let value = eval_owned.recv().await.unwrap();

            match value {
                serde_json::Value::String(value) => {
                    on_change.call(value.clone());
                    *selected_env.write() = Some(value.into());
                }

                _ => {
                    panic!("Somehow we got non string value");
                }
            }
        });

        my_eval
    } else {
        eval(
            r#"    while(true){
               let msg = await dioxus.recv();
               localStorage.setItem("selectedEnv", msg);
            }"#,
        )
    };

    if selected_env_value.is_none() {
        return rsx! {
            div { "Loading selected env..." }
        };
    }

    let selected_env_value = selected_env_value.unwrap();

    let items = envs.into_iter().map(|itm| {
        if selected_env_value.as_str() == itm.as_str() {
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
            h4 { style: "color:white;text-shadow: 1px 1px 1px #747474;", "MyNoSqlServer" }
            select {
                class: "form-control",

                style: "background-color: #2c2c2c;color: white;border-color: black;",
                onchange: move |e| {
                    let value = e.value();
                    let value_spawn = value.clone();
                    let _ = my_eval.send(value_spawn.into());
                    if selected_env_value.as_str() == value.as_str() {
                        return;
                    }
                    on_change.call(e.value());
                },
                {items}
            }
        }
    }
}
