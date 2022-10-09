use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ConnectionProps {
    pub connection: String
}

#[function_component(Connection)]
pub fn header(props: &ConnectionProps) -> Html {
    html! {
        if props.connection == "true" {
            <div class="led-green"></div>
        } else {
            <div class="led-red"></div>
        }
    }
}
