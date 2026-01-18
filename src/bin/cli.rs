use thesis_cc::{
    std::util::{fractional_value_string_from_u8_array, hex_string_to_u8_array},
    tokenizer::tokenizer::tokenize,
};

fn main() {
    let tokens = tokenize(
        r##"#include<stdio.h>
int main() {
	printf("Hello, world!\n");
	return 0;
}"##,
    );

    println!("{:?}", tokens);

    println!("====================");

    let arr = hex_string_to_u8_array("F", false).unwrap();

    println!("Slice: {:?}", arr);

    println!("{}", fractional_value_string_from_u8_array(arr.as_slice()));
}
