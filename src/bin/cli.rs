use thesis_cc::tokenize::tokenize;

fn main() {
    let tokens = tokenize(
        r##"
	int main() {
		return 0;
	}
	"##,
    );

    println!("{:?}", tokens);
}
