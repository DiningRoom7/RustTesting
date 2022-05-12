
fn main() {
    //Four primary scalar types:
    //  integers, floats, booleans, and characters

    //INTEGERS:
    let _: i8; let _: u8;       //8 bit
    let _: i16; let _: u16;     //16 bit
    let _: i32; let _: u32;     //32 bit
    let _: i64; let _: u64;     //64 bit
    let _: i128; let _: u128;   //128 bit
    let _: isize; let _: usize; //64 bits on 64 bit computers, 32 bits on 32 bit computers
    //INTEGER LITERALS:
    let _ = 10_999;     //=10999 Decimal literal. Append type onto end to specify (eg. 10_999u16)
    let _ = 0xff;       //Hex literal. Again you can specify type for all literals
    let _ = 0o77;       //Octal literal
    let _ = 0b111_00;   //Binar literal
    let _ = b'A';       //Byte. u8 only

    //FLOATING POINT NUMBERS:
    let _: f32;     //32 bit
    let _: f64;     //64 bit

    //BOOLEAN:
    let _: bool = true;

    //CHARACTERS:
    let _: char = 'B'; //Four byte unicode character. Must use single quotes
}