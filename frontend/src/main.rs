use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
mod components;
use components::database::database_list::DatabaseList;
use components::database::types::Database;

macro_rules! post_inc {
    ($i:ident) => {{
        let old = $i;
        $i += 1;
        old
    }};
}

fn main() {
    yew::start_app::<App>();
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = invokeConnected, catch)]
    pub async fn connected(url: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = invokeShowTables, catch)]
    pub async fn show_tables(url: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = invokeLoadDatasource, catch)]
    pub async fn load_datasource() -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {
    let welcome = use_state_eq(|| "".to_string());
    let url = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "client".to_string());
    let connect = use_state_eq(|| "404".to_string());
    let tables = use_state_eq(|| wasm_bindgen::JsValue::from_str("hoge"));

    // Execute tauri command via effects.
    // The effect will run every time `name` changes.
    {
        let url = url.clone();
        update_url_state(url);
    }
    {
        let welcome = welcome.clone();
        use_effect_with_deps(
            move |name| {
                update_welcome_message(welcome, name.clone());
                || ()
            },
            (*name).clone(),
        );
    }
    {
        let connect = connect.clone();
        use_effect_with_deps(
            move |url| {
                update_connected_state(connect, url.clone());
                || ()
            },
            (*url).clone(),
        );
    }
    {
        let tables = tables.clone();
        use_effect_with_deps(
            move |url| {
                update_tables_state(tables, url.clone());
                || ()
            },
            (*url).clone(),
        );
    }

    let message = (*welcome).clone();
    let m_connect = (*connect).clone();
    let ts = (*tables).clone();
    use gloo_utils::format::JsValueSerdeExt;
    let dlist: Vec<Database>;
    if let Ok(database_list) = ts.into_serde::<Vec<String>>() {
        let mut cnt = 1;
        dlist = database_list
            .into_iter()
            .map(|name| Database {
                id: post_inc!(cnt),
                name: name.clone(),
            })
            .collect();
    } else {
        dlist = vec![];
    }

    html! {
        <div>
            <h2 class={"heading"}>{message}</h2>
            <p>{"Connection: "}{m_connect}</p>

            <h3>{"Table List"}</h3>
            <DatabaseList database_list={dlist}/>
         </div>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        match hello(name).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap());
            }
            Err(e) => {
                // let window = window().unwrap();
                // window
                //     .alert_with_message(&format!("Error: {:?}", e))
                //     .unwrap();
            }
        }
    });
}

fn update_connected_state(state: UseStateHandle<String>, url: String) {
    spawn_local(async move {
        match connected(url).await {
            Ok(message) => {
                state.set(message.as_string().unwrap());
            }
            Err(e) => {
                // let window = window().unwrap();
                // window
                //     .alert_with_message(&format!("Error: {:?}", e))
                //     .unwrap();
            }
        }
    })
}

fn update_tables_state(state: UseStateHandle<wasm_bindgen::JsValue>, url: String) {
    spawn_local(async move {
        match show_tables(url).await {
            Ok(message) => {
                state.set(message);
            }
            Err(e) => {
                // let window = window().unwrap();
                // window
                //     .alert_with_message(&format!("Error: {:?}", e))
                //     .unwrap();
            }
        }
    })
}

fn update_url_state(state: UseStateHandle<String>) {
    spawn_local (async move {
        match load_datasource().await {
            Ok(url) => {
                state.set(url.as_string().unwrap());
            }
            Err(e) => {
                // let window = window().unwrap();
                // window
                //     .alert_with_message(&format!("error: source could not loaded: {:?}", e))
                //     .unwrap();
            }
        }
    })
}
