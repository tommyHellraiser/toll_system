#[macro_export]
macro_rules! panic_from_row {
    ($column: literal, $table: expr) => {
        std::panic::panic_any(
            format!("Coudln't find column name {} in table {}", $column, $table)
        )
    };
}

#[macro_export]
macro_rules! get_value_from_row {
    ($row: expr, $column: literal, $table: expr, $datatype: ty) => {
        if let Some(content) = $row.get::<$datatype, _>($column) {
            content
        } else {
            $crate::panic_from_row!($column, $table)
        }
    };
}