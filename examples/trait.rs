extern crate atty;

use atty::IsATTY;

fn main() {
    let stdin  = std::io::stdin();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let file   = std::fs::File::open("LICENSE").unwrap();
    let tcp    = std::net::TcpStream::connect("1.1.1.1:80").unwrap();

    println!("{:<25} - {}", "std::io::Stdin",      stdin.isatty());
    println!("{:<25} - {}", "std::io::Stdout",     stdout.isatty());
    println!("{:<25} - {}", "std::io::Stderr",     stderr.isatty());
    println!("{:<25} - {}", "std::fs::File",       file.isatty());
    println!("{:<25} - {}", "std::net::TcpStream", tcp.isatty());

    println!("\ndyn trait Example:");
    let all : Vec<Box<dyn IsATTY<_>>> = vec![
        Box::new(stdin),
        Box::new(stdout),
        Box::new(stderr),
        Box::new(file),
        // The following is not possible on windows because, as the IsATTY trait was
        // implemented, the boxed type signatures differ:
        // Box<dyn IsATTY<SOCKET>> vs Box<dyn IsATTY<HANDLE>>
        #[cfg(unix)]
        Box::new(tcp)
        ];

    for (i, b) in all.into_iter().enumerate() {
        println!("element #{} : {}", i, b.isatty());
    }
}