pub fn vector_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut single_row: Vec<SpreadsheetCell> = Vec::new();

    single_row.push(SpreadsheetCell::Int(3));
    single_row.push(SpreadsheetCell::Text(String::from("foo bar koo")));
    single_row.push(SpreadsheetCell::Float(82.12));

    for cell in &single_row {
        print!("{:?} ", cell);
    }

    single_row.push(SpreadsheetCell::Float(489.23));

    match single_row.get(5) {
        None => println!("None"),
        _ => println!("{:?}", single_row[4]),
    }

    match &single_row[3] {
        // Some(i) => println!("Some {:?}", i),
        // None => println!("None"),
        SpreadsheetCell::Int(i) => println!("Int {:?}", i),
        SpreadsheetCell::Text(m) => println!("Text {:?}", m),
        _ => println!("default: single row element {:?}", single_row[3]),
    }
}
