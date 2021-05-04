use std::env;
use std::io::Write;
use std::process::Command;
use std::thread::JoinHandle;
use std::thread::spawn;

fn main() {
    if env::args().len() < 2 {
        println!("usage: git-clone repo1 repo2 ...");
        return;
    }

    let repos = env::args().skip(1);
    let mut threads: Vec<JoinHandle<_>> = vec![];

    for repo in repos {
        let thread = spawn(move || {
            let output = Command::new("git").args(&["clone", &repo]).current_dir(env::current_dir().unwrap()).output().unwrap();
            if output.status.success() == false {
                std::io::stdout().write_all(&output.stdout).unwrap();
                std::io::stderr().write_all(&output.stderr).unwrap();
                return;
            }
            println!("clone {} done!", repo);
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
