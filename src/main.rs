
fn main() {

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
