use yew::{function_component, html, Html, Properties, MouseEvent, Callback};
#[derive(Properties, PartialEq)]
pub struct TableItemProps {
    pub name: String,
}

#[function_component(TableItem)]
pub fn table_item (props: &TableItemProps) -> Html {


    html! {
        <li class="list-group-item">
            {&props.name}
        </li>
    }
}
