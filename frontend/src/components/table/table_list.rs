use crate::components::table::table_item::TableItem;
use crate::components::table::types::Table;
use yew::{function_component, html, Html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct TableListProps {
    pub table_list: Vec<Table>,
}

#[function_component(TableList)]
pub fn table_list(props: &TableListProps) -> Html {
    let table_list = props.table_list.clone();
    html! {
        <ul class="list-group">
        {table_list.iter().map(|table| html! {
            <TableItem name={table.name.clone()} />
        }).collect::<Html>()}
        </ul>
    }

}
