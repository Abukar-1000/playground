
// import system module
use std::process::Command;
use sysinfo::SystemExt;
use sysinfo::ProcessExt;
// extern crate sysinfo::SystemExt;
// extern crate sysinfo::SystemExt;

use std::thread;
use std::time::Duration;
use sysinfo::System;

fn fib(int:i32) -> i32{
    if (int < 2){
        return 1;
    } else {
        return (fib(int - 1) + fib(int - 2));
    }
}

fn listProcesses(){
    
}
/*
implements multithreading
*/
fn main(){

    // listProcesses();
    let mut system = System::new_all();
    thread::spawn(move || {
        for value in 0..10 {
            println!("[+] Random line [{}]",value);
            thread::sleep(Duration::from_millis(1));
        } 
    });
    // processes 
    
    for (id,process) in system.processes() {
        println!("{} {}",id,process.name());
        thread::sleep(Duration::from_millis(1));
    }
}