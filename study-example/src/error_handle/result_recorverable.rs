use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn open_file(file_path: &str) -> File {
    /*
    File::open 的返回类型是 Result<T, E> 。
    通用参数 T 已由 File::open 的实现填充为成功值的类型 std::fs::File ，它是一个文件句柄。
    错误值中使用的 E 类型为 std::io::Error 。
    此返回类型意味着对 File::open 的调用可能会成功并返回我们可以读取或写入的文件句柄。
    函数调用也可能失败：例如，文件可能不存在，或者我们可能没有访问该文件的权限。
    File::open 函数需要有一种方法来告诉我们它是成功还是失败，同时为我们提供文件句柄或错误信息。
    此信息正是 Result 枚举所传达的信息
    */
    let open_file_result = File::open(file_path);
    match open_file_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(fc) => fc,
                Err(error_create) => panic!("create file error: {:?}", error_create),
            },
            other_error => {
                panic!("Open file error: {:?}", other_error);
            }
        },
    }
}

fn open_file_unwrap(file_path: &str) -> File {
    // let file = File::open(file_path).unwrap();
    let file = File::open(file_path).expect("file not exist.");
    println!("file: {:?}", file);
    return file;
}

fn read_file_from_name(file_path: &str) -> Result<String, io::Error> {
    let open_file_result = File::open(file_path);
    let mut file = match open_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => return Err(e),
    }
}

fn read_file_from_name_short(file_path: &str) -> Result<String, io::Error> {
    let mut content = String::new();
    /*let mut open_file = File::open(file_path)?;
    open_file.read_to_string(&mut content)?;*/
    // 可以简化为如下
    File::open(file_path)?.read_to_string(&mut content)?;
    return Ok(content);
}

pub fn result_recover_study() {
    let file_path = String::from("hello.txt");
    open_file(&file_path);
    open_file_unwrap(&file_path);
    let file_content = read_file_from_name(&file_path).expect("read file error.");
    println!("file content: {}", file_content);
    let file_content2 = read_file_from_name_short(&file_path).expect("read file short error.");
    println!("file content2: {}", file_content2);
}
