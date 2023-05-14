use std::fs::File;
use std::io::copy;
use std::thread;
use std::time::Duration;

use reqwest::{blocking::Client, header};

fn main() {
    // 创建 HTTP 客户端
    let client = Client::new();

    loop {
        // 创建 GET 请求
        let request = client
            .get("http://173.82.58.11:8000/logo.jpg")
            .header(header::USER_AGENT, "reqwest")
            .build().expect("fail");

        // 发送请求并获取响应
        let response = client.execute(request);
        println!("{:?}",response);
        if let Ok(mut response) = response {
            // 读取响应体中的数据并将其写入文件
            let mut file = File::create("logo.jpg").unwrap();
            copy(&mut response, &mut file).unwrap();

        }

        // 暂停 10 分钟
        thread::sleep(Duration::from_secs(10 * 60));
    }
}