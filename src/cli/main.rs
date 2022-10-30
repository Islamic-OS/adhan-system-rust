use colored::Colorize;
use inquire::{Confirm};
use serde_json::{Value};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
  greet_and_version();

  ask_for_location_reload().await;
}

fn greet_and_version() {
  println!("      __     ______   __     __     __     ___    __  {}", "     _______ __       ________".green());
  println!("     /  /\\  |   __  \\|   |  |   |  /  /\\  |   |\\ |   |{}", "    /  ____  \\ |     |__    __|".green());
  println!("    /  /  \\ |  |  \\  \\   |__|   | /  /  \\ |   | \\|   |{}", "   /  /    \\__||        |  |".green());
  println!("   /  /_\\  \\|  |   |  |         |/  /_\\  \\|   |  \\   |{}", "  |  |      |  |        |  |".green());
  println!("  /  _____  \\  |   |  |   __    |  _____  \\   |\\  \\  |{}", "  |  |      __ |        |  |".green());
  println!(" /  /     \\  \\ |__/  /   |  |   | /     \\  \\  | \\  \\ |{}", "   \\  \\____/  ||______ _|  |__".green());
  println!("/__/       \\__\\____ /|__ |  |__ |/       \\__\\ |  \\__\\|{}", "    \\ ______ /_______ |______ |".green());

  println!("");
  println!("{} {}{}", "Adhan Client CLI".green(), "v".green().bold(), VERSION.green().bold());
  println!("\n");
}

async fn ask_for_location_reload() {
  let ans = Confirm::new("Do you want to recalculate your location coordinates?")
    .with_default(false)
    .with_help_message("Make sure you have a good internet connection, and that you are not using a VPN.")
    .prompt();
  
  match ans {
    Ok(true) => {
      println!("{}", "Reloading location...".blue().bold());
      reload_location().await.unwrap();
    },
    Ok(false) => println!("Skipping location reload..."),
    Err(e) => println!("Error: {}", e),
  }
}

async fn reload_location() -> Result<(), ()> {
  let res = reqwest::get("https://ipwho.is/").await.unwrap();
  let json_str = res.text().await.unwrap_or_else(|e| {
    println!("Error: {}", e);
    std::process::exit(1);
  });

  let json_parsed: Value = serde_json::from_str(&json_str).unwrap_or_else(|e| {
    println!("Error: {}", e);
    std::process::exit(1);
  });

  let json_obj = json_parsed.as_object().unwrap();

  println!("Latitude: {}, Longitude: {}", json_obj.get("latitude").unwrap(), json_obj.get("longitude").unwrap());

  Ok(())
}
