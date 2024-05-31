use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};


// #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
// pub struct ItemOffer {
//     pub author: String,
//     pub item: i64,
//     pub title: String,
//     pub description: String,
//     pub items_per_packet: i64,
//     pub packets: i64,
//     pub price: OfferPrices,
//     pub date: String
// }

// #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
// pub struct OfferPrices{
//     pub yen: Option<i64>,
//     pub rubel: Option<i64>,
//     pub lira: Option<i64>,
//     pub peso_chile: Option<i64>,
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct SimpleItemOffer {
    pub author: String,
    pub item: i64,
    pub title: String,
    pub description: String,
    pub items_per_packet: i64,
    pub packets: i64,
    pub price_per_item: f64,
    pub date: String
}

#[derive(Clone, PartialEq, Debug,  Serialize, Deserialize)]
pub struct ItemOffersQueryData{
    pub item_id: i64,
    pub page: u32,
    pub author_username: String,
    pub sort_by: String,
    pub sort_order: String,
    pub currency: String,
    pub max_price_per_item: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
    use crate::utils::pool;
    use sqlx::{query_builder::QueryBuilder, Postgres, PgPool};

    #[allow(dead_code)]
    struct ValidatedItemOffersQueryData {
        item_id: i64,
        page: u32,
        author_username: String,
        sort_by: String,
        sort_order: String,
        currency: String,
        max_price_per_item: f64,
    }

    impl ItemOffersQueryData{
        fn validate(self) -> ValidatedItemOffersQueryData{
            // TODO validate querry data
            todo!()
        }
    }
    impl ValidatedItemOffersQueryData {
        /// Querry builder and executor for item offers
        /// 
        /// **EXAMPLE QUERRY BUILDER OUTPUT**
        /// ```sql
        /// SELECT 
        ///     users.username as author, 
        ///     items.display_name_eng as item, 
        ///     item_offers.title as title, 
        ///     item_offers.description as description, 
        ///     item_offers.packets as packets,
        ///     cast(item_offers.items_per_packet as double precision) / cast(item_offers_prices.price as double precision) as price_per_item
        /// FROM (((item_offers_prices 
        ///     INNER JOIN currencies ON item_offers_prices.currency_id = currencies.id) 
        ///     INNER JOIN item_offers ON item_offers_prices.offer_id = item_offers.id) 
        ///     INNER JOIN users ON item_offers.author_id = users.id) 
        ///     INNER JOIN items ON item_offers.item_id = items.id
        /// WHERE
        ///     users.username LIKE '%'
        ///     AND cast(item_offers.items_per_packet as double precision) / cast(item_offers_prices.price as double precision) < 10.0
        ///     AND currencies.name = 'yen'
        /// ORDER BY 
        ///     price_per_item DESC;
        /// LIMIT 100 OFFSET 100
        /// ```
        async fn query(self, pool: PgPool) -> Result<Vec<SimpleItemOffer>, ServerFnError>{
            let mut query: QueryBuilder<Postgres> = QueryBuilder::new("");

            query.push("SELECT users.username as author, users.username as author, items.display_name_eng as item, item_offers.title as title, item_offers.description as description, item_offers.packets as packets, cast(item_offers.items_per_packet as double precision) / cast(item_offers_prices.price as double precision) as price_per_item ");
            query.push("FROM (((item_offers_prices INNER JOIN currencies ON item_offers_prices.currency_id = currencies.id) INNER JOIN item_offers ON item_offers_prices.offer_id = item_offers.id) INNER JOIN users ON item_offers.author_id = users.id) INNER JOIN items ON item_offers.item_id = items.id ");
            query.push(" WHERE ");

            query.push(" (LOWER(users.username) LIKE ");
            query.push_bind(self.author_username);

            query.push(" AND currencies.name LIKE ");
            query.push_bind(self.currency);

            query.push(" AND cast(item_offers.items_per_packet as double precision) / cast(item_offers_prices.price as double precision) < ");
            query.push(self.max_price_per_item);

            query.push(" ORDER BY ");
            query.push(self.sort_by);

            query.push(" ");
            query.push(self.sort_order);
            
            query.push(" LIMIT 100 OFFSET ");
            query.push_bind((self.page*100) as i64);

            let mut item_offers = Vec::with_capacity(100);
            let mut rows = query.build_query_as::<SimpleItemOffer>().fetch(&pool);

            use futures::TryStreamExt;

            while let Some(row) = rows.try_next().await? {
                item_offers.push(row);
            }
        
            Ok(item_offers)
        }
    }
}}


#[server(GetItemOffers, "/api")]
pub async fn get_item_offers(query_data: ItemOffersQueryData) -> Result<Vec<SimpleItemOffer>, ServerFnError> {
    let pool = pool()?;
    let valid_query_data = query_data.validate();

    valid_query_data.query(pool).await
}