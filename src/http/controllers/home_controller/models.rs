use my_http_server_swagger::MyHttpInput;

#[derive(Debug, MyHttpInput)]
pub struct GetTableNameInput {
    #[http_path(description = "Table name")]
    pub table_name: String,
}
