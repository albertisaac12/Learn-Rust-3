// struct Point<X1, Y1> {
//     x: X1,
//     y:Y1
// }

// impl<X2 , Y2> Point<X2,Y2> {

//     fn mixup<X3,Y3>(self,other:Point<X2,Y2>) -> Point<X2,Y2> {
//         Point { x: (self.x), y: (other.y) }
//     }

// }
#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {

    let p1 = Point {x:5,y:10.4};
    let p2 = Point {x:"hello",y:'c'};

    let p3 = p1.mixup(p2);

    println!("{:#?}",p3);

}

