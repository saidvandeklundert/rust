fn comp_errors() {
    let s1: String = String::from("this is me");

    let s2: &str = "myself";

    some_function(s1, s2);

    println!("{} {}", s1, s2);
    let v1 = recieving_ownership_from();
    // who is the owner of the vec defined in recieving_ownership_from  ?

    let v2 = vec![5, 8, 9, 7];

    let v3 = recieving_giving_back_ownership(v2);
    // who owns the data in v2 now?
    let mut stack_num = 56;

    stack_num = stack_function(stack_num);
    //What will be the value of the variable stack_num at the end of the main() function in the program given below.
    println!(
        "The value of the stack_num after the function call is {} ",
        stack_num
    );
    // What is the issue with the code below (up untill the print statement)?
    let mut some_vec = vec![1, 2, 3];

    let first = get_first_element(&some_vec);

    some_vec.push(4);

    println!("The first number is: {}", first);
    // There is a borrowing related issue in the code below. Try identifying it?
    let mut numbers = vec![1, 2, 3];

    let slice = get_slice(&mut numbers);

    numbers.push(4);

    println!("Slice: {:?}", slice);
}

fn get_slice(numbers: &mut Vec<i32>) -> &[i32] {
    &numbers[0..2]
}

fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
    &num_vec[0]
}

fn stack_function(mut var: i32) -> i32 {
    var = 50;

    var
}

fn some_function(a1: String, a2: &str) {
    println!("{} {}", a1, a2);
}

fn recieving_ownership_from() -> Vec<i32> {
    let vec1 = vec![4, 5, 6, 9];

    vec1
}

fn recieving_giving_back_ownership(vec1: Vec<i32>) -> Vec<i32> {
    vec1
}
