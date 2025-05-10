use std::fs::File;
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Local};
use daemonize::Daemonize;

fn main() {
    let stdout = File::create("/tmp/rs-daemon-spike.out").unwrap();
    let stderr = File::create("/tmp/rs-daemon-spike.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/rs-daemon-spike.pid") // Every method except `new` and `start`
        // .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        // .user("nobody")
        // .group("daemon") // Group name
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            println!("Success, daemonized");

            loop {
                let local_datetime: DateTime<Local> = Local::now();
                println!("{} Hello, world!", local_datetime);

                thread::sleep(Duration::from_secs(60));
            }
        },
        Err(e) => eprintln!("Error, {}", e),
    }
}
