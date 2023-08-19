use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::pages::items_list_page::ItemsQueryData;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::PgPool;
        use crate::utils::pool;
}}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Item {
    pub id: i64,
    pub item_name: String,
    pub minecraft_item_id: String,
    pub item_meta: String,
    pub has_nbt: bool,
    pub display_name_eng: String,
    pub display_name_pl: String,
    pub filename: String,
}

#[server(GetItems, "/api")]
pub async fn get_items(cx: Scope, query_data: ItemsQueryData) -> Result<Vec<Item>, ServerFnError> {
    let pool = pool(cx)?;

    let page_offset = (query_data.page * 100) as i64;

    let mut items = Vec::new();
    let mut rows = sqlx::query_as!(
        Item,
        r#"
        SELECT * FROM items LIMIT 100 OFFSET $1
        "#,
        page_offset
    ).fetch(&pool);
    
    use futures::TryStreamExt;
    while let Some(row) = rows.try_next().await? {
        items.push(row);
    }

    Ok(items)
}
