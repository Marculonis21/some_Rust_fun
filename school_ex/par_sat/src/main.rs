use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::io::stdin;
use anyhow::Result;

async fn solve(clauses: Vec<Vec<i64>>) -> bool {

    dbg!(&clauses);

    if clauses.is_empty() {
        return true;
    }

    let variable = clauses[0][0].abs();

    let left_negative = clauses
        .iter()
        .filter(|clause| !clause.contains(&(-variable)))
        .cloned()
        .collect::<Vec<_>>();

    let right_positive = clauses
        .iter()
        .filter(|clause| !clause.contains(&(variable)))
        .cloned()
        .collect::<Vec<_>>();

    let left_task = tokio::spawn(solve(left_negative));
    let right_task = tokio::spawn(solve(right_positive));

    dbg!(left_negative, right_positive);
    return left_task.await || right_task.await;
}

#[tokio::main]
async fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdin_reader = BufReader::new(stdin);

    let mut line = String::new();
    let mut clauses = vec![];

    while stdin_reader
        .read_line(&mut line)
        .await
        .is_ok()
    {
        if line.trim().is_empty() {
            break;
        }

        let clause = line.split_whitespace()
            .map(|x| x.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()?;

        clauses.push(clause);
        line.clear();
    }

    // println!("{clauses:?}");

    solve(clauses).await;

    Ok(())
}
