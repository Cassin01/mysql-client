use yew::{function_component, html, Html, Properties};
#[derive(Properties, PartialEq)]
pub struct DatabaseItemProps {
    pub id: usize,
    pub name: String,
}

#[function_component(DatabaseItem)]
pub fn database_item (props: &DatabaseItemProps) -> Html {
    html! {
        <li class="list-group-item">
        {&props.id} {". "} {&props.name}
        </li>
    }
}
