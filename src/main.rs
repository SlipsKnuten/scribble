use rand::Rng;
use chrono::Local;
use std::env::args;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::Read;


use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

fn main() {
    //dice_game();
    //randomize_number();
    //odd_or_even();
    //largest_smallest();
    //duplicates();
    //sum_of_even();
    //filter_even();
    //traffic();
    //structs();
    //boxes();
    //boxes_enums();
    //word_count();
    //todo_list();
    error_handling();


    /*

    channels

    let (rx, handle) = channels();
    let received = rx.recv().expect("Failed to receive message");
    println!("Message received: {}", received);
    handle.join().unwrap();
    */

}

fn largest_smallest(){
    let test_arr = vec![42, 17, 93, -5, 0, 18];
    let mut largest = test_arr[0];
    let mut smallest = test_arr[0];
    for &i in &test_arr {
        if i > largest {
            largest = i;
        }
        if i < smallest {
            smallest = i;
        }
    }

    println!("Largest: {}", largest);
    println!("Smallest: {}", smallest);
}

fn odd_or_even(){

    let mut test = vec![1, 2, 3, 4, 5, 6];
    let mut even = vec![];
    let mut odd = vec![];
    for c in test {
        if c % 2 == 0{
            even.push(c);
        }
        else{
            odd.push(c);
        }
    }

    println!("Even: {:?}", even);
    println!("Odd: {:?}", odd);
}


fn duplicates(){
    let numbers = vec![1, 3, 3, 7, 1, 5, 5, 5, 9];
    let mut unique_numbers = Vec::new();

    for num in numbers {
        if !unique_numbers.contains(&num){
            unique_numbers.push(num);
        }
    }
    println!("Unique numbers: {:?}", unique_numbers);
}

fn sum_of_even(){
    let numbers = vec![5, 10, 15, 20, 25, 30];
    let mut total = 0;

    for num in &numbers {
        if num % 2 == 0 {
            total += num;
        }
    }

    println!("Sum of even: {}", total);

}

fn randomize_number(){
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(1..=10);
    let now = Local::now();

    println!("Random number: {}", num);
    println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S"));

}

fn dice_game(){
    
    let mut game_active: bool = true;
    let mut rng = rand::thread_rng();

    let num: i32 = rng.gen_range(1..=10);
    println!("Random number: {}", num);


    while game_active {

        let new_num: i32 = rng.gen_range(1..=10);
        
        println!("Go again? y/n");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read Stdin");

        
        if input.trim() == "y" {
            println!("Random number: {}", new_num);
        }
        if input.trim() == "n" {
            game_active = false
        }
    }
    
}

fn channels() -> (mpsc::Receiver<String>, thread::JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let msg = String::from("Message from thread!");
        tx.send(msg).expect("Failed to send message");
    });

    (rx, handle)
}

fn filter_even(){


    let numbers = vec![1,2,3,4,5];
    let doubled_even: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();

    println!("Original numbers: {:?}", numbers);
    println!("Doubled even numbers: {:?}", doubled_even);

}

fn traffic(){

    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    let traffic_light = TrafficLight::Red;

    match traffic_light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Gogogogo!"),
    }
}

fn structs(){

    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle{
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let result = Rectangle {width: 30, height: 60};
    println!("The area of the rectangle is {} square pixels.", result.area());

}

fn boxes(){
    let my_box = Box::new(1);
    println!("Value of box: {}", my_box);
}

fn boxes_enums() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    impl List {
        fn len(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }

        fn print(&self) {
            match *self {
                List::Cons(head, ref tail) => {
                    println!("{}", head);
                    tail.print();
                }
                List::Nil => {}
            }
        }
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("Length: {}", list.len());
    println!("Contents:");
    list.print();
}

fn word_count() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let words = contents.split_whitespace();
    let count = words.count();

    println!("Words: {}", count);

    Ok(())
}

fn todo_list() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("\n1) Add task");
        println!("2) Show tasks");
        println!("3) Exit");
        print!("> ");
        io::stdout().flush().expect("flush failed");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");

        match choice.trim() {
            "1" => {
                let mut task = String::new();
                println!("Enter task:");
                io::stdin().read_line(&mut task).expect("Failed to read task");
                tasks.push(task.trim().to_string());
                println!("Task added!");
            }
            "2" => {
                println!("\nYour TODO list:");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}: {}", i + 1, task);
                }
            }
            "3" => {
                println!("Exiting");
                break;
            }
            _ => {
                println!("Invalid option, please type 1, 2 or 3");
            }
        }
    }
}

fn error_handling() {
    let a: i32 = 5;
    let b = i32::MAX;

    match a.checked_add(b) {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: overflow occured"),
    }
}

fn fizzbuzz (){
    let numbers: Vec<i32> = (1..=100).collect();
    let (fizz, buzz, fizzbuzz) = ("Fizz", "Buzz", "FizzBuzz");
    
    for i in numbers {
        if i % 15 == 0 {
            println!("{}", fizzbuzz);
        }
        else if i % 5 == 0 {
            println!("{}", buzz);
        }
        else if i % 3 == 0 {
            println!("{}", fizz);
        }
        else {
            println!("{}", i);
        }
    }
}
