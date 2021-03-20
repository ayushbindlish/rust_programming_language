// #[allow(unconditional_panic)]
pub fn compound_types() {


    // tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tuple values stored in separate variables: {}, {}, {}",x,y,z);

    println!("printing using indices");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("tuple values using indices: {}, {}, {}",five_hundred,six_point_four,one);

    // arrays
    let arr = [1,2,3,4];
    println!("{}",arr[0]);

    let months :[&str;12] = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    println!("{}",months[2]);

    let mut arr2 = [2;5];
    println!("{:?}",arr2);
    arr2[4]=3;
    println!("{}", arr2[1]);




}