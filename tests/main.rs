use std::collections::HashMap;
use urlcat::urlcat;

#[test]
fn concatenation() {
    let expected = "http://example.com/path";
    let actual = urlcat("http://example.com", "path", HashMap::new());
    assert_eq!(expected, actual);

    let expected = "http://example.com/path";
    let actual = urlcat("http://example.com/", "path", HashMap::new());
    assert_eq!(expected, actual);

    let expected = "http://example.com/path";
    let actual = urlcat("http://example.com", "path", HashMap::new());
    assert_eq!(expected, actual);

    let expected = "http://example.com/path";
    let actual = urlcat("http://example.com/", "/path", HashMap::new());
    assert_eq!(expected, actual);

    let expected = "http://example.com/path/1";
    let actual = urlcat(
        "http://example.com/",
        "/path/:p",
        HashMap::from([("p", "1".to_string())]),
    );
    assert_eq!(expected, actual);

    let expected = "http://example.com/path/1?q=2";
    let actual = urlcat(
        "http://example.com/",
        "/path/:p",
        HashMap::from([("p", "1".to_string()), ("q", "2".to_string())]),
    );
    assert_eq!(expected, actual);

    let expected = "http://example.com/path/a/b/a/r";
    let actual = urlcat(
        "http://example.com/",
        "/path/:p1/:p2/:p1/r",
        HashMap::from([("p1", "a".to_string()), ("p2", "b".to_string())]),
    );
    assert_eq!(expected, actual);

    let expected =
        "http://example.com/users/123/posts/987/comments?aa=aa&author_id=456&limit=10&offset=120";
    let actual = urlcat(
        "http://example.com/",
        "/users/:user_id/posts/:post_id/comments",
        HashMap::from([
            ("aa", "aa".to_string()),
            ("user_id", "123".to_string()),
            ("post_id", "987".to_string()),
            ("author_id", "456".to_string()),
            ("limit", "10".to_string()),
            ("offset", "120".to_string()),
        ]),
    );
    assert_eq!(expected, actual);

    let expected = "http://localhost:3000/path/test";
    let actual = urlcat(
        "http://localhost:3000/path/:p",
        "",
        HashMap::from([("p", "test".to_string())]),
    );
    assert_eq!(expected, actual);

    let expected = "/a";
    let actual = urlcat("/a", "", HashMap::new());
    assert_eq!(expected, actual);

    let expected = "/a/";
    let actual = urlcat("/a/", "", HashMap::new());
    assert_eq!(expected, actual);
}

#[test]
fn escape() {
    let expected = "http://example.com/path/a%20b?q=b%20c";
    let actual = urlcat(
        "http://example.com/",
        "/path/:p",
        HashMap::from([("p", "a b".to_string()), ("q", "b c".to_string())]),
    );
    assert_eq!(expected, actual);
}

#[test]
fn concat_edge_cases() {
    assert_eq!(
        urlcat("", "a", HashMap::new()),
        urlcat("a", "", HashMap::new())
    );

    let expected = "nice";
    let actual = urlcat("nice", "", HashMap::new());
    assert_eq!(expected, actual);

    let expected = "/nice";
    let actual = urlcat("/nice", "", HashMap::new());
    assert_eq!(expected, actual);

    let expected = "/nice/";
    let actual = urlcat("", "/nice/", HashMap::new());
    assert_eq!(expected, actual);

    let expected = "/nice/";
    let actual = urlcat("/nice/", "", HashMap::new());
    assert_eq!(expected, actual);
}

#[test]
fn empty_params_colon() {
    let url = "/:a/b/:c";

    assert_eq!(url, urlcat(url, "", HashMap::new()));
    assert_eq!(url.to_string() + url, urlcat(url, url, HashMap::new()));
    assert_eq!(url.to_string() + "/", urlcat(url, "/", HashMap::new()));
}
