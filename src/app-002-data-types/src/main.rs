fn main() {
    /*
    Scalar Types
    */

    /*
    Char Type
    */

    let learn_face = '🤓';
    let eyes = '👀';

    /*
    String Type
    */

    let title = "Learning Rust";
    println!(
        "{} {} {} (Look at the code and comments)",
        learn_face, title, eyes
    );

    println!("\nChar Type\n");

    println!("char: {}", eyes);

    println!("\nString Type\n");

    println!("string: {}", title);

    /*
    Integer Types

    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    arch	isize	usize
    */
    println!("\nInteger Types\n");

    let number_integer_inferred = 42;
    println!("integer (inferred): {}", number_integer_inferred);

    let number_integer: i32 = 42;
    println!("integer: {}", number_integer);

    /*
    Float Types

    f32
    f64
    */
    println!("\nFloat Types\n");

    let float_f64 = 2.0;
    println!("float -> f64: {}", float_f64);

    let float_f32: f32 = 3.0;
    println!("float -> f32: {}", float_f32);

    /*
    Numeric Operations
    */
    println!("\nNumeric Operations\n");

    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("division -> quotient: {}", quotient);

    let floored = 2 / 3; // Results in 0
    println!("division -> floored: {}", floored);

    let floored_float = 2. / 3 as f64; // Results in 0 (both should be represented as float type)
    println!("division -> floored (integers as float): {}", floored_float);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    /*
    Boolean Type
    */
    println!("\nBoolean Type\n");

    let boolean_true = true;
    println!("boolean -> true: {}", boolean_true);

    let boolean_false: bool = false; // with explicit type annotation
    println!("boolean -> false: {}", boolean_false);

    /*
    Compound Types
    */

    /*
    Tuple Type
    */
    println!("\nTuple Type\n");

    let tup = (500, 6.4, 1 as f32);

    let (_x, y, z) = tup; // destructuring 😀

    println!("Tuple -> tup: {:?}", tup);

    println!(
        "Tuple -> (Destructuring) the value of 'y' and 'z' are: {}, {:.32}",
        y, z
    );

    println!(
        "Tuple -> to access the value of 'y': {} and 'z' {} use a dot '.' followed by the index e.g: tup.0",
        tup.1, tup.2
    );

    /*
    Array Type
    */
    println!("\nArray Type\n");
    let arr = ['🥑', '🥦', '🥒'];

    println!("Array -> arr: {:?}", arr);

    println!("Array -> arr[0]: {}", arr[0]);

    let typed_arr: [i32; 5] = [1, 2, 3, 4, 5]; // type how many items should be in the array and the type of each item

    println!("Array -> typed_arr: {:?}", typed_arr);

    let generated_arr = [3; 5]; // generates an array of 5 items with value 3

    println!("Array -> generated_arr: {:?}", generated_arr);
}
