use std::fs::{remove_file, File};
use std::process::exit;
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Local};
use daemonize::Daemonize;
use signal_hook::consts::SIGTERM;
use signal_hook::iterator::Signals;

fn main() {
    let stdout = File::create("/tmp/rs-daemon-spike.log").unwrap();
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
        Ok(_) => println!("Success, daemonized"),
        Err(e) => {
            eprintln!("Error, {}", e);

            exit(1);
        },
    };

    // シグナル処理の設定
    let mut signals = Signals::new(&[SIGTERM]).expect("TODO: panic message");

    // シグナル処理用のスレッド
    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGTERM => {
                    println!("終了シグナルを受信しました。プログラムを終了します :)");

                    let _ = remove_file("/tmp/rs-daemon-spike.pid");

                    // TODO: クリーンアップ処理をここに書く

                    exit(0);
                },
                _ => {},
            }
        }
    });

    // メインの無限ループ（ここに実際のサービス処理を書く）
    loop {
        let local_datetime: DateTime<Local> = Local::now();
        println!("{} Hello, world!", local_datetime);

        thread::sleep(Duration::from_secs(60));
    }
}
