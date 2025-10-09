
fn main() {

    //odd_or_even();
    //largest_smallest();
    //duplicates();
    sum_of_even();
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