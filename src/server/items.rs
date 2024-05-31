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
    pub color_search: bool,
    pub color: String,
    pub color_distance: String,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::utils::pool;
    use sqlx::{query_builder::QueryBuilder, Postgres, PgPool};
    
    #[allow(dead_code)]
    struct ValidatedItemsQueryData {
        page: u32,
        item_name: String,
        language: String,
        sort_by: String,
        sort_order: String,
        color_search: bool,
        color: (f64, f64, f64),
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
                "eng-name" => "items.display_name_eng".to_string(),
                "pl-name" => "items.display_name_pl".to_string(),
                "mc-id" => "items.minecraft_item_id".to_string(),
                // TODO: Dodać wyświetlanie tej opcji przy filtrowaniu kolorami
                // "color-distance" => "color-distance".to_string(),
                "default" | _ => "items.id".to_string(),
            }; 

            let valid_sort_order = match self.sort_order.as_str() {
                "Z-A" => "DESC".to_string()
                "A-Z" | _ => "ASC".to_string(),
            }; 
            let valid_color_search = self.color_search;

            use colors_transform::{Rgb, Color};
            let parsed_color = Rgb::from_hex_str(self.color.as_str()).unwrap_or(Rgb::from(0.0, 0.0, 0.0));
            let valid_color = (parsed_color.get_red() as f64, parsed_color.get_green() as f64, parsed_color.get_blue() as f64);
            let valid_color_distance = self.color_distance.parse::<u32>().unwrap_or_default(); 

            ValidatedItemsQueryData { 
                page: valid_page, 
                item_name: valid_item_name, 
                language: valid_language, 
                sort_by: valid_sort_by, 
                sort_order: valid_sort_order,
                color_search: valid_color_search,
                color: valid_color, 
                color_distance: valid_color_distance 
            }
        }
    }
// SELECT items.id, items.display_name_eng, SUM(colors.color_index) 
// FROM items INNER JOIN colors ON items.id = colors.item_id 
// GROUP BY items.id
// ORDER BY items.id
// LIMIT 100
    impl ValidatedItemsQueryData{
        async fn query(self, pool: PgPool) -> Result<Vec<Item>, ServerFnError> {
            // let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM items WHERE ");
            let mut query: QueryBuilder<Postgres> = QueryBuilder::new("");

            query.push("SELECT items.id, items.item_name, items.display_name_eng, items.display_name_pl, items.item_meta, items.minecraft_item_id, items.has_nbt, items.filename, COALESCE(SUM(colors.color_index),0) AS color_similiarity ");
            query.push("FROM items INNER JOIN colors ON items.id = colors.item_id WHERE ");


            query.push(" (LOWER(items.display_name_eng) LIKE ");
            query.push_bind(self.item_name.clone());

            query.push(" OR LOWER(items.display_name_pl) LIKE ");
            query.push_bind(self.item_name);
            query.push(" ) ");

            if self.color_search {
                query.push(" AND colors.color <-> cube(array[");
                query.push_bind(self.color.0);
                query.push(",");
                query.push_bind(self.color.1);
                query.push(",");
                query.push_bind(self.color.2);
                query.push("]) < ");
                query.push_bind(self.color_distance as f64);
            }

            query.push(" GROUP BY items.id ");

            query.push(" ORDER BY ");
            query.push(self.sort_by.clone());
            
            if self.sort_by == "items.minecraft_item_id"{
                query.push("::INT ");
            }

            query.push(" ");
            query.push(self.sort_order);
            
            query.push(" LIMIT 100 OFFSET ");
            query.push_bind((self.page*100) as i64);

            let mut items = Vec::with_capacity(100);
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
    pub color_similiarity: i64
}

#[server(GetItems, "/api")]
pub async fn get_items(query_data: ItemsQueryData) -> Result<Vec<Item>, ServerFnError> {
    let pool = pool()?;

    // let page_offset = (query_data.page * 100) as i64;
    let valid_query_data = query_data.validate();

    valid_query_data.query(pool).await
}
