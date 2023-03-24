use std::process::Command;
use thirtyfour::prelude::*;
use webdriver_install::Driver;
use std::path::PathBuf;


fn check_geckodriver() -> Result<PathBuf, Box<dyn std::error::Error>>{
    // Check if ./bin directory exists, if not, create it:
    if !PathBuf::from("./bin").exists() {
        std::fs::create_dir("./bin").expect("Failed to create bin directory");
    }

    // Check if geckodriver exists in bin directory; if not, fetch it!
    #[cfg(windows)]
    let gecko_path = PathBuf::from("./bin/geckodriver.exe");
    #[cfg(not(windows))]
    let gecko_path = PathBuf::from("./bin/geckodriver");

    // Install geckodriver if it doesn't exist:
    if !gecko_path.exists() {
        Driver::Gecko
            .install_into(PathBuf::from("./bin/"))
            .expect("Failed to install geckodriver");
    }

    // Return gecokdriver path:
    Ok(gecko_path)
}


#[tokio::main]
async fn run_web_automation(gecko_path: PathBuf) -> WebDriverResult<()> {

    // Launch geckodriver process in background:
    let mut gecko_process = Command::new(gecko_path)
        .args(&["-p", "9515"])
        .spawn()
        .expect("Failed to launch geckodriver. Please make sure that geckodriver is installed and available in your PATH.");


    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to the URL:
    driver.goto("https://docs.rs/thirtyfour/").await?;

    println!("Page title: {}", driver.title().await?);

    // Close the browser:
    driver.quit().await?;

    // Kill geckodriver process:
    gecko_process.kill().expect("Failed to kill geckodriver process");

    Ok(())
}

fn main() {
    let gecko_path = check_geckodriver().expect("Failed to configure geckodriver");
    run_web_automation(gecko_path).expect("Failed to run web automation");
}
