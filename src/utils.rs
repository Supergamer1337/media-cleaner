use itertools::Itertools;

pub fn create_param_string(params: Option<Vec<(&str, &str)>>) -> String {
    params
        .unwrap_or(vec![])
        .into_iter()
        .map(|param| format!("{}={}", param.0, param.1))
        .collect_vec()
        .join("&")
}
