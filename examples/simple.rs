use colored::Colorize;
use rucline::completion;
use rucline::Prompt;

fn main() {
    if let Ok(Some(string)) = Prompt::from("What's you favorite website? ".bold())
        // Add some likely values as completions
        .completer(completion::Basic::new(&[
            "https://www.rust-lang.org/",
            "https://docs.rs/",
            "https://crates.io/",
        ]))
        // Add some tab completions
        .suggester(completion::Basic::new(&[
            "https://www.startpage.com/",
            "https://www.google.com/",
        ]))
        //Block until value is ready
        .read_line()
    {
        println!("'{}' seems to be your favorite website", string);
    }
}
