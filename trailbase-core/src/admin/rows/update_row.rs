use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::admin::AdminError as Error;
use crate::app_state::AppState;
use crate::records::json_to_sql::{simple_json_value_to_param, Params, UpdateQueryBuilder};

#[derive(Debug, Serialize, Deserialize, Default, TS)]
#[ts(export)]
pub struct UpdateRowRequest {
  pub primary_key_column: String,

  #[ts(type = "Object")]
  pub primary_key_value: serde_json::Value,

  /// This is expected to be a map from column name to value.
  ///
  /// Note that using an array here wouldn't make sense. The map allows for sparseness and only
  /// updating specific cells.
  #[ts(type = "{ [key: string]: Object | undefined }")]
  pub row: serde_json::Value,
}

pub async fn update_row_handler(
  State(state): State<AppState>,
  Path(table_name): Path<String>,
  Json(request): Json<UpdateRowRequest>,
) -> Result<(), Error> {
  let Some(table_metadata) = state.table_metadata().get(&table_name) else {
    return Err(Error::Precondition(format!("Table {table_name} not found")));
  };

  let pk_col = &request.primary_key_column;
  let Some((column, _col_meta)) = table_metadata.column_by_name(pk_col) else {
    return Err(Error::Precondition(format!("Missing column: {pk_col}")));
  };

  if !column.is_primary() {
    return Err(Error::Precondition(format!("Not a primary key: {pk_col}")));
  }

  UpdateQueryBuilder::run(
    &state,
    &table_metadata,
    Params::from(&table_metadata, request.row, None)?,
    &column.name,
    simple_json_value_to_param(column.data_type, request.primary_key_value)?,
  )
  .await?;

  return Ok(());
}
