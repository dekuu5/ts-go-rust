
fn get_input() -> &'static str {
    return "forward 5
down 5
forward 8
up 3
down 8
forward 2";
}

struct Point {
    x: i32,
    y: i32,
}

fn parse_line(line: &str ) -> Point {
    let (dir, amount) = line.split_once(" ").expect("must contain a  ");
    let amount = str::parse::<i32>(amount).expect("sec arg must be int");

    if dir == "forward" {
        return Point{x: amount, y: 0};
    } else if dir == "up" {
        return Point{x: 0, y: -amount};
    }
    return Point{x: 0, y: amount};

}

fn main() {
    let lines = get_input().lines();

    let mut x_and_y :Point = Point { x: 0, y: 0};

    for line in lines {
        let points = parse_line(line);
        x_and_y.x += points.x;
        x_and_y.y += points.y;
    } 

    println!("{}", x_and_y.x * x_and_y.y);

    // another way

    let result = get_input()
                    .lines()
                    .map(parse_line)
                    .fold(Point{x:0, y:0}, |mut acc, point| {
                        acc.x += point.x;
                        acc.y += point.y;
                        return acc;
                    });

    println!("{}", result.x * result.y);
    
}
