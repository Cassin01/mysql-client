use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
mod components;
use components::connection::Connection;
use components::database::database_form::DatabaseForm;
use components::database::database_list::DatabaseList;
use components::database::types::Database;
use components::header::Header;
use components::table::table_clear::TableClear;
use components::table::table_list::TableList;
use components::table::types::Table;

macro_rules! post_inc {
    ($i:ident) => {{
        let old = $i;
        $i += 1;
        old
    }};
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
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
    #[wasm_bindgen(js_name = invokeShowItems, catch)]
    pub async fn show_items(url: String, tblname: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = invokeAddTable, catch)]
    pub async fn add_table(url: String, tblname: String) -> Result<JsValue, JsValue>;
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
    // let connect = use_state_eq(|| "404".to_string());
    let connect = use_state_eq(|| "404".to_string());
    let tables = use_state_eq(|| wasm_bindgen::JsValue::from_str("hoge"));
    let items = use_state_eq(|| wasm_bindgen::JsValue::from_str("hoge"));
    // ユーザのテーブル状態を保持する
    let tbl = use_state_eq(|| None::<String>);
    {
        let url = url.clone();
        update_url_state(url);
    }

    // Execute tauri command via effects.
    // The effect will run every time `name` changes.
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
            move |connect| {
                update_tables_state(tables, connect.clone());
                || ()
            },
            (*connect).clone(),
        );
    }
    {
        let items = items.clone();
        let connect = connect.clone();
        use_effect_with_deps(
            move |tbl| {
                update_items_state(items, connect.to_string(), tbl.clone());
                || ()
            },
            (*tbl).clone(),
        );
    }

    let message = (*welcome).clone();
    let m_connect = (*connect).clone();
    let m_tbl = (*tbl).clone();
    let ts = (*tables).clone();
    let is = (*items).clone();
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
    let tlist: Vec<Table>;
    if let Ok(tbl_list) = is.into_serde::<Vec<Vec<String>>>() {
        tlist = tbl_list
            .into_iter()
            .map(|name| Table { name: name.clone() })
            .collect();
    } else {
        tlist = vec![];
    }

    let on_add = {
        Callback::from(move |tbl_name: String| {
            // add_table(url.clone().to_string(), tbl_name);
            // log::info!("on_add: {:?}", tbl_name);
            call_add_table(connect.clone().to_string(), tbl_name);
            {
                let tables = tables.clone();
                update_tables_state(tables, connect.to_string());
            }
        })
    };
    let on_tbl_select = {
        Callback::from(move |tbl_name: Option<String>| {
            let tbl = tbl.clone();
            update_tbl_state(tbl, tbl_name);
        })
    };

    html! {
        <div>
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container">
                <Header />
                <Connection connection={m_connect} />
                </div>
                </nav>
            </nav>

            if let Some(name) = m_tbl {
                    <nav aria-label="breadcrumb" class="navbar-light bg-light pt-1">
                        <div class="container-fluid">
                    <ol class="breadcrumb">
                    // <li class="breadcrumb-item"><a href="#">{"Home"}</a></li>
                    <TableClear on_tbl_clear={on_tbl_select} />
                    <li class="breadcrumb-item active" aria-current="page">{name}</li>
                    </ol>
                    </div>
                    </nav>
                <main class="container-fluid mt-2">
                    // <div class="mb-1 row justify-content-between">
                    //     <div class="col-auto">
                    //     {name}
                    //     </div>
                    //     <div class="col-auto ml-auto">

                    //     <TableClear on_tbl_clear={on_tbl_select} />
                    //     </div>
                    // </div>
                    <TableList table_list={tlist} />
                </main>
            } else {
                    <nav aria-label="breadcrumb" class="navbar-light bg-light pt-1">
                        <div class="container-fluid">
                    <ol class="breadcrumb">
                    <li class="breadcrumb-item"><a href="#">{"Home"}</a></li>
                    // <li class="breadcrumb-item active" aria-current="page">{name}</li>
                    </ol>
                        </div>
                    </nav>
                <main class="container-fluid mt-2">
                    <DatabaseForm {on_add} />
                    <DatabaseList database_list={dlist} on_tbl_select={on_tbl_select} />
                    </main>
            }
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
                state.set("404".to_string());
                // let window = window().unwrap();
                // window
                //     .alert_with_message(&format!("Error: {:?}", e))
                //     .unwrap();
            }
        }
    })
}

fn update_tables_state(state: UseStateHandle<wasm_bindgen::JsValue>, connect: String) {
    spawn_local(async move {
        match show_tables(connect).await {
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

fn update_items_state(state: UseStateHandle<wasm_bindgen::JsValue>, connect: String, tbl: Option<String>) {
    if let Some(tbl_name) = tbl {
        spawn_local(async move {
            match show_items(connect, tbl_name).await {
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
}

fn update_url_state(state: UseStateHandle<String>) {
    spawn_local(async move {
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

fn call_add_table(url: String, tbl_name: String) {
    spawn_local(async move {
        match add_table(url, tbl_name).await {
            Ok(message) => {
                log::info!("on_add message: {:?}", message);
            }
            Err(e) => {
                log::warn!("on_add message: {:?}", e);
                let window = window().unwrap();
                window
                    .alert_with_message(&format!("Error: {:?}", e))
                    .unwrap();
            }
        }
    })
}

fn update_tbl_state(state: UseStateHandle<Option<String>>, tbl: Option<String>) {
    spawn_local(async move {
        state.set(tbl);
    })
}
