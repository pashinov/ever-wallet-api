use anyhow::Result;

use crate::models::*;
use crate::sqlx_client::*;

impl SqlxClient {
    pub async fn get_callback(&self, service_id: ServiceId) -> Result<String> {
        sqlx::query!(
            r#"SELECT callback
                FROM api_service_callback
                WHERE service_id = $1"#,
            service_id as ServiceId,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(From::from)
        .map(|x| x.callback)
    }

    pub async fn set_callback(&self, payload: ApiServiceCallbackDb) -> Result<ApiServiceCallbackDb> {
        sqlx::query_as!(ApiServiceCallbackDb,
                r#"INSERT INTO api_service_callback
                (service_id, callback)
                VALUES ($1, $2)
                RETURNING
                id, service_id as "service_id: _", callback, created_at"#,
                payload.service_id as ServiceId,
                payload.callback,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }
}
