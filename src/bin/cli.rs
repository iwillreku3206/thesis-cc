use thesis_cc::tokenize::tokenize;

fn main() {
    let tokens = tokenize(
        r##"#include<stdio.h>
int main() {
	printf("Hello, world!\n");
	return 0;
}"##,
    );

    println!("{:?}", tokens);
}
