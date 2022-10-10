use yew::{function_component, html, Html, Properties, MouseEvent, Callback};
#[derive(Properties, PartialEq)]
pub struct TableClearProps {
    pub on_tbl_clear: Callback<Option<String>>,
}

#[function_component(TableClear)]
pub fn table_clear (props: &TableClearProps) -> Html {
    let onclick = {
        let on_tbl_clear = props.on_tbl_clear.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_tbl_clear.emit(None);
        })
    };


    html! {
        // <button class="btn btn-secondary" type="button" onclick={onclick} >{"Home"}</button>
        <li class="breadcrumb-item"><a href="#" onclick={onclick} >{"Home"}</a></li>
    }
}
