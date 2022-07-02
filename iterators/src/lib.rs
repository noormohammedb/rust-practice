#[derive(Debug, PartialEq, Eq)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_my_size(my_shoes: Vec<Shoe>, my_shoe_size: u32) -> Vec<Shoe> {
    my_shoes
        .into_iter()
        .filter(|s| s.size == my_shoe_size)
        .collect()
}
mod iterator;

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            }
        ]
    );

    println!("{:?}", in_my_size);
}
