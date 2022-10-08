use crate::components::database::database_item::DatabaseItem;
use crate::components::database::types::Database;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DatabaseListProps {
    pub database_list: Vec<Database>
}

#[function_component(DatabaseList)]
pub fn database_list(props: &DatabaseListProps) -> Html {
    // let database_items = vec![
    //     Database {
    //         id: 1,
    //         name: "pass".to_string(),
    //     },
    //     Database {
    //         id: 2,
    //         name: "diary".to_string(),
    //     },
    //     Database {
    //         id: 3,
    //         name: "notes".to_string(),
    //     },

    // ];
    html! {
        <ul class="list-group">
        {props.database_list.iter().map(|database| html! {
            <DatabaseItem id={database.id} name={database.name.clone()} />
        }).collect::<Html>()}
        </ul>
    }
}
