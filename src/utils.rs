use itertools::Itertools;

pub fn create_param_string(params: Option<Vec<(&str, &str)>>) -> String {
    params
        .unwrap_or(vec![])
        .into_iter()
        .map(|param| format!("{}={}", param.0, param.1))
        .collect_vec()
        .join("&")
}

pub fn create_api_error_message(code: u16, service: &str) -> String {
    match code {
        400 => format!("Got 400 Bad Request from {}. Please create issue on Github. Dependent API seems to have changed.", service),
        401 => format!("Got 401 Unauthorized from {}, please check the appropriate API key.", service),
        403 => format!("Got 403 Forbidden from {}, please check the appropriate API key.", service),
        404 => format!("Got 404 Not Found from {}. Please create issue on Github. Dependent API seems to have changed.", service),
        505 => format!("Got 505 internal server error from {}. Please try again later.", service),
        code => {
            format!(
                "Error {} returned from {}. Code unknown, please create issue on Github.",
                code, service
            )
        }
    }
}
