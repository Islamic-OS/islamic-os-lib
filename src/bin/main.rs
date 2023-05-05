use deps::{
    clicode::CLIChores,
    tuicode::{Routes, TUIChores},
};

mod deps;

#[tokio::main]
async fn main() {
    let mut tui_tool = TUIChores::new(vec![Routes::Home], &mut Routes::Home);
    let mut cli_tool = CLIChores::new();

    if cli_tool.is_args_provided() {
        cli_tool.run();
    } else {
        tui_tool.run().await;
    }
}
