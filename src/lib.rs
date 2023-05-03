#![allow(clippy::needless_return)]

use std::collections::HashMap;
use urlencoding::encode;

pub fn urlcat(base: &str, path: &str, params: HashMap<&str, String>) -> String {
    // base + path
    let mut url_base = String::new();

    {
        let mut base = base.to_string();
        while base.chars().last().unwrap_or('_') == '/' {
            base.pop();
        }
        url_base.push_str(&base);
    }

    url_base.push('/');

    {
        let mut path = path.to_string();
        while path.chars().next().unwrap_or('_') == '/' {
            println!("{}", path);
            path = path.chars().skip(1).collect();
        }
        url_base.push_str(&path);
    }

    let mut used_param_keys = vec![];

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

    for (i, (key, value)) in params
        .iter()
        .filter(|(key, _)| !used_param_keys.contains(&key.to_string()))
        .enumerate()
    {
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

    return with_dynamic_params + "?" + &querystring;
}
