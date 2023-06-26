fn main() {
    if cfg!(target_os = "macos") {
        println!("macos");
    } else if cfg!(target_os = "linux") {
      println!("linux");
    }
}