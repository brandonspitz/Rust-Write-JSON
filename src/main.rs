use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article {
        article: String::from("Writing JSON with Rust"),
        author: String::from("Brandon"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence")
            },
            Paragraph {
                name: String::from("body of paragraph")
            },
            Paragraph {
                name: String::from("end of paragraph")
            }
        ]
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("The JSON is: {}", json)
}