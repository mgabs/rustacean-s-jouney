use reqwest::{blocking, header};
use serde::{Deserialize, Serialize};
use std::error::Error;

// Results Object
#[derive(Debug, Deserialize)]
struct Obj {
    items: Vec<Issue>,
}

/// Basic information about a pull request
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Issue {
    /// The human-visible number of the pull request
    number: usize,
    /// The title
    title: String,
    // creation date
    created_at: String,
    /// The URL of the pull request
    url: String,
    // body - for body length
    body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct VecResult {
    /// The human-visible number of the pull request
    number: usize,
    /// The title
    title: String,
    // creation date
    created_at: String,
    /// The URL of the pull request
    url: String,
    // body - for body length
    body: usize,
}

fn getter(get_url: &str) -> reqwest::blocking::Response {
    let client = blocking::Client::new();
    client
        .get(get_url)
        .header(header::USER_AGENT, "AwesomeBuilder")
        .send()
        .expect("Invalid json to parse")
}

fn main() -> Result<(), Box<dyn Error>> {
    //! defining the repo here.
    // let repo = "torvalds/linux";
    let repo = "rust-lang/rust";
    println!("Fetching data from Github for {}..", repo);

    let mut page_count = 0;
    let mut all: Vec<VecResult> = vec![];
    let url = format!( "https://api.github.com/search/issues?q=repo:{} is:open type:pr&sort=created_at&order=asc&per_page=100&page=", repo);

    // since github has a limit of 100 per response, looping to get all the data
    loop {
        // prepare url
        let page_url = format!("{}{}", url, page_count);
        // get data
        let resp = getter(&page_url);

        // check request response
        if resp.status().is_success() {
            if let Ok(a) = serde_json::from_str::<Obj>(&resp.text()?) {
                if a.items.len() == 0 {
                    // break if request is empty - github return empty success headers
                    break;
                }
                for i in a.items {
                    let result_vec = VecResult {
                        number: i.number,
                        title: i.title,
                        created_at: i.created_at,
                        url: i.url,
                        body: i.body.len(),
                    };
                    all.push(result_vec);
                }
            };
        } else {
            // break if request failed
            break;
        };
        page_count += 1;
    }

    if all.len() > 0 {

        // the number of results
        println!("Number of open PRs: {}", all.len());
        let oldest = all.len() - 1;
        // using the sort function on the API side
        println!(
            "Oldest: {}#{}: {}",
            repo, all[oldest].number, all[oldest].title
        );

        // ! Oldest: rust-lang/rust#65819: Add `IntoIterator` impl for arrays by value (`for [T; N]`)
        // ! Longest body: rust-lang/rust#79135: stabilize `#![feature(min_const_generics)]` in 1.50

        // folding to get maximum length of the body
    let x = all.iter().max_by(|&x, y| x.body.cmp(&y.body)).unwrap();
    // The longest PR is of length 65570 - this is API limitation on github's side
    println!(
        "Longest body: {}:#{}: {}",
        repo, x.number, x.title
    );
} else
{
    println!("The query didn't turn any results")
}

    Ok(())
}
