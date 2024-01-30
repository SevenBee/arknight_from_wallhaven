use anyhow::{Ok, Result};
use reqwest;
use select::document::Document;
use select::predicate::Class;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

static BASE_URL: &str = "https://wallhaven.cc/search?q=明日方舟";

#[tokio::main]
async fn main() -> Result<()> {
    println!("请求网页中");
    let content = reqwest::get(BASE_URL).await?.text().await?;
    // println!("{:#?}",content);
    // 创建缓存目录
    println!("检查文件目录");
    let path: &Path = Path::new("cache/base.html");
    if !path.parent().unwrap().exists() {
        fs::create_dir(path.parent().unwrap())?;
    }

    // if !path.exists(){
    // 创建文件
    println!("写入文件");
    let mut file = File::create(path).unwrap();
    // 写入文件
    file.write_all(content.as_bytes())?;
    // }

    let document = Document::from(include_str!("../cache/base.html"));
    // println!("{:#?}",document);
    for node in document.find(Class("preview")) {
        let image_url = node.attr("href").unwrap();
        println!("{:?}", image_url);
    }

    println!("获取最大值");
    // println!("{:#?}",page_node);
    let max_page_num: i64 = get_mux_page(document).unwrap();
    let urls: Vec<String> = create_base_urls(max_page_num);
    Ok(())
}

fn get_mux_page(document: Document) -> Result<i64> {
    // 获取当前最大page
    let mut page_num: i64 = 1;
    for node in document.find(Class("pagination")) {
        let json_content: Value = serde_json::from_str(node.attr("data-pagination").unwrap())?;
        println!("{:?}", json_content["total"].as_i64().unwrap());
        page_num = json_content["total"].as_i64().unwrap();
    }
    Ok(page_num)
}

fn create_base_urls(page_num: i64) -> Vec<String> {
    // 生成一系列最开的请求链接
    let mut urls = Vec::<String>::new();
    if page_num == 1 {
        urls.push(BASE_URL.to_string());
    } else if page_num > 1 {
        for num in 2..page_num + 1 {
            let page_url = format!("{}&page={}", BASE_URL, num);
            urls.push(page_url);
        }
    }
    println!("{:#?}", urls);
    urls
}
