use sqlx::{any::AnyRow, postgres::any::AnyColumn, Column, Row, TypeInfo};

pub struct SqlParser;

impl SqlParser {
    // See following links : 
    // https://docs.rs/sqlx/latest/sqlx/mysql/types/index.html
    // https://docs.rs/sqlx/latest/sqlx/sqlite/types/index.html
    // https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html
    pub fn convert_from_sqlx_row(col: &AnyColumn, row: &AnyRow) -> String {
        let row_type = col.type_info().name();
        let ord = col.ordinal();
        match row_type {
            // mysql types
            // sqlx doesnt implement decode i8/uX for any
            "TINYINT" | "BOOL" | "SMALLINT" | "TINYINT UNSIGNED" | "SMALLINT UNSIGNED" => {
                return row.get::<i16, _>(ord).to_string()
            }
            "INT" | "INT UNSIGNED" => return row.get::<i32, _>(ord).to_string(),
            "BIGINT" | "BIGINT UNSIGNED" => return row.get::<i64, _>(ord).to_string(),
            "FLOAT" => return row.get::<f32, _>(ord).to_string(),
            "DOUBLE" => return row.get::<f64, _>(ord).to_string(),
            "VARCHAR" | "CHAR" | "TEXT" | "INET4" | "INET6" => return row.get::<_, _>(ord),
            "VARBINARY" | "BINARY" | "BLOB" => return "BLOB...".to_string(),

            //sqlite types
            "BOOLEAN" => return row.get::<bool, _>(ord).to_string(),
            "INTEGER" => return row.get::<i32, _>(ord).to_string(),
            "INT8" => return row.get::<i64, _>(ord).to_string(),
            "REAL" => return row.get::<f64, _>(ord).to_string(),

            // postgres types
            "SMALLSERIAL" | "INT2" => return row.get::<i16, _>(ord).to_string(),
            "SERIAL" | "INT4" => return row.get::<i32, _>(ord).to_string(),
            "BIGSERIAL" => return row.get::<i64, _>(ord).to_string(),
            "FLOAT4" => return row.get::<f32, _>(ord).to_string(),
            "DOUBLE PRECISION" | "FLOAT8" => return row.get::<f64, _>(ord).to_string(),
            "BYTEA" => return "BLOB...".to_string(),

            _ => return "UNIMPLEMENTED".to_string(),
        }
    }
}
