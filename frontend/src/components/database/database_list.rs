use crate::components::database::database_item::DatabaseItem;
use crate::components::database::types::Database;
use yew::{function_component, html, Html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct DatabaseListProps {
    pub database_list: Vec<Database>,
    pub on_tbl_select: Callback<Option<String>>,
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
    let database_list = props.database_list.clone();
    let on_tbl_select = &props.on_tbl_select;
    html! {
        <ul class="list-group">
        {database_list.iter().map(|database| html! {
            <DatabaseItem id={database.id} name={database.name.clone()} on_tbl_select={on_tbl_select} />
        }).collect::<Html>()}
        </ul>
    }
}
