fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    // let result = largest_func(&number_list);

    //println!("The largest number is {}", *largest);
    // println!("The largest number is {}", result);
}

fn largest_func(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        println!("{} {}", item, largest);
        if item > largest {
            largest = item;
        }
    }

    largest
}
  