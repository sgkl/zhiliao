use clap::{Arg, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches: App = App::new("zl")
        .version("0.1.0")
        .author("sgk <sunguanke11@mails.ucas.edu.cn>")
        .about("知了客户端")
        .subcommand(
            App::new("article")
                .about("文章")
        )
        .subcommand(
            App::new("articles")
                .about("文章列表")
        )
        .subcommand(
            App::new("books")
                .about("列出本地已下载书籍")
        )
        .subcommand(
            App::new("book")
                .about("书籍")
                .subcommand(
                    App::new("list")
                        .about("list local books")
                )
                .subcommand(
                    App::new("delete")
                        .about("delete book")
                        .arg("<book> 'book'")
                )
        )
        .subcommand(
            App::new("note")
                .about("笔记")
        )
        .subcommand(
            App::new("notes")
                .about("列出笔记")
        );

        matches.get_matches();

    println!("Hello, world!");
    Ok(())
}
