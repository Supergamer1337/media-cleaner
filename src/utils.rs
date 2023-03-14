use itertools::Itertools;

pub fn create_param_string(params: Option<Vec<(&str, &str)>>) -> String {
    params
        .unwrap_or(vec![])
        .into_iter()
        .map(|param| format!("{}={}", param.0, param.1))
        .collect_vec()
        .join("&")
}

pub fn create_api_error_message(code: u16, path: &str, service: &str) -> String {
    match code {
        400 => format!("Got 400 Bad Request from {} at {}. The api may have changed, please report this on Github.", service, path),
        401 => format!("Got 401 Unauthorized from {}, please check the appropriate API key.", service),
        403 => format!("Got 403 Forbidden from {}, please check the appropriate API key.", service),
        404 => format!("Got 404 Not Found from {} at path {}. Please make sure the URl is correct.", service, path),
        505 => format!("Got 505 internal server error from {}. Please try again later.", service),
        code => {
            format!(
                "Error {} returned from {}. Code unknown, please create issue on Github.",
                code, service
            )
        }
    }
}

pub fn human_file_size(size: i64) -> String {
    let gig_size = 1000000000.0;
    let gigs: f64 = size as f64 / gig_size;
    format!("{:.2}GB", gigs)
}
