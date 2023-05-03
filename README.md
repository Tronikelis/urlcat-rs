<h1 align="center">urlcat</h1>

<h3 align="center">Inspired by <a href="https://github.com/balazsbotond/urlcat">urlcat</a></h3>

## What is this?

This package [urlbat](https://github.com/Tronikelis/urlbat) rewritten in rust

## Simple example showing what it can do

```rust
use std::collections::HashMap;
use urlcat::urlcat;

fn main() {
    let url = urlcat(
        "https://example.com/",
        "path/:crab/resource",
        HashMap::from([
            ("pretty", "cool/+/& yep".to_string()),
            ("really", "cool/+/& probably".to_string()),
            ("crab", "🦀".to_string()),
        ]);
    );
    assert_eq!(
        url,
        "https://example.com/path/🦀 well actually (%F0%9F%A6%80)/resource?pretty=cool%2F%2B%2F%26+yep&really=cool%2F%2B%2F%26+probably"
    );
}
```
