extern crate reqwest;

fn main() {
    let api_key: &str = "9cb6de7ec25ee5dc276df1539830eb35";
    // let mut easy = Easy::new();
    let url = format!("http://api.openweathermap.org/geo/1.0/direct?q=London&limit=1&appid={}", api_key);
    match reqwest::get(&url) {
        Ok(mut response) => {
            // Check if 200
            if response.status() == reqwest::StatusCode::Ok {
                match response.text(){
                    Ok(text) => println!("Response text = {}", text),
                    Err(_) => println!("no"),
                }
            } else {
                println!("Response code not 200");
            }
        },
        Err(err) => println!("Somthing wrong!! : {}", err),
    }
}
