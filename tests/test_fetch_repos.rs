use gtrending::fetch;
use mockito;
use std::error::Error;
use tokio::runtime::Runtime;

const URL: &'static str = &mockito::server_url();

#[test]

fn test_fetch() -> Result<(), Box<dyn Error>> {
    let _mt = mockito::mock("GET", "/python")
        .with_status(200)
        .with_body("")
        .create();
    let language_list_mock = mockito::mock("GET", "/languages")
        .with_status(200)
        .with_header("content_type", "application/json")
        .with_body_from_file("./language_list-mock.json")
        .create();
    let full_url = format!(
        "{}/repositories?language=&since=&spoken_language_code=",
        URL
    );
    let repos_mock = mockito::mock("GET", &full_url[..])
        .with_status(200)
        .with_header("content_type", "application/json")
        .with_body_from_file("./repos_list_mock.json");

    let mut run_time = Runtime::new()?;
    //make requests
    let future_repos = fetch::fetch_repos(None, None, None);
    // not readable but cool and short:)
    let repos_list: Vec<fetch::Repository> =
        serde_json::from_slice(&std::fs::read("./repos_list_mock.json")?)?;
    let repos = run_time.block_on(future_repos)?;
    assert_eq!(repos_list, repos);
    Ok(())
}

async fn basic_assertions(
    lang: Option<&str>,
    spoken: Option<&str>,
    since: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let repos = fetch::fetch_repos(lang, spoken, since).await?;
    for repo in repos {
        assert_eq!(
            repo.url,
            format!("https://github.com/{}/{}", repo.author, repo.name)
        );
    }
    Ok(())
}
#[test]
fn test_language() -> Result<(), Box<dyn Error>> {
    let repos = basic_assertions(Some("python"), None, None);

    assert_eq!(repos, Ok(()));
    let repos = basic_assertions(Some("rust"), None, None);
    assert_eq!(repos, Ok(()));
    Ok(())
}
#[test]
// using dynamic errors might just be a temp fix. I think i need to implement my own errors. work on this next. like NisporError? right?
fn test_incorrect_values() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        basic_assertions(Some("invalid-language"), None, None),
        Box::new(std::io::Error {})
    );
    Ok(())
}
