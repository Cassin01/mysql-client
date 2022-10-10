#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use std::sync::Mutex;
// use tokio::sync::Mutex;
use tauri::State;
mod conf;
mod mysql;
pub use crate::mysql::utils as sql;
pub use crate::mysql::utils::connect;
pub use crate::mysql::utils::Pool;

#[tauri::command]
fn load_datasource() -> Result<String, String> {
    // This is a very simplistic example but it shows how to return a Result
    // and use it in the front-end.
    let cnf = conf::load_source();
    match cnf {
        Ok(url) => Ok(url),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
    // This is a very simplistic example but it shows how to return a Result
    // and use it in the front-end.
    if name.contains(' ') {
        Err("Name should not contain spaces".to_string())
    } else {
        Ok(format!("mysql, {}", name))
    }
}

#[tauri::command]
async fn connected(url: &str, pool: State<'_, Pool>) -> Result<String, String> {
    // This is a very simplistic example but it shows how to return a Result
    // and use it in the front-end.

    match connect(url).await {
        Ok(p) => {
            let mut pool = pool.0.lock().await;
            *pool = Some(p);
            Ok("true".to_string())
        }
        Err(_) => Err("connection failed".to_string()),
    }
}

#[tauri::command]
async fn add_table(url: &str, tblname: &str, pool: State<'_, Pool>) -> Result<Vec<String>, String> {
    // println!("add_table {}", tblname);
    {
        let pool = pool.0.lock().await;
        match pool.clone() {
            Some(p) => match sql::add_table(&p.clone(), tblname).await {
                Ok(tables) => return Ok(tables),
                Err(e) => return Err(e.to_string()),
            },
            None => (),
        }
    }
    match connect(url).await {
        Ok(p) => {
            let mut pool = pool.0.lock().await;
            *pool = Some(p.clone());
            match sql::add_table(&p, tblname).await {
                Ok(tables) => return Ok(tables),
                Err(e) => return Err(e.to_string()),
            }
        }
        Err(_) => Err("connection failed".to_string()),
    }
}

#[tauri::command]
async fn show_tables(url: &str, pool: State<'_, Pool>) -> Result<Vec<String>, String> {
    {
        let pool = pool.0.lock().await;
        match pool.clone() {
            Some(p) => match sql::show_tables(&p.clone()).await {
                Ok(tables) => return Ok(tables),
                Err(e) => return Err(e.to_string()),
            },
            None => (), // None => Err("not connect".to_string()),
        }
    }
    match connect(url).await {
        Ok(p) => {
            let mut pool = pool.0.lock().await;
            *pool = Some(p.clone());
            match sql::show_tables(&p).await {
                Ok(tables) => return Ok(tables),
                Err(e) => return Err(e.to_string()),
            }
        }
        Err(_) => Err("connection failed".to_string()),
    }
}

#[tauri::command]
async fn show_items(
    url: &str,
    tblname: &str,
    pool: State<'_, Pool>,
) -> Result<Vec<Vec<String>>, String> {
    {
        let pool = pool.0.lock().await;
        match pool.clone() {
            Some(p) => match sql::show_items(&p, tblname).await {
                Ok(tables) => return Ok(tables),
                Err(e) => return Err(e.to_string()),
            },
            None => (), // None => Err("not connect".to_string()),
        }
    }
    match connect(url).await {
        Ok(p) => {
            let mut pool = pool.0.lock().await;
            *pool = Some(p.clone());
            match sql::show_items(&p, tblname).await {
                Ok(tables) => return Ok(tables),
                Err(e) => return Err(e.to_string()),
            }
        }
        Err(_) => Err("connection failed".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Pool(Default::default()))
        .invoke_handler(tauri::generate_handler![
            hello,
            connected,
            show_tables,
            show_items,
            add_table,
            load_datasource
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
