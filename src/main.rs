use std::env;
use std::process::Command;

fn main() {
    let args:Vec<String> = env::args().collect();

    let bootloader_path: &String = &args[1];
    let json_path:&String = &args[2];

    let res = run_script(bootloader_path, json_path);
    match res {
        Ok(_) =>(),
        Err(e) => eprintln!("{}", e)
    }
}


fn run_script<'a>(script_path: &str, json_path: &str) -> std::io::Result<()> {
    // bootloader only receives json and produce json
    let (program, args) = if script_path.ends_with(".lua") {
        ("lua", vec![script_path, json_path])
    } else if script_path.ends_with(".py") {
        ("python", vec![script_path, json_path])
    } else if script_path.ends_with(".ps1") {
        // Windows PowerShell 写法，若安装了 PowerShell 7 可用 "pwsh"
        ("powershell", vec!["-File", script_path, json_path])
    } else if script_path.ends_with(".sh") {
        ("sh", vec![script_path, json_path])
    } else {
        panic!("unsupported bootloader");
    };

    let output = Command::new(program)
        .args(&args)
        .output()?; // 等待执行完成并捕获输出 (在我们的设计中，输入 json 得到 output.json 在工作目录)

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}