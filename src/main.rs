use serde_json::*;
use serde::*;
use std::thread;
use std::fs::File;
use std::sync::RwLock;
use lazy_static::lazy_static;
use std::thread::sleep;
use std::env;
pub struct FileReader{
    pub path : &'static str,

}
lazy_static! {
    pub static ref INC: RwLock<FileReader> = RwLock::new(FileReader{ path: "./sample.json"});
}
impl FileReader {
    pub fn read( filepath:&'static str) -> File{
        File::open("./sample.json").unwrap()
    }
    pub fn write() {
        for i in 1..10 {
            println!("{}", i)
        }
    }
}

fn main() {
   env_t();
//   json_t()

}
pub fn json_t(){
    let thread1 = thread::spawn(|| {

        let f = FileReader::read("./sample.json");
        let values:serde_json::Value = serde_json::from_reader(f).unwrap();
        println!("整个字符串 t1：{:?}",values);
        println!("name t1:{}",values["name"]);
        println!("age t1:{}",values["age"]);
        println!("hobby-sports t1:{}",values["hobby"]["sports"]);
        println!("hobby-food t1:{}",values["hobby"]["food"]);
        println!("t1 {:?}", values["name"].as_str().unwrap());
        println!("t1 {:?}", values["age"].as_i64().unwrap());
    });
    let thread2 = thread::spawn(|| {
        FileReader::write()
    });
    let thread3 = thread::spawn(|| {
        let f = FileReader::read("./sample.json");
        let values:serde_json::Value = serde_json::from_reader(f).unwrap();
        println!("整个字符串 t3：{:?}",values);
        println!("name t3:{}",values["name"]);
        println!("age t3:{}",values["age"]);
        println!("hobby-sports t3:{}",values["hobby"]["sports"]);
        println!("hobby-food t3:{}",values["hobby"]["food"]);
        println!("t3 {:?}", values["name"].as_str().unwrap());
        println!("t3 {:?}", values["age"].as_i64().unwrap());
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();

    let r = INC.read();
}
pub fn env_t(){
    let args: Vec<String> = env::args().collect();
    //println!("get command args :{:?} ", args);
    //println!("get env args : ");
    for (key, value) in env::vars() {
        //println!("  {}  =>  {}", key, value);
    }
    let key = "PATH";
    match env::var(key) {
        Ok(val) => {
            // val is String, splited by ";"
            println!("val =>{}",val);
        },
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
}