use deps::clicode::{CLIChores, Routes};

mod deps;

#[tokio::main]
async fn main() {
    let mut cli_tool = CLIChores::new(vec![Routes::Home], &mut Routes::Home);

    cli_tool.run().await;
}
