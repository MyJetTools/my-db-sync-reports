use std::{collections::BTreeMap, sync::Arc};

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput, WebContentType};

use crate::{app::AppContext, db::TableColumn};

use super::GetTableNameInput;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/table/{table_name}",
    input_data: "GetTableNameInput",
)]
pub struct GetTableAction {
    app: Arc<AppContext>,
}

impl GetTableAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetTableAction,
    input_data: GetTableNameInput,
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let mut html = String::new();

    let table_scema =
        crate::operations::get_table_schema(&action.app, &input_data.table_name).await;

    html.push_str(
        r###"<html><body><link rel="icon" type="image/x-icon" href="/img/{favicon_file_name}">
    <link href="/css/bootstrap.css" rel="stylesheet" type="text/css" />
    <link href="/css/site.css" rel="stylesheet" type="text/css" />
    <script src="/lib/jquery.js"></script>"###,
    );

    html.push_str(format!("<h1>Table: {}</h1>", input_data.table_name).as_str());

    html.push_str(r###"<table class="table"><tr><th>Column</th>"###);

    for (env, _) in action.app.postgress.iter() {
        html.push_str(&format!("<th>{}</th>", env));
    }
    html.push_str("</tr>");

    for (column_name, envs) in table_scema.columns {
        let difference = columns_are_different(&envs);

        if difference {
            html.push_str("<tr style=\"background:#ffdede\">");
        } else {
            html.push_str("<tr>");
        }

        html.push_str(format!("<td>{}</td>", column_name).as_str());

        for (env, _) in action.app.postgress.iter() {
            if let Some(env_column) = envs.get(env) {
                html.push_str(
                    format!(
                        "<td>{}:{} [nullable:{}]</td>",
                        env_column.column_name, env_column.data_type, env_column.is_nullable
                    )
                    .as_str(),
                );
            } else {
                html.push_str("<td style=\"color:red\">No</td>");
            }
        }
        html.push_str("</tr>");
    }

    html.push_str("</table>");

    // Inidexes
    html.push_str("<h2>Indexes</h2>");

    html.push_str(r###"<table class="table"><tr><th>Column</th>"###);

    for (env, _) in action.app.postgress.iter() {
        html.push_str(&format!("<th>{}</th>", env));
    }
    html.push_str("</tr>");

    for (index_name, envs) in table_scema.indexes {
        let difference = indexes_are_different(&envs);

        if difference {
            html.push_str("<tr style=\"background:#ffdede\">");
        } else {
            html.push_str("<tr>");
        }

        html.push_str(format!("<td>{}</td>", index_name).as_str());

        for (env, _) in action.app.postgress.iter() {
            if let Some(index_def) = envs.get(env) {
                html.push_str(format!("<td>{}</td>", index_def).as_str());
            } else {
                html.push_str("<td style=\"color:red\">No</td>");
            }
        }
        html.push_str("</tr>");
    }

    html.push_str("</table>");

    html.push_str("</body></html>");

    HttpOutput::Content {
        headers: None,
        content_type: Some(WebContentType::Html),
        content: html.into_bytes(),
    }
    .into_ok_result(false)
}

fn columns_are_different(envs: &BTreeMap<String, TableColumn>) -> bool {
    if envs.len() == 1 {
        return true;
    }

    let mut first = None;

    for column in envs.values() {
        if first.is_none() {
            first = Some(column);
            continue;
        }

        if let Some(column_info_first_name) = first {
            if column_info_first_name.has_difference_with(column) {
                return true;
            }
        }
    }

    false
}

fn indexes_are_different(envs: &BTreeMap<String, String>) -> bool {
    if envs.len() == 1 {
        return true;
    }

    let mut first = None;

    for index in envs.values() {
        if first.is_none() {
            first = Some(index);
            continue;
        }

        if let Some(index_firs_env) = first {
            if index_firs_env != index {
                return true;
            }
        }
    }

    false
}
