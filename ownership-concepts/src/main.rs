// Checklist

// Write the concatenate_strings function signature.


// Implement the concatenate_strings function.


// Initialize two String variables in the main function.


// Call the concatenate_strings function with string slices of the variables.

// Define a function to concatenate two string slices into a new String


fn concatenate_strings(string1: &str, string2: &str) -> String {
    
    let mut result = String::new();
    
    
    result.push_str(string1);
    
   
    result.push_str(string2);
    
    
    result
}

fn main() {
   
    let string1 = String::from("Hello, ");
    let string2 = String::from("Ehsan!");
    
   
    let concatenated_string = concatenate_strings(&string1, &string2);
    
    
    println!("{}", concatenated_string);
}
