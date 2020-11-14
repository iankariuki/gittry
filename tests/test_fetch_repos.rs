use gtrending::fetch;
use std::error::Error;

fn basic_assertions(repos: Vec<fetch::Repository>) -> Result<(), Box<dyn Error>> {
    let repos = fetch::fetch_repos(None, None, None)?;
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
    let repos = fetch::fetch_repos(Some("python"), None, None)?;
    assert_eq!(basic_assertions(repos), Ok(()));
    let repos = fetch::fetch_repos(Some("rust"), None, None)?;
    assert_eq!(basic_assertions(repos), Ok(()));
    Ok(())
}
#[test]
// using dynamic errors might just be a temp fix. I think i need to implement my own errors. work on this next. like NisporError? right?
fn test_incorrect_values() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        fetch::fetch_repos(Some("not_a_language"), None, None)?,
        Box::new(std::io)
    );
}
