use reqwest;
use serde::Deserialize;

use std::error::Error;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub author: String,
    pub name: String,
    pub url: String,
    pub description: String,
    pub language: String,
    pub stars: String,
    pub forks: String,
    pub avatar: String,
    pub current_period_stars: String,
    #[serde(skip)]
    pub built_by: Vec<Person>,
}
#[derive(Deserialize, PartialEq, Debug)]
pub struct Person {
    pub href: String,
    pub avatar: String,
    pub username: String,
}
#[derive(Deserialize, PartialEq)]
pub struct ProgrammingLanguage {
    pub id: String,
    pub name: String,
}
#[derive(Deserialize, PartialEq)]
pub struct SpokenLanguage {
    pub url_param: String,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Developer {
    pub username: String,
    pub name: String,
    #[serde(rename = "type")]
    pub ttype: String,
    pub url: String,
    pub avatar: String,
    pub repo: Repo,
}
#[derive(Deserialize, PartialEq, Debug)]
pub struct Repo {
    pub repo_name: String,
    pub description: String,
}

const URL: &str = "https://hackertab.pupubird.com";

/// Fetch trending repositories on GitHub
///
/// # Arguments
///
/// * `language` - the programming language to filter by. eg: python
/// * `spoken_language_code` - the spoken language, eg: en for english
/// * `since` - one of the `["daily", "weekly", "monthly"]`. Defaults to "daily"
///
///  # Returns
///
///  * `Result<Vec<Repository>, Box<dyn Error>>` - the list of all trending repositories fetched or error if fetching fails.
pub async fn fetch_repos(
    language: Option<&str>,
    spoken_lang_code: Option<&str>,
    since: Option<&str>,
) -> Result<Vec<Repository>, Box<dyn Error>> {
    let mut query = String::from("/repositories?");
    if let Some(language) = language {
        // Check if the programming language is valid
        if check_language_validity(language).await? {
            query = format!("{}language={}", query, language);
        }
    } else {
        query = format!("{}language=", query);
    }
    if let Some(since) = since {
        // check if since is either "daily", "weekly", or "monthly"
        if check_since_validity(since)? {
            query = format!("{}&since={}", query, since);
        }
    } else {
        // since defaults to "daily"
        query = format!("{}&since=daily", query);
    }
    if let Some(spoken_lang_code) = spoken_lang_code {
        // check if spoken language is valid
        if check_spoken_lang_code_validity(spoken_lang_code).await? {
            query = format!("{}&spoken_lang_code={}", query, spoken_lang_code);
        }
    } else {
        query = format!("{}&spoken_lang_code=", query);
    }

    let full_url = format!("{}{}", URL, query);
    println!("here's final url:{}", full_url);
    let res = reqwest::get(&full_url)
        .await?
        .json::<Vec<Repository>>()
        .await?;
    return Ok(res);
}

/// Fetch trending devs on GitHub
///
/// # Arguments
///
///  * `language` - the programming language to filter by. eg: python
/// * `since` - one of the `["daily", "weekly", "monthly"]`. Defaults to "daily"
///
/// Returns
///
/// * `Result<Vec<Developer>, Box<dyn std::error::Error>>` - the list of trending developers or error if fetching fails.
pub async fn fetch_developers(
    language: Option<&str>,
    since: Option<&str>,
) -> Result<Vec<Developer>, Box<dyn std::error::Error>> {
    let mut query = String::from("/developers?");
    if let Some(language) = language {
        if check_language_validity(language).await? {
            // need formating before being pushed to the querry
            query = format!("{}language={}", query, language);
        }
    } else {
        query = format!("{}language=", query);
    }

    if let Some(since) = since {
        // Check since validity
        if check_since_validity(&since)? {
            query = format!("{}&since{}", query, since);
        }
    } else {
        query = format!("{}&sincedaily", query);
    }
    let full_url = format!("{}{}", URL, query);
    println!("{}", full_url);
    let res = reqwest::get(&full_url)
        .await?
        .json::<Vec<Developer>>()
        .await?;
    Ok(res)
}
/// Check if the language exists among valid languages.
///
/// # Arguments
///
///  * `language` - the programming language. eg: python
///
/// # Returns
///
/// * A boolena value. True for valid langugae, False otherwise.
async fn check_language_validity(language: &str) -> Result<bool, Box<dyn Error>> {
    // TODO: insteaf of fetchign these, i could just cache them
    let languages: Vec<ProgrammingLanguage> = languages_list().await?;
    let language = language.to_ascii_lowercase();

    for lang in languages {
        if language == lang.name.to_ascii_lowercase() {
            return Ok(true);
        }
    }
    return Ok(false);
}
/// Check if the spoken language exists.
///
/// # Arguments
///
/// * `lang_code` - the spoken language code. eg: en for English
///
/// # Returns
///
/// * `Result<bool, Box<dyn Error>>` `Ok(true)` for valid spoken language, `Ok(false)` otherwise.
async fn check_spoken_lang_code_validity(lang_code: &str) -> Result<bool, Box<dyn Error>> {
    let spoken_languages = spoken_languages_list().await?;
    let lang_code = lang_code.to_ascii_lowercase();

    for lang in spoken_languages {
        if lang_code == lang.name.to_ascii_lowercase() {
            return Ok(true);
        }
    }
    Ok(false)
}
/// Check if the time range value is correct
///
/// # Arguments
///
/// * `since` - the time range
///
/// # Returns
///
/// * `Result<bool, Box<dyn Error>>` - `Ok(true)` for valid `since`, `Ok(false)` otherwise
fn check_since_validity(since: &str) -> Result<bool, Box<dyn Error>> {
    Ok(["daily", "weekly", "monthly"].contains(&since))
}
/// Fetches programming languages from GitHub.
///
/// # Returns
///
/// * `Result<Vec<ProgrammingLanguage>, Box<dyn Error>>` - the list of programming languages
/// or error if fetching fails
async fn languages_list() -> Result<Vec<ProgrammingLanguage>, Box<dyn Error>> {
    let url = format!("{}/languages", URL);
    let response: Vec<ProgrammingLanguage> = reqwest::get(&url)
        .await?
        .json::<Vec<ProgrammingLanguage>>()
        .await?;

    Ok(response)
}
/// Fetch spoken languages from GitHub.
///
/// # Returns
///
/// * `Result<Vec<SpokenLanguage>, Box<dyn Error>>` - a list of spoken languages,
/// or error if fetching fails

async fn spoken_languages_list() -> Result<Vec<SpokenLanguage>, Box<dyn Error>> {
    let url = format!("{}/spoken_languages?", URL);
    let response: Vec<SpokenLanguage> = reqwest::get(&url)
        .await?
        .json::<Vec<SpokenLanguage>>()
        .await?;
    Ok(response)
}
