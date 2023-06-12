
mod argparser;

fn main() {
    let args = argparser::parse_args();
    println!("Value {args:?}");

    if let Some(input_path) = args.input.as_deref() {
        println!("input file: {}", input_path.display());
    }
    if let Some(output_path) = args.output.as_deref() {
        println!("output file: {}", output_path.display());
    }

}
