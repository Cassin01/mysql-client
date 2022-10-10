use yew::{function_component, html, Html, Properties, MouseEvent, Callback};
#[derive(Properties, PartialEq)]
pub struct DatabaseItemProps {
    pub id: usize,
    pub name: String,
    pub on_tbl_select: Callback<Option<String>>,
}

#[function_component(DatabaseItem)]
pub fn database_item (props: &DatabaseItemProps) -> Html {
    let onclick = {
        let on_tbl_select = props.on_tbl_select.clone();
        let name = props.name.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_tbl_select.emit(Some(name.clone()));
        })
    };


    html! {
        <li class="list-group-item">
            <a onclick={onclick}>
            {&props.id} {". "} {&props.name}
            </a>
        </li>
    }
}
