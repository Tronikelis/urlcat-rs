#![allow(clippy::needless_return)]

use std::collections::HashMap;
use urlencoding::encode;

mod utils;
use utils::join_paths::join_paths;

pub fn urlcat(base: &str, path: &str, params: HashMap<&str, String>) -> String {
    let url_base = join_paths(base, path);

    let mut used_param_keys = vec![];

    if params.is_empty() {
        return url_base;
    }

    // overriding :xxx params
    let with_dynamic_params = url_base
        .split('/')
        .map(|x| {
            if x.chars().next().unwrap_or('_') != ':' {
                return x.to_string();
            }

            let without: String = x.chars().skip(1).collect();
            let param = params
                .get(&without as &str)
                .expect("missing param in the base url");

            used_param_keys.push(without);

            return encode(param).to_string();
        })
        .collect::<Vec<String>>()
        .join("/");

    let mut querystring = String::new();

    let mut filtered_params: Vec<(String, String)> = params
        .iter()
        .filter(|(key, _)| !used_param_keys.contains(&key.to_string()))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    filtered_params.sort_by(|a, b| {
        let a = a.0.to_lowercase();
        let b = b.0.to_lowercase();
        return a.cmp(&b);
    });

    for (i, (key, value)) in filtered_params.iter().enumerate() {
        let mut parameter = String::new();
        let ampersand: String = {
            if i == 0 {
                "".to_string()
            } else {
                "&".to_string()
            }
        };

        parameter.push_str(&ampersand);
        parameter.push_str(key);
        parameter.push('=');
        parameter.push_str(&encode(value));

        querystring.push_str(&parameter);
    }

    if querystring.is_empty() {
        return with_dynamic_params;
    }

    return with_dynamic_params + "?" + &querystring;
}
