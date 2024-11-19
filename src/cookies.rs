use std::env;
use std::process::{Child, Command};
use thirtyfour::{DesiredCapabilities, WebDriver};

const INFO_INSTALL_GECKODRIVER: &str = 
    "Error launching geckodriver \
    Please ensure that geckodriver is installed and available in your PATH \
    You may download geckodriver from: https://github.com/mozilla/geckodriver/releases or install it by running cargo install gecodriver";


fn launch_geckodriver(geckodriver_path: &str) -> std::io::Result<Child> {
    // Add a spinner to indicate progress
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner.set_message("Installing geckodriver... (cargo install geckodriver)");

    if geckodriver_path.is_empty() {
        Command::new("cargo")
            .arg("install")
            .arg("geckodriver")
            .output()
            .expect("failed to install geckodriver. Please ensure that cargo is installed and available in your PATH");
    }

    // If the OS is linux:
    // Check if geckodriver is installed
    // If not, install it using cargo
    // If it is installed, launch it as a subprocess
    // If the OS is Windows:
    // Check if geckodriver is installed
    // If not, download the latest release from the geckodriver releases page
    // If it is installed, launch it as a subprocess
    // If the OS is MacOS:
    // Check if geckodriver is installed
    // If not, install it using cargo

    let os = env::consts::OS;
    match os {
        "linux" => {
            let geckodriver_installed = Command::new("which")
                .arg("geckodriver")
                .output()
                .expect("failed to check if geckodriver is installed");
            if !geckodriver_installed.stdout.is_empty() {
                spinner.finish_with_message("geckodriver is installed -> launching geckodriver");
                return Command::new("geckodriver")
                    .arg("--port")
                    .arg("4444")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
            } else {
                spinner.finish_with_message(
                    "geckodriver is not installed -> installing geckodriver though cargo",
                );
                return Ok(Command::new("cargo")
                    .arg("install")
                    .arg("geckodriver")
                    .spawn()
                    .expect(INFO_INSTALL_GECKODRIVER));
            }
        }

        "windows" => {
            let geckodriver_installed = Command::new("where")
                .arg("geckodriver")
                .output()
                .expect("failed to check if geckodriver is installed");
            if !geckodriver_installed.stdout.is_empty() {
                spinner.finish_with_message("geckodriver is installed -> launching geckodriver");
                return Command::new("geckodriver")
                    .arg("--port")
                    .arg("4444")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
            } else {
                println!("{}", INFO_INSTALL_GECKODRIVER);
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Please install geckodriver"))
            }
        }
        "macos" => {
            let geckodriver_installed = Command::new("which")
                .arg("geckodriver")
                .output()
                .expect("failed to check if geckodriver is installed");
            if !geckodriver_installed.stdout.is_empty() {
                spinner.finish_with_message("geckodriver is installed -> launching geckodriver");
                return Command::new("geckodriver")
                    .arg("--port")
                    .arg("4444")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
            } else {
                spinner.finish_with_message(
                    "geckodriver is not installed -> installing geckodriver though cargo",
                );
                return Ok(Command::new("cargo")
                    .arg("install")
                    .arg("geckodriver")
                    .spawn()
                    .expect(INFO_INSTALL_GECKODRIVER));
            }
        }
        _ => {
            spinner.finish_with_message("Unsupported OS");
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported OS",
            ))
        }
    }
}


pub async fn get_auth_cookie() -> Result<String, Box<dyn std::error::Error>> {
    let geckodriver_path = env::var("GECKODRIVER_PATH").unwrap_or("geckodriver".to_string());
    let mut _geckodriver = launch_geckodriver(&geckodriver_path);
    if _geckodriver.is_err() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error launching geckodriver",
        )));
    }

    // Wait for geckodriver to start
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Set up Chrome options for a non-headless browser
    let caps = DesiredCapabilities::firefox();

    // Start the WebDriver session
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    // Add a cookie to the browser session

    // Navigate to a sample webpage
    driver.get("https://aulaglobal.uc3m.es").await?;

    if driver.windows().await?.is_empty() {
        println!("Window closed---------------------");
    }

    loop {
        let cookies = driver.get_all_cookies().await?;
        if cookies
            .iter()
            .any(|cookie| cookie.name == "MoodleSessionag")
        {
            let cookie_moodle = cookies
                .iter()
                .find(|cookie| cookie.name == "MoodleSessionag")
                .unwrap();

            // Close the WebDriver session
            driver.quit().await?;
            _geckodriver.unwrap().kill().unwrap();

            return Ok(cookie_moodle.value.clone());
        }
        tokio::time::sleep(std::time::Duration::from_millis(750)).await;
    }
}
