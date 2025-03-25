use crate::pager::read_file;

mod links {
    use thiserror::Error;
    use select::document::Document;
    use select::predicate::Name;
    use reqwest;

    #[derive(Error, Debug)]
    pub enum LinkError {
        #[error("Request failed: {0}")]
        ReqError(#[from] reqwest::Error),
        #[error("IO error: {0}")]
        IoError(#[from] std::io::Error),
    }

    async fn get_links(page: &str) -> Result<Vec<Box<str>>, LinkError> {
        let res = reqwest::get(page)
            .await?
            .text()
            .await?;

        let links = Document::from(res.as_str())
            .find(Name("a"))
            .filter_map(|node| node.attr("href"))
            .into_iter()
            .map(|link| Box::<str>::from(link.to_string()))
            .collect();

        Ok(links)
    }

    pub async fn get_valid_links(page: &str) -> Result<Vec<Box<str>>, LinkError> {
        let mut links = get_links(page).await?;
        links.retain(|link| {
            if link.starts_with("http") {
                true
            } else {
                println!("Invalid link: {}", link);
                false
            }
        });
        Ok(links)
    }
}

mod pager {
    use std::fs;

    pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
        let contents = fs::read_to_string(file_path);
        contents
    }

}

#[tokio::main]
async fn main() -> Result<(), links::LinkError> {
    let url = &reqwest::Url::parse_with_params("https://www.bing.com/search?q=", &[("ie", "UTF-8"), ("q", "inurl:ftp intext:admin")]).unwrap().to_string();
    let response = reqwest::get(url).await?;
    println!("Response status code: {}", response.status());
    response.text().await?;
    let page_links = links::get_valid_links(url).await?;
    for link in page_links {
        println!("{}", link);
    }
    let binding = read_file("dorks.dat").unwrap();
    let dorks = binding.as_str();
    for dork in dorks.lines() {
        let url = &reqwest::Url::parse_with_params("https://www.bing.com/search?q=", &[("ie", "UTF-8"), ("q", dork)]).unwrap().to_string();
        println!("{}", url);
    }
    Ok(())
}
