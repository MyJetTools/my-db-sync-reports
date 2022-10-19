use std::{collections::BTreeMap, sync::Arc};

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput, WebContentType};

use crate::{app::AppContext, db::TableColumn};

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/",
)]
pub struct IndexAction {
    app: Arc<AppContext>,
}

impl IndexAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &IndexAction,
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let mut html = String::new();

    let tables = crate::operations::get_tables_difference(&action.app).await;

    if let Err(err) = tables {
        return HttpOutput::as_text(err).into_ok_result(false);
    }

    let tables = tables.unwrap();

    html.push_str(
        r###"<html><body><link rel="icon" type="image/x-icon" href="/img/{favicon_file_name}">
    <link href="/css/bootstrap.css" rel="stylesheet" type="text/css" />
    <link href="/css/site.css" rel="stylesheet" type="text/css" />
    <script src="/lib/jquery.js"></script>
    <table class="table"><tr><th>TableName</th>"###,
    );

    for (env, _) in action.app.postgress.iter() {
        html.push_str(&format!("<th>{}</th><th>Columns</th><th>Indexes</th>", env));
    }
    html.push_str("</tr>");

    for (table_name, table_info) in tables {
        let mut has_difference = columns_are_different(
            &action.app.postgress.first().unwrap().0,
            &table_info.columns,
        );

        if !has_difference {
            has_difference = indexes_are_different(
                &action.app.postgress.first().unwrap().0,
                &table_info.indexes,
            )
        }

        if has_difference {
            html.push_str("<tr style=\"background:#ffdede\">");
        } else {
            html.push_str("<tr>");
        }

        html.push_str(
            format!(
                "<td><a href=\"/table/{}\">{}</a></td>",
                table_name, table_name
            )
            .as_str(),
        );

        for (env, _) in action.app.postgress.iter() {
            if let Some(columns) = table_info.columns.get(env) {
                html.push_str("<td style=\"color:green\">Yes</td>");

                html.push_str(format!("<td>{}</td>", columns.len()).as_str());
            } else {
                html.push_str("<td style=\"color:red\">No</td><td>--</td>");
            }

            if let Some(indexes) = table_info.indexes.get(env) {
                if indexes.len() == 0 {
                    html.push_str(
                        format!(
                            "<td style=\"background:red;color:white\"><b>{}</b></td>",
                            indexes.len()
                        )
                        .as_str(),
                    );
                } else {
                    html.push_str(format!("<td>{}</td>", indexes.len()).as_str());
                }
            } else {
                html.push_str("<td>--</td>");
            }
        }
        html.push_str("</tr>");
    }

    html.push_str("</table></body></html>");

    HttpOutput::Content {
        headers: None,
        content_type: Some(WebContentType::Html),
        content: html.into_bytes(),
    }
    .into_ok_result(false)
}

fn columns_are_different(
    first_env_id: &str,
    envs: &BTreeMap<String, BTreeMap<String, TableColumn>>,
) -> bool {
    if envs.len() == 1 {
        return true;
    }

    let first_env = envs.get(first_env_id);

    if first_env.is_none() {
        return true;
    }

    let first_env = first_env.unwrap();

    for (env_name, columns) in envs {
        if env_name == first_env_id {
            continue;
        }

        for (column_name, column_info) in columns {
            if let Some(column_info_first_name) = first_env.get(column_name) {
                if column_info_first_name.has_difference_with(column_info) {
                    return true;
                }
            } else {
                return true;
            }
        }
    }

    false
}

fn indexes_are_different(
    first_env_id: &str,
    envs: &BTreeMap<String, BTreeMap<String, String>>,
) -> bool {
    if envs.len() == 1 {
        return true;
    }

    let first_env = envs.get(first_env_id);

    if first_env.is_none() {
        return true;
    }

    let first_env = first_env.unwrap();

    for (env_name, columns) in envs {
        if env_name == first_env_id {
            continue;
        }

        for (column_name, column_info) in columns {
            if let Some(column_info_first_name) = first_env.get(column_name) {
                if column_info_first_name != column_info {
                    return true;
                }
            } else {
                return true;
            }
        }
    }

    false
}
