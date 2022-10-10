use yew::{function_component, InputEvent, html, Html, use_state_eq, Callback, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct DatabaseFormProps {
    pub on_add: Callback<String>,
}

#[function_component(DatabaseForm)]
pub fn database_form(props: &DatabaseFormProps) -> Html {
    let tbl_name = use_state_eq(|| "".to_string());
    let oninput = {
        let tbl_name = tbl_name.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.data();
            match value {
                Some(value) => {
                    tbl_name.set((*tbl_name).clone() + &value);
                }
                None => {
                    tbl_name.set("".to_string());
                }
            }
        })
    };
    let onclick = {
        let on_add = props.on_add.clone();
        let tbl_name = tbl_name.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            tbl_name.set("".to_string());
            on_add.emit((*tbl_name).clone());
            // log::info!("tbl_name: {:?}", *tbl_name);
        })
    };
    html! {
        <form class="mb-2">
            // <div class="mb-1">
            <div class="mb-1 row">
                <div class="col-auto">
                <label for="title" class="col-form-label">{"Table name"}</label>
                </div>
                <div class="col-auto">
                <input type="text" value={(*tbl_name).clone()} oninput={oninput} class="form-control" id="tbl_name" />
                </div>
                <div class="col-auto">
                <button type="submit" onclick={onclick} class="btn btn-secondary">{"Add"}</button>
                </div>
            </div>
            // <div class="mb-3">
            //     {&*tbl_name}
            // </div>
        </form>
    }
}
