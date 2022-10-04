fn main() {

    let my_array = ['a', 'b', 'c', 'd',];
    println!("first item in array -> {}", my_array[0]);

    for l in my_array {

        println!("{}", l);

    }


    let detailed_array: [u32; 4] = [4, 3, 2, 1,];
    println!(" Debug mode -> {:?}", detailed_array); // Cannot run with default formatter {}
                                      // debug mode {:?}
    for n in detailed_array {
        
        println!("{}", n);

    }


    let detailed_tuple: (u8, u8, u8) = (1, 2, 3);
    println!(" Debug mode -> {:?}", detailed_tuple); // Cannot run with default formatter {}
                                      // debug mode {:?}

    println!("first item in tuple -> {}", detailed_tuple.0);
}
