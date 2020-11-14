use reqwest;
use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub author: String,
    pub name: String,
    pub avatar: String,
    pub url: String,
    pub description: String,
    pub language: String,
    pub language_color: u32,
    pub stars: u32,
    pub forks: u32,
    pub current_period_stars: u32,
    pub built_by: Vec<Person>,
}
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub href: String,
    pub avatar: String,
    pub username: String,
}
#[derive(Serialize, Deserialize)]
pub struct ProgrammingLanguage {
    pub id: String,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct SpokenLanguage {
    pub url_param: String,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Developer {
    pub username: String,
    pub name: String,
    pub ttype: String,
    pub url: String,
    pub avatar: String,
    pub repo: Repo,
}
#[derive(Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
    pub description: String,
    pub url: String,
}

const URL: &str = "https://ghapi.huchen.dev";

//TODO: I need to make this function async. which means, i might need to bring in a runtime. actix or tokio.

// Fetch trending repositories on GitHub

//Parameters:
//  language (str, optional):  Filtering by language, eg: python
//   spoken_language_code (str, optional): The spoken language, eg: en for english
//  since (str, optional): The time range, choose from: [daily, weekly, monthly]. Defaults to "daily"

//Returns:
//    A list of dicts containing information for the trending repositories found
#[tokio::main]
pub async fn fetch_repos(
    language: Option<&str>,
    spoken_lang_code: Option<&str>,
    since: Option<&str>,
) -> Result<Vec<Repository>, Box<dyn Error>> {
    let mut query = String::from("/repositories?");
    if let Some(language) = language {
        if check_language_validity(language).await? {
            // need formating before being pushed to the querry
            query.push_str(language);
        }
    }
    if let Some(spoken_lang_code) = spoken_lang_code {
        if check_spoken_lang_code_validity(spoken_lang_code).await? {
            //needs formatting
            query.push_str(spoken_lang_code);
        }
    }
    if let Some(since) = since {
        // check if it is a valid language then add to the url.
        if check_since_validity(since) {
            //needs formatting
            query.push_str(since);
        }
    }
    let url = format!("{}{}", URL, query);
    let res = reqwest::get(&url).await?.json::<Vec<Repository>>().await?;
    return Ok(res);
}
async fn check_language_validity(language: &str) -> Result<bool, Box<dyn Error>> {
    //Check if the language exists.
    // parameters:
    // langugage(str): The language, eg: python.
    //
    // Returns:
    // A boolena value. True for valid langugae, False otherwise.
    let languages: Vec<ProgrammingLanguage> = languages_list().await?;
    let language = language.to_ascii_lowercase();

    for lang in languages {
        if language == lang.name.to_ascii_lowercase() {
            return Ok(true);
        }
    }
    return Ok(false);
}

async fn check_spoken_lang_code_validity(lang_code: &str) -> Result<bool, Box<dyn Error>> {
    // Check if the spoken language exits.
    //
    // Parameters:
    // lang_code(str): the spoken language code, eg: en for english
    //
    // Returns:
    // A boolean value. True for valid speoken langauge, False otherwise
    let spoken_languages = spoken_languages_list().await?;
    let lang_code = lang_code.to_ascii_lowercase();

    for lang in spoken_languages {
        if lang_code == lang.name.to_ascii_lowercase() {
            return Ok(true);
        }
    }
    Ok(false)
}
// Check if the time range value is correct
//
// Parameters
// * since(&str): the time range
// Returns
// * a boolean alue. true for valid parameter, false otherwise
fn check_since_validity(since: &str) -> bool {
    ["daily", "weekly", "monthly"].contains(&since)
}
async fn languages_list() -> Result<Vec<ProgrammingLanguage>, Box<dyn Error>> {
    let url = "https://ghapi.huchen.dev/languages";
    let response: Vec<ProgrammingLanguage> = reqwest::get(url)
        .await?
        .json::<Vec<ProgrammingLanguage>>()
        .await?;

    Ok(response)
}
async fn spoken_languages_list() -> Result<Vec<SpokenLanguage>, Box<dyn Error>> {
    let url = format!("{}/spoken_languages?", URL);
    let response: Vec<SpokenLanguage> = reqwest::get(&url)
        .await?
        .json::<Vec<SpokenLanguage>>()
        .await?;
    Ok(response)
}
#[tokio::main]
pub async fn fetch_developers(
    language: Option<&str>,
    since: Option<&str>,
) -> Result<Vec<Developer>, Box<dyn std::error::Error>> {
    let mut url = format!("{}/developers?", URL);
    if let Some(language) = language {
        if check_language_validity(language).await? {
            // need formating before being pushed to the querry
            url.push_str(language);
        }
    }
    if let Some(since) = since {
        if check_since_validity(&since) {
            // need proper formating before being pushed to the querry
            url.push_str(since);
        }
    }
    let res = reqwest::get(&url).await?.json::<Vec<Developer>>().await?;
    Ok(res)
}
