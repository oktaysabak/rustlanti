/* Integer Types
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize
*/
fn main() {
    let spaces = "   ";
    let hex = 0xff;
    let space_size = spaces.len();
    println!("space size is: {}", space_size);
    println!("space size is: {}", hex);

    // float types
    let x = 2.0;

    let y: f32 = 3.0;

    println!("x: {}, y: {}", x, y);

    //boolean type

    let t = true;

    let f: bool = false;

    println!("tru: {}, false: {}", t, f);

    //character type
    let heart_eyed_cat = '😻';

    println!("cat {}", heart_eyed_cat);

    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (first, second, third) = tup;

    println!("{} - {} - {} ", tup.0, tup.1, tup.2);

    println!("the value of first is {}", first);

    // array type
    //unlike tuples all values should be same type
    let a = [1, 2, 3, 4];

    println!("{}", a[0]);

}