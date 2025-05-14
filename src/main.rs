use chrono::{DateTime, Local};
use daemonize::Daemonize;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use std::process::exit;
use std::time::Duration;
use std::{fs, thread};

fn main() {
    let stdout = if fs::exists("/tmp/rs-daemon-spike.log").unwrap() {
        fs::File::options()
            .append(true)
            .open("/tmp/rs-daemon-spike.log")
            .unwrap()
    } else {
        fs::File::create("/tmp/rs-daemon-spike.log").unwrap()
    };
    let stderr = if fs::exists("/tmp/rs-daemon-spike.err").unwrap() {
        fs::File::options()
            .append(true)
            .open("/tmp/rs-daemon-spike.err")
            .unwrap()
    } else {
        fs::File::create("/tmp/rs-daemon-spike.err").unwrap()
    };

    let daemonize = Daemonize::new()
        .pid_file("/tmp/rs-daemon-spike.pid") // Every method except `new` and `start`
        // .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        // .user("nobody")
        // .group("daemon") // Group name
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => {
            eprintln!("Error, {}", e);

            exit(1);
        }
    };

    // シグナル処理の設定
    let mut signals =
        Signals::new(&[SIGTERM, SIGUSR1, SIGUSR2, SIGINT, SIGHUP]).expect("TODO: panic message");

    // シグナル処理用のスレッド
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("シグナル受信: {}", sig); // シグナル番号をログ出力

            match sig {
                SIGTERM => {
                    println!("終了シグナルを受信しました。プログラムを終了します :)");

                    let _ = fs::remove_file("/tmp/rs-daemon-spike.pid");

                    // TODO: クリーンアップ処理をここに書く

                    exit(0);
                }
                SIGUSR1 => {
                    println!("ユーザ定義のシグナル (その1)を受信しました。 :)");
                }
                SIGUSR2 => {
                    println!("ユーザ定義のシグナル (その2)を受信しました。 :)");
                }
                SIGINT | SIGHUP => {
                    println!("SIGINT/SIGHUP受信。無視して処理を継続します。");
                }
                _ => {
                    println!("未処理のシグナル: {}", sig);
                }
            }
        }
    });

    // メインの無限ループ（ここに実際のサービス処理を書く）
    loop {
        match std::panic::catch_unwind(|| {
            let local_datetime: DateTime<Local> = Local::now();
            println!("{} Hello, world!", local_datetime);
            println!("プロセス実行中 - PID: {}", std::process::id());
            thread::sleep(Duration::from_secs(10)); // 60秒から10秒に短縮してデバッグしやすく
        }) {
            Ok(_) => {} // 正常実行
            Err(e) => {
                println!("メインループでエラー発生: {:?}", e);
                // エラーが発生しても終了せずに継続
            }
        }
    }
}
