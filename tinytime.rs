use std::env::args;
use std::process::Command;
use std::time::Instant;

fn main() {
    let argv: Vec<String> = args().collect();
    if argv.len() > 1 {
        let command = &argv[1];
        let args = if argv.len() > 2 { Some(&argv[2..]) } else { None };

        let then = Instant::now();

        let mut process = match args {
            Some(args) => Command::new(command).args(args).spawn().unwrap(),
            None => Command::new(command).spawn().unwrap(),
        };

        process.wait().unwrap();

        let seconds = {
            let duration = then.elapsed();
            duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
        };

        println!("{}s", seconds);
    } else {
        println!("INFO:\nTinytime times the given shell command (COMMAND). The number returned is the seconds the process took to execute.\nhttps://github.com/asmoaesl/tinytime/\n");
        println!("USAGE:\ttinytime(.exe) <COMMAND>");
    }
}