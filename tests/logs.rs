use std::sync::Arc;

use anyhow::Result;

use chrome_applicative::browser::tab::Tab;
use chrome_applicative::Browser;

pub mod logging;
mod server;

#[test]
fn enable_and_disable_logs() -> Result<()> {
    logging::enable_logging();
    let server = server::Server::with_dumb_html(include_str!(
        "logs_fixtures/basic_page_with_console_messages.html"
    ));
    let browser = Browser::default()?;
    let tab: Arc<Tab> = browser.wait_for_initial_tab()?;

    tab.enable_log()?;

    let url = format!("http://127.0.0.1:{}", server.port());
    tab.navigate_to(&url)?;

    tab.wait_until_navigated()?;

    tab.disable_log()?;

    Ok(())
}
