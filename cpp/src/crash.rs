
// import system module
use std::process::Command;
extern crate sysinfo::SystemExt;

fn fib(int:i32) -> i32{
    if (int < 2){
        return 1;
    } else {
        return (fib(int - 1) + fib(int - 2));
    }
}

fn main(){

    let mut values = vec![
        "first String",
        "Second String",
        "THird String"
    ];
    // processes 
    let system = sysinfo::System::new();
    let processes = system.get_process_list();
    for (id,process) in processes.itter() {
        println!("{} {}",process.id,process.name);
    } 
}