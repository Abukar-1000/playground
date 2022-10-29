// import system module
use std::process::Command;
extern crate sysinfo;
use sysinfo::{ Pid, ProcessExt, System, SystemExt };
use regex::RegexSet;

fn fib(int:i32) -> i32{
    if (int < 2){
        return 1;
    } else {
        return (fib(int - 1) + fib(int - 2));
    }
}

fn main(){

    
    let precessesToTerminate = RegexSet::new(&[
        r"Spotify",
        r"Discord"
    ]).unwrap();
    let mut targetIds: Vec<i32> = vec![];
    // processes 
    let mut system = System::new_all();
    let mut running: bool = true;


    while (running){
        for (id,process) in system.processes() {
            let matches: Vec<_> = precessesToTerminate.matches(process.name()).into_iter().collect();
            if (matches.len() > 0){
                process.kill();
            }
        } 
        system = System::new_all();
    }
}