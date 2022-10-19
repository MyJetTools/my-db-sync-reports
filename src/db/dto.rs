use my_postgres_macros::SelectDbEntity;

#[derive(SelectDbEntity, Clone)]
pub struct TableNameDto {
    pub table_name: String,
}

#[derive(SelectDbEntity, Clone)]
pub struct TableColumn {
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: String,
}

impl TableColumn {
    pub fn has_difference_with(&self, other: &TableColumn) -> bool {
        self.column_name != other.column_name
            || self.data_type != other.data_type
            || self.is_nullable != other.is_nullable
    }
}

#[derive(SelectDbEntity, Clone)]
pub struct TableIndex {
    pub indexname: String,
    pub indexdef: String,
}
