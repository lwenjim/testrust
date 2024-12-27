use regex::Regex;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{stdin, Write};
use tokio;
#[tokio::main]
async fn main() {
    loop {
        let mut file = OpenOptions::new()
            .append(true)
            .open("/tmp/a.cache")
            .unwrap();
        let mut data = String::new();
        stdin().read_line(&mut data).expect("error");
        let url: String = match data.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if url.len() == 0 {
            continue;
        }
        for i in url.split(" ").into_iter() {
            let re = Regex::new(r"(http|ftp|https):\/\/[\w\-_]+(\.[\w\-_]+)+([\w\-\.,@?^=%&:/~\+#]*[\w\-\@?^=%&/~\+#])?").unwrap();
            if !re.is_match(i) {
                println!("{}, {}", "failed ", i);
                continue;
            }
            let _ = file.write(((i.to_string() + "\n").as_str()).as_bytes());

            let data = get_html(i).await.unwrap().replace("\n", "");
            let _ = file.write((data + "\n").as_bytes());

            let buff = fs::read_to_string("/tmp/a.cache").unwrap();
            println!("{}", buff);
        }
    }
}

async fn get_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::builder().build().unwrap();
    let data = client.get(url).send().await?.text().await?;
    Ok(data)
}
