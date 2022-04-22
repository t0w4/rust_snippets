use std::collections::HashMap;

fn hello(name: String) {
    println!("hello {}!", name);
}

fn hello_borrow(name: &str) {
    println!("hello {}!", name);
}

// wordはこの関数が終了する時点で所有権が破棄されるので. wordの参照先がなくなるのでエラーになる
// fn return_dead_value() -> &str {
//     let word = String::from("hello");
//     return &word
// }

fn change_name(name: &mut String) {
    // let new_name = (*name).clone(); // &String の参照を外してからclone, *name.cloneだと str型になる
    // *name = new_name + " sun";
    *name = format!("{} {}", *name, String::from("sun")) // format!マクロの方が楽
}

fn call_user_name_and_age(user: User) {
    println!("I'm {}, {}", user.name, user.age);
}

struct User {
    name: String,
    age: i32,
}

fn main() {
    // {}で任意にスコープを設定可能
    {
        let name1 = String::from("bob");
        let name2 = name1;
        // println!("{}", name1); // エラーになる
        println!("{}", name2);

        let address1 = String::from("tokyo");
        let address2 = address1.clone();
        println!("{}", address1); // cloneすればエラーにならない
        println!("{}", address2);

        // プリミティブ型は所有権が移動しない(コピー)
        let age1 = 12;
        let age2 = age1;
        println!("{}", age1);
        println!("{}", age2);

        // 関数呼び出しでも所有権は移動する
        let hello_name = String::from("jon");
        hello(hello_name);
        // println!("{}", hello_name); // エラーになる

        // 参照渡しならなら借用になり所有権は渡らない
        let hello_name2 = String::from("jon");
        hello_borrow(&hello_name2);
        println!("{}", hello_name2);

        // 関数呼び出しで引数を変更する場合は関数の定義と呼び出し側で &mut を指定する必要がある
        let mut name_change = String::from("alice");
        change_name(&mut name_change); // 変数の定義で mut をつけていたとしても呼び出しの際に mut が必要
        println!("{}", name_change)
    }

    let word = String::from("Rhogeあ森");
    for char in word.chars() {
        println!("{}", char);
    }
    for byte in word.bytes() {
        println!("{:x}", byte);
    }

    // 構造体
    let mut user1 = User {
        name: String::from("bob"),
        age: 15,
    };
    user1.age = 12;
    call_user_name_and_age(user1);

    // HashMap
    let mut hoge_map: HashMap<&str, i32> = HashMap::new();
    hoge_map.insert("A", 1);
    hoge_map.insert("B", 2);
    println!("{:?}", hoge_map);
    println!("{:?}", hoge_map.get("B"));
    println!("{:?}", hoge_map.get("C"));
    match hoge_map.get("B") {
        Some(v) => println!("B={}", v),
        None => println!("B=None!"),
    }
    match hoge_map.get("C") {
        Some(v) => println!("C={}", v),
        None => println!("C=None!"),
    }
    // キーとバリューをforで取得
    for (str, num) in hoge_map {
        println!("{} {}", str, num);
    }

    // 配列
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", array[0]);
    print_array(array);

    // Vec
    let mut nums: Vec<i32> = vec![1, 2, 3, 4];
    let poped_num = nums.pop().unwrap();
    println!("{}", poped_num);
    println!("{}", nums[0]);
    nums.push(5);
    println!("length={}", nums.len()); // lenで要素数取れる
    for elem in nums.iter() {
        println!("v={}", elem);
    }
    // iterつけると参照が返されるっぽい
    for elem in nums.iter() {
        println!("v={}", elem);
    }

    let mut word = vec![];
    word.push("O");
    word.push("K");
    println!("{}", word.join(""));
    let mut it = word.iter();
    println!("{:?}", it.next().unwrap());
    println!("{:?}", it.next().unwrap());

    let mut nums2 = Vec::new();
    nums2.push(2);
    nums2.push(4);
    nums2.push(6);
    println!("{:?}", nums2);

    // slice
    let word = String::from("abcde");
    println!("{:?}", &word[0..2]); // sliceを取得するときは&変数名[n..m]の形で取得(元の変数への参照)
    println!("{:?}", &word[..]); // sliceの全てを取得するときは[..]を指定

    // vecや配列も同様
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", &vector[0..2]); // sliceを取得するときは&変数名[n..m]の形で取得(元の変数への参照)
    let vector: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", &vector[0..2]); // sliceを取得するときは&変数名[n..m]の形で取得(元の変数への参照)
    print_slice(&vector[..]);

    // タプル
    let tuple_test: (i32, String, i32) = (12, String::from("bob"), 23);
    println!("{} {} {}", tuple_test.0, tuple_test.1, tuple_test.2);
}

fn print_array(arr: [i32; 5]) {
    println!("length={}", arr.len()); // lenで要素数取れる
    for (i, v) in arr.iter().enumerate() {
        println!("index: {}, value: {}", i, v)
    }
}

// sliceを引数に指定する場合は &[型] の形式で取得
fn print_slice(slice: &[i32]) {
    for v in slice {
        println!("v={}", v)
    }
}