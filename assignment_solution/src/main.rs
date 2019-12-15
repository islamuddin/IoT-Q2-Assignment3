use std::thread;
use std::time::Duration;

fn main() {
    println!("========= [ Assignment 3 Soultion 1 ]");
    solution_1();
    println!("========= [ Assignment 3 Soultion 2 ]");
    solution_2();
    println!("========= [ Assignment 3 Soultion 3 ]");
    solution_3();    
}

// Q1. Write a function which expects a closure with FnOnce() trait and call that closure in the
// function body.
fn solution_1() {
    fn consume_closure<F:FnOnce()->String>(func: F) where F: FnOnce() -> String    {
        println!("Consumed: {}", func());
        println!("After!");
    }

    println!("Before!");
    let msg=||->String{ 
        String::from("Hello!")  
    };

    consume_closure(msg);

}   

// ------------------------------------------------------
// Q2. Write a function which expects a closure with FnMut() trait and call that closure in the
// function body.
fn solution_2(){
    fn fnmut_closure<T: FnMut()->u32>(mut counter: T){
        println!("Counter : {:?}",counter());
        println!("Counter : {:?}",counter());
        println!("Counter : {:?}",counter());
        println!("Counter : {:?}",counter());
        println!("Counter : {:?}",counter());    
    }    
    let mut counter = 0;
    let counter_closure = ||{
                                counter = counter+1;
                                counter
                            };
    fnmut_closure(counter_closure);
}

// ------------------------------------------------------
// Q3. Create a new thread with spawn and Waiting for All Threads to Finish Using join Handles.
fn solution_3(){

    let handle = thread::spawn(move || {
        for i in 1..10{
            println!("{}.   / .-\"-. \\ ",i);
            thread::sleep(Duration::from_millis(10));
        }
    });


    for j in 1..10{
        println!("{}.   \\ .-\"-. /",j);
        thread::sleep(Duration::from_millis(10));

    }

    handle.join().unwrap();

}
