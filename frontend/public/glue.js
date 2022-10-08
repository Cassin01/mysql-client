const invoke = window.__TAURI__.invoke

export async function invokeHello(name) {
  return await invoke("hello", {name: name})
}

export async function invokeConnected(url) {
    return await invoke("connected", {url: url})
}

export async function invokeShowTables(url) {
  return await invoke("show_tables", {url: url})
}

export async function invokeLoadDatasource() {
  return await invoke("load_datasource", {})
}
