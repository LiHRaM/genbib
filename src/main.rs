use argh::FromArgs;
use async_std::task;
use scraper::{Html, Selector};
use uuid::Uuid;

#[doc(hidden)]
pub(crate) type Error = Box<dyn std::error::Error>;

#[doc(hidden)]
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Generate a bibtex entry from an url.
#[derive(FromArgs, PartialEq, Debug)]
struct Args {
    /// the url of the entry.
    #[argh(positional)]
    url: String,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    task::block_on(print_entry(args))
}

async fn print_entry(args: Args) -> Result<()> {
    let response = surf::get(&args.url).recv_string().await?;
    let document = Html::parse_document(&response);

    let title: String = document
        .select(&Selector::parse("title").unwrap())
        .next()
        .unwrap()
        .text()
        .collect();

    let key = Uuid::new_v4().to_string();

    println!(r#"@misc{{{},
    title = {{{}}},
    howpublished = {{\url{{{}}}}},
}}"#, key, title.trim(), &args.url);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

fn sorted<'a>(x: &'a str, y: &'a str) -> (&'a str, &'a str) {
    if x < y {
        (x, y)
    } else {
        (y, x)
    }
}

    #[test]
    fn success() {
        
    }
}