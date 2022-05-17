
struct RgbColor(u8, u8, u8);

struct Point(i32, i32, i32);

fn main() {
    let _black = RgbColor(0,0,0);
    let _white = RgbColor(255,255,255);

    let _origin = Point(0,0,0);
    let _p = Point(142, 550, 21);
}