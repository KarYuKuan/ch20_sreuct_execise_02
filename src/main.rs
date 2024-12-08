fn main() {
    let rec1 = Rectangle{
        width:32,
        height:44,
    };

    println!("the area of rec1 ={}-----",area(&rec1));
    println!("the area of rec1 ={rec1:?}-----");
}

#[derive(Debug)]
struct Rectangle{
    width:i32,
    height:i32,
}

fn area(rectangle:&Rectangle)->i32{
    rectangle.height*rectangle.width
}
