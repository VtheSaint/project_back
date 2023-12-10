use actix_web::web::Data;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Channel {
    pub channel_id: Option<Uuid>,
    pub name: Option<String>,
    pub users: Option<Vec<Uuid>>,
    pub img: Option<String>,
    pub last_message_id: Option<Uuid>,
    pub creator_id: Option<Uuid>
}

#[derive(Serialize)]
pub struct StructForGetChannels {
    pub username: Option<String>,
    pub message_body: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>
}

impl Channel {
    pub async fn get_messages(
        channel_id: Uuid,
        start_index: i32,
        end_index: i32,
        pool: Data<PgPool>
    ) -> Result<Vec<StructForGetChannels>, sqlx::Error> {
        sqlx::query_as!(
            StructForGetChannels,
            "SELECT * FROM get_messages_by_channel($1, $2, $3)",
            channel_id,
            start_index,
            end_index
        )
        .fetch_all(pool.as_ref())
        .await
    }


    pub async fn create(
        name: String,
        users: Vec<Uuid>,
        creator_id: Uuid,
        channel_id: Uuid,
        pool: Data<PgPool>,
        // img: Option<String>
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO channels VALUES ($1, null, $2, $3, null, $4)",
            channel_id,
            name,
            users.as_slice(),
            creator_id
        )
        .execute(pool.as_ref())
        .await
    }


    pub async fn join_user(
        users: &Vec<Uuid>,
        channel_id: &Uuid,
        pool: &Data<PgPool>
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE channels SET users = $1 WHERE channel_id = $2",
            users,
            channel_id
        )
        .execute(pool.as_ref())
        .await
    }


    pub async fn get_by_id(
        channel_id: &Uuid,
        pool: &Data<PgPool>,
    ) -> Result<Channel, sqlx::Error> {
        sqlx::query_as!(
            Channel,
            "SELECT * FROM channels WHERE channel_id = $1",
            channel_id
        )
        .fetch_one(pool.as_ref())
        .await
    }



}
