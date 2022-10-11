use std::process::Command;
use std::time::Instant;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub fn home_dir() -> String {
    dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}
pub fn runshellcmd(command: &mut Command) -> bool {
    let title = format!("{:?}", command);
    runshellcmd_titled(&title, command)
}
pub fn runshellcmd_titled(title: &str, command: &mut Command) -> bool {
    let start = Instant::now();
    println!("[ start ] {}", title);
    if title != format!("{:?}", command) {
        println!("> {:?}", command);
    }
    if let Ok(mut child) = command.spawn() {
        match child.wait().expect("error").code() {
            Some(code) => {
                if code == 0 {
                    println!("[  end  ] {} [{:?}]", title, start.elapsed());
                    return true;
                } else {
                    println!(
                        "[ error ] ({}) [{:?}] [code={}]",
                        title,
                        start.elapsed(),
                        code
                    );
                }
            }
            None => {
                println!("[ error ] ({}) [{:?}] [no code]", title, start.elapsed());
            }
        }
    } else {
        println!("[ error ] {} [{:?}] [no start]", title, start.elapsed());
    }
    false
}
pub fn success(start: &Instant) -> bool {
    println!("[success] [{:?}]", start.elapsed());
    true
}
pub fn lint_rust(paths: &[&str]) -> bool {
    let start = Instant::now();
    let mut ok: bool = true;
    for path in paths {
        if !runshellcmd_titled(
            &format!("Linting {}", path),
            Command::new("cargo")
                .arg("clippy")
                .arg("--")
                .arg("--deny")
                .arg("warnings")
                .current_dir(path),
        ) {
            ok = false;
        }
    }
    ok && success(&start)
}
pub fn format_rust(paths: &[&str]) -> bool {
    let start = Instant::now();
    let mut ok: bool = true;
    for path in paths {
        if !runshellcmd_titled(
            &format!("Linting {}", path),
            Command::new("cargo")
                .arg("fmt")
                .arg("--manifest-path")
                .arg(format!("{}/Cargo.toml", path)),
        ) {
            ok = false;
        }
    }
    ok && success(&start)
}
