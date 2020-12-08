#`rustgtrending`

Lightweight and easy-to-use rust library for fetching
trending repositories and developers. Relies on
[github-trending-api](https://github.com/huchenme/github-trending-api)
which is in JavaScript, so gtrending aims to fill the gap
for rust.

## Simple Example
- # Fetch trending devs:

```rust 
use tokio;
use rustgtrending;
use std::error::Error;
/// Use tokio runtime to enable asynchronous coding.
/// Without a runtime, rust can't be able to call asynchronous functions
/// such as `rustgtrending::fetch_developers`
[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let language = "rust";
    /// since can be one of "daily", "weekly", or "monthly"
    let since = "daily";
    // Now we fetch trending developers from Github
    let devs = rustgtrending::fetch_developers(language, since).await?;
    println!("{:?}", devs);
    Ok(())
}
```
- # Fetch trending repositories:
```rust
use tokio;
use rustgtrending;
use std::error::Error;
[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let language = "rust";
    /// `since` can be one of "daily", "weekly", or "monthly"
    let since = "weekly";
    /// Let's use `en` for English.
    let spoken_language_code = "en";
    // Now we fetch trending repositories from Github
    let devs = rustgtrending::fetch_repos(language, spoken_language_code, since).await?;
    println!("{:?}", devs);
    Ok(())
}
```





