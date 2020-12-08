#`rustgtrending`

Lightweight and easy-to-use rust library for fetching
trending repositories and developers. Relies on
[github-trending-api](https://github.com/huchenme/github-trending-api)
which is in JavaScript, so gtrending aims to fill the gap
for rust.

### Simple Example

```rust 
use tokio;
use rustgtrending;
use std::error::Error;
[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let language = "rust";
    let since = matches.value_of("since");
    // Now we fetch trending developers from Github
    let devs = gtrending::fetch_developers(language, since).await?;
    println!("{:?}", devs);
    Ok(())
}

