use select::{document::Document, predicate::{Class, Name}};
use scraper::{Html, Selector};

/**
 *  Returns Vec<&str> of direct links to results of a rule34.paheal.net search using the provided space delimited keywords
 */
pub fn fetch_direct_links(keywords: String) -> Vec<String>
{
    // Create search request
    let page: String = format!("https://rule34.paheal.net/post/list/{}/1", keywords);

    // Send request and hold response
    let response = reqwest::blocking::get(page).unwrap();
    assert!(response.status().is_success());

    // Create searchable html document from response
    let html = Document::from_read(response).unwrap();

    let mut links: Vec<String> = Vec::new();
    for image_list in html.find(Class("shm-image-list"))
    {
        for image in image_list.find(Name("a"))
        {
            let results = image.attr("href").unwrap();
            if results.contains("https")
            {
                links.push(results.to_string());
            }
        }
    }

    return links;
}