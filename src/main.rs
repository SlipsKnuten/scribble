
fn main() {

    //odd_or_even();
    largest_smallest();
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