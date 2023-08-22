use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ItemsQueryData{
    pub page: u32,
    pub item_name: String,
    pub language: String,
    pub sort_by: String,
    pub sort_order: String,
    pub color: String,
    pub color_distance: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::utils::pool;
        use sqlx::{query_builder::QueryBuilder, Postgres, PgPool};

    struct ValidatedItemsQueryData {
        page: u32,
        item_name: String,
        language: String,
        sort_by: String,
        sort_order: String,
        color: (u8, u8, u8),
        color_distance: u32,
    }

    impl ItemsQueryData{
        fn validate(self) -> ValidatedItemsQueryData {
            let valid_page = self.page;
            // TODO: Validate item name if needed for db qyerry
            let valid_item_name = "%".to_string() + self.item_name.to_lowercase().as_str() + "%";

            let valid_language = match self.language.as_str() {
                "pl" => "pl".to_string(),
                "eng" | _ => "eng".to_string(),
            };

            let valid_sort_by = match self.sort_by.as_str(){
                "eng-name" => "display_name_eng".to_string(),
                "pl-name" => "display_name_pl".to_string(),
                "mc-id" => "minecraft_item_id".to_string(),
                // "color-distance" => "color-distance".to_string(),
                "default" | _ => "id".to_string(),
            }; 

            let valid_sort_order = match self.sort_order.as_str() {
                "A-Z" => "ASC".to_string(),
                "Z-A" | _ => "DESC".to_string()
            }; 

            let valid_color = (0,0,0);
            let valid_color_distance = self.color_distance.parse::<u32>().unwrap_or_default(); 

            ValidatedItemsQueryData { 
                page: valid_page, 
                item_name: valid_item_name, 
                language: valid_language, 
                sort_by: valid_sort_by, 
                sort_order: valid_sort_order, 
                color: valid_color, 
                color_distance: valid_color_distance 
            }
        }
    }

    impl ValidatedItemsQueryData{
        // TODO: NAPRAWIĆ TO ŻE PRZY WYSZUKiWANIU KONKRETNEJ NAZWY ITEMU KOLEJNE STRONY PSUJĄ WYNIKI NP> PRZY SZUKANIU ITEMU RESETOWAĆ PAGE W URL DO 0
        async fn query<'a>(self, pool: PgPool,  page_offset: i64) -> Result<Vec<Item>, ServerFnError> {
            let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM items WHERE ");

            query.push(" (LOWER(display_name_eng) LIKE ");
            query.push_bind(self.item_name.clone());

            query.push(" OR LOWER(display_name_pl) LIKE ");
            query.push_bind(self.item_name);
            query.push(" ) ");

            query.push(" ORDER BY ");
            query.push(self.sort_by.clone());
            
            if self.sort_by == "minecraft_item_id"{
                query.push("::INT ");
            }

            query.push(" ");
            query.push(self.sort_order);


            query.push(" LIMIT 100 OFFSET ");
            query.push_bind(page_offset);

            let mut items = Vec::new();
            let mut rows = query.build_query_as::<Item>().fetch(&pool);

            use futures::TryStreamExt;
            while let Some(row) = rows.try_next().await? {
                items.push(row);
            }
        
            Ok(items)
        }   
    }
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
    let valid_query_data = query_data.validate();

    valid_query_data.query(pool, page_offset).await
}
