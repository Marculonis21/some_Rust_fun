use tokio::net::{TcpListener, TcpStream};
use tokio::io::{
    AsyncWriteExt, AsyncBufReadExt, BufReader
};
use std::env::args;
use anyhow::Result;

use rand::seq::SliceRandom;

const ANSWERS: [&'static str; 20] = ["It is certain",
"Reply hazy, try again",
"Don’t count on it",
"It is decidedly so	",
"Ask again later",
"My reply is no",
"Without a doubt",
"Better not tell you now",
"My sources say no",
"Yes definitely	",
"Cannot predict now	",
"Outlook not so good",
"You may rely on it	",
"Concentrate and ask again",
"Very doubtful",
"As I see it, yes",
"Most likely",
"Outlook good",
"Yes",
"Signs point to yes",
];

fn choose_answer() -> &'static str {
    let mut rng = rand::thread_rng();
    ANSWERS.choose(&mut rng).unwrap()
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    while buf_reader.read_line(&mut line).await.is_ok() {
        let input = line.trim();
        let answer = choose_answer();

        writer.write_all(format!("The answer to {input} is\n{answer}\n").as_bytes()).await?;

        line.clear();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let port = args().nth(1)
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    while let Ok((stream,address)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}
