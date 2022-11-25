
// import system module
use std::process::Command;
// extern crate sysinfo::SystemExt;
// extern crate sysinfo::SystemExt;

use std::thread;
use std::time::Duration;

fn fib(int:i32) -> i32{
    if (int < 2){
        return 1;
    } else {
        return (fib(int - 1) + fib(int - 2));
    }
}

fn listProcesses(){
    thread::spawn(|| {
        let system = sysinfo::System::new();
        let processes = system.get_process_list();
        for (id,process) in processes.itter() {
            println!("{} {}",process.id,process.name);
            thread::sleep(Duration::from_millis(1));
        } 
    });
}
fn main(){

    let mut values = vec![
        "first String",
        "Second String",
        "THird String"
    ];
    // processes 
    for value in 0..10 {
        println!("[+] Random line [{}]",value);
    } 
}