pub fn data_type() {
  // Integers

  let a: i32 = 98_234; // Decimal
  let b: i32 = 0xff; // Hex
  let c: i32 = 0o04; // Octal
  let d: i32 = 0b1101_1011; // Binary
  let e: u8 = b'A'; // Byte (u8 only) // 'A' = 65

  println!("{} {} {} {} {}", a, b, c, d, e);

  // Integer overflow
  let f: u8 = 255;
  /*
    u8 can only repersent 0-255
    at production build its become 2's complement
    256 -> 0
    257 -> 1
    etc ...
  */
  println!("{}", f);

  // Floating point

  let g: f64 = 2.0;
  let h: f32 = 4.0;

  println!("{} {}", g, h);

  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3; //  returns Float

  println!("{} {}", sum, difference);

  // Characters
  let x = 's';
  let y = 'S';
  let heart_eyed_cat = '😻';

  println!("{} {} {}", x, y, heart_eyed_cat);

  // Compound Types
  let tup: (&str, i64) = ("key", 83);
  // destructuring touple
  let (state, value) = tup;
  let val = tup.1;
  // let k = error_codes[5]; // error. index out of bound

  println!("{} {} {}", state, value, val);

  let error_codes = [200, 404, 500];
  let not_found = error_codes[1];

  let byte = [0; 8]; // array with 8 zeros

  println!("{:?} {} {:?}", error_codes, not_found, byte);
}
