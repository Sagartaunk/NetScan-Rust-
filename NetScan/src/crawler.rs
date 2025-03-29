use tokio::{task , sync::Mutex};
use reqwest;
use scraper::{Html,Selector};
use std::sync::Arc;
use crate::{cli, save};
use futures::future;

pub async fn run(){
    let (url , max_depth) = cli::domain();
    let to_visit = Arc::new(Mutex::new(vec![url]));
    let visited = Arc::new(Mutex::new(Vec::new()));
    crawl(0, max_depth, to_visit.clone(), visited.clone()).await;
}


pub async fn crawl(depth : u8 , max_depth : u8 , to_visit : Arc<Mutex<Vec<String>>> , visited : Arc<Mutex<Vec<String>>>){
    if depth >= max_depth {
        let visited_lock = visited.lock().await;
        save::save_domains(visited_lock.to_vec());
        return;
    }


    let mut tasks = vec![];
    let urls = {
        let mut  to_visit_lock = to_visit.lock().await;
        std::mem::take(&mut *to_visit_lock)
    };

    for url in urls{
        let to_visit = Arc::clone(&to_visit);
        let visited = Arc::clone(&visited);
        
        tasks.push(task::spawn(async move{
            let mut visited_lock = visited.lock().await;
            if visited_lock.contains(&url){
                return;
            }
            visited_lock.push(url.clone());
            drop(visited_lock);
            let links = parse(url.clone()).await;
            let mut to_visit_lock = to_visit.lock().await;
            to_visit_lock.extend(links);
            drop(to_visit_lock);
        }));
    }
    future::join_all(tasks).await;
    if depth == 0 {
        let visited_lock = visited.lock().await;
        save::save_domains(visited_lock.to_vec());
    }
    Box::pin(crawl(depth + 1, max_depth, to_visit, visited)).await;


}

pub async fn parse(url : String) -> Vec<String>{
    let request = reqwest::get(url).await.unwrap().text().await.unwrap();
    let text_data = Html::parse_document(&request);
    let selector = Selector::parse("a").unwrap();
    let mut links  = Vec::new();
    for element in text_data.select(&selector){
        if let Some(href) = element.value().attr("href"){
            if href.starts_with("https://") {
                links.push(href.to_string());
            }
        }
    }
    println!("Found {} links", links.len());
    links
}