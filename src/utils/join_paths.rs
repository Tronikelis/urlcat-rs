fn last_char(string: &String) -> Option<char> {
    return string.chars().last();
}

pub fn join_paths(a: &str, b: &str) -> String {
    let filtered = [a, b];
    let filtered: Vec<_> = filtered.iter().filter(|x| !x.is_empty()).collect();

    if filtered.len() == 1 {
        return filtered[0].to_string();
    }

    let mut url_base = String::new();

    {
        let mut a = a.to_string();
        while a.chars().last().unwrap_or('_') == '/' {
            a.pop();
        }
        url_base.push_str(&a);
    }

    url_base.push('/');

    {
        let mut b = b.to_string();
        while b.chars().next().unwrap_or('_') == '/' {
            b = b.chars().skip(1).collect();
        }
        url_base.push_str(&b);
    }

    let a_b: String = a.to_string() + b;
    // if the last part does not have "/" then the url should not have it either
    while last_char(&a_b).unwrap_or('_') != '/' && last_char(&url_base).unwrap_or('_') == '/' {
        url_base.pop();
    }

    return url_base;
}
