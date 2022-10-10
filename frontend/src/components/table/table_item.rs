use yew::{function_component, html, Callback, Html, MouseEvent, Properties};
#[derive(Properties, PartialEq)]
pub struct TableItemProps {
    pub name: Vec<String>,
}

#[function_component(TableItem)]
pub fn table_item(props: &TableItemProps) -> Html {
    let names = props.name.clone();
    html! {
        <li class="list-group-item">
            <div class="row">
            // {&props.name}
            {names.iter().map(|name| html! {
                <div class="col ">
                {name}
                </div>
            }).collect::<Html>()}
            </div>
        </li>
    }
}
