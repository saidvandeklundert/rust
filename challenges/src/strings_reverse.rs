pub fn print_hello() {

    let string = String::from("Hello, world!");

    // Chaining a few methods available to String:
    let reversed: String = string.chars().rev().collect();
    println!("{reversed}");

    // print the letters in reverse one at a time:
    for letter in string.chars().rev(){
        println!("{letter}")
    }
    // build a new string entirely by pushing characters onto the new string one at a time:
    let mut in_reverse: String = "".to_owned();
    
    for character in string.chars().rev(){
        in_reverse.push(character);
        
    }    
    
    println!("{}", in_reverse);    
}