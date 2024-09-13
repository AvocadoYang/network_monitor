use tokio::process::Command;
use tokio::time::{sleep, Duration};
use std::process::Command as StdCommand;
use std::str;
use std:: io;

#[tokio::main]
async fn main() {
    let mut network_interface = String:: new();
    println!("Please input your network interface you want to monitor: ");
    io::stdin()
    .read_line(&mut network_interface)
    .expect("input error");

    let network_interface = network_interface.trim();

    if !check_if_interface_exists(&network_interface).await {
        eprintln!("Network interface '{}' does not exist!", network_interface);
        return;
    }

    loop {
        match check_network_interface_status(&network_interface).await {
            Ok(true) => {
                println!("{} is up", network_interface);
            }
            Ok(false) => {
                println!("{} is down or blocked. Restarting...", network_interface);
                restart_network_interface(&network_interface).await;
            }
            Err(e) => {
                eprintln!("Error checking status: {}", e);
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn check_if_interface_exists(interface: &str) -> bool {
    let output = StdCommand::new("ip")
        .args(&["link", "show", interface])
        .output();


    match output {
        Ok(result) => result.status.success(), // 成功找到網卡
        Err(_) => false, // 網卡不存在或命令執行失敗
    }
}

async fn check_network_interface_status(interface: &str) -> Result<bool, Box<dyn std::error::Error>> {

    let rfkill_output = StdCommand::new("rfkill")
        .args(&["list", interface])
        .output()?;

    let rfkill_output_str = str::from_utf8(&rfkill_output.stdout)?;

    if rfkill_output_str.contains("Soft blocked: yes") || rfkill_output_str.contains("Hard blocked: yes") {
        return Ok(false);
    }
    let ip_output = StdCommand::new("ip")
        .args(&["addr", "show", interface])
        .output()?;

    let ip_output_str = str::from_utf8(&ip_output.stdout)?;

    Ok(ip_output_str.contains("inet "))
}

async fn restart_network_interface(interface: &str) {

    //this is for wifi
    // if let Err(e) = Command::new("sudo")
    //     .arg("nmcli")
    //     .arg("radio")
    //     .arg("wifi")
    //     .arg("on")
    //     .output()
    //     .await
    // {
    //     eprintln!("Failed to unblock network interface: {}", e);
    //     return;
    // }

        if let Err(e) = Command::new("sudo")
        .arg("service")
        .arg("network-manager")
        .arg("restart")
        .output()
        .await
    {
        eprintln!("Failed to unblock network interface: {}", e);
        return;
    }
  

    println!("Successfully restarted network interface: {}", interface);
}



// for wifi
    // async fn check_network_interface_status(interface: &str) -> Result<bool, Box<dyn std::error::Error>> {
    //     let rfkill_output = StdCommand::new("rfkill")
    //         .args(&["list", "all"])
    //         .output()?;
    
    //     let rfkill_output_str = str::from_utf8(&rfkill_output.stdout)?;
    
    //     if rfkill_output_str.contains(&format!("{}: Soft blocked: yes", interface)) ||
    //        rfkill_output_str.contains(&format!("{}: Hard blocked: yes", interface)) {
    //         return Ok(false);
    //     }
    
    //     let ip_output = StdCommand::new("ip")
    //         .args(&["link", "show", interface])
    //         .output()?;
    
    //     let ip_output_str = str::from_utf8(&ip_output.stdout)?;
    
    //     Ok(ip_output_str.contains("state UP"))
    // }
    
