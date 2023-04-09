

fn main() {
    let pi: f32 = 3.14;

    // even this works
    // let formatted: String = format!("{:0>10.5}", pi);
    // print!("formatted pi: {}", formatted);

    print!("formatted pi: {:0>10.5}", pi);

    // meaning of 10.5 is:
    //                                 .--5--.          <---- this is `.5`
    //                                |       |               5 digits after decimal point
    //   3 . 1 4  --->    0 0 0 0 3 . 1 4 0 0 0
    //                    |                   |
    //                     `--------10-------`          <---- this is `10`
    //                                                        total 10 digits
    //                                                        decimal point excluded
    //

}







