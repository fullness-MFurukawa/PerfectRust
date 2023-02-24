//!
//! 4章 基本データ型
//! サンプルプログラム
//! 

/// ### 4-8.文字列型
/// #### リスト4-29 文字列リテラル
#[allow(dead_code)]
pub fn declar() {
    let str = "Hello Rust.";
    println!("value = {:?} , ptr = {:p} , len = {}" , str  , str , str.len() );
}

/// ### 4-8.文字列型
/// #### リスト4-31 文字列の結合
#[allow(dead_code)]
pub fn binding(){
    let str_a = "Hello ";
    let str_b = "Rust.";
    println!("str_a:value = {:?} , ptr = {:p} , len = {}" , str_a  , str_a , str_a.len() );
    println!("str_b:value = {:?} , ptr = {:p} , len = {}" , str_b  , str_b , str_b.len() );
    let str_c = str_a.to_owned() + str_b;
    println!("str_c:value = {:?} , ptr = {:p} , len = {}" , str_c  , &str_c , str_c.len() );
}

/// ### 4-8.文字列型
/// #### リスト4-32 データ変換メソッド
#[allow(dead_code)]
pub fn methods_1(){
    let str_value = "ABCDEF";
    let result = str_value.as_bytes(); // 文字列をバイトスライスに変換した結果を返す
    println!("as_bytes() = {:?}" , &result);
    let result = str_value.bytes(); // 文字列をバイトコードのイテレータに変換した結果を返す
    println!("bytes() = {:?}" , result);
    let result = str_value.chars(); // 文字列を文字のイテレータに変換した結果を返す
    println!("chars() = {:?}" , result);
    let str_value = "123";
    let result = str_value.parse::<i32>().unwrap();// 文字列を整数に変換した結果を返す
    println!("parse() = {:?}" , result);
}

/// ### 4-8.文字列型
/// #### リスト4-33 値を調べるメソッド
#[allow(dead_code)]
pub fn methods_2(){
    let str_value = "山田太郎";
    let result = str_value.contains("田"); // 指定された文字列が含まれているか
    println!("contains() = {:?}" , result);  
    let result = str_value.find("田");// 指定された文字列の位置を返す
    println!("find() = {:?}" , result);
    let str_value = "sample.txt";
    let result = str_value.ends_with(".txt"); // 文字列の終わりが指定された文字列と一致するか
    println!("ends_with() = {:?}" , result);
    let result = str_value.starts_with("sample"); // 文字列の始まりが指定された文字列と一致するか
    println!("starts_with() = {:?}" , result);
    let result = str_value.is_empty(); // 文字列が空であるかを返す
    println!("is_empty() = {:?}" , result);
}

/// ### 4-8.文字列型
/// #### リスト4-34 値を取得するメソッド
#[allow(dead_code)]
pub fn methods_3(){
    let str_value = "ABC\r\nDEF\r\nXYZ\r\n";
    let result = str_value.lines();// 改行単位で文字列を取得する
    for row in result {
        println!("lines() = {:?}" , row);
    }
    let str_value = "ABCDEFXYZ";
    let result = str_value.get(0..=3).unwrap(); // 0番目から4番目の要素を取得する
    println!("get(0..3) = {:?}" , result);
    let result = str_value.get(0..).unwrap(); // 0番目以降のすべての要素を取得する
    println!("get(0..) = {:?}" , result);
}


/// ### 4-8.文字列型
/// #### リスト4-35 値を生成、置換するメソッド
#[allow(dead_code)]
pub fn methods_4(){
    // 指定された回数文字列生成した結果を返す
    let str_value = "ABCDEFXYZ";
    let result = str_value.repeat(3);
    println!("repeat(3) = {:?}" , result);
    // 文字列を置き換える
    let result = str_value.replace("ABC", "abc");
    println!("replace() = {:?}" , result);
}    

/// ### 4-8.文字列型
/// #### リスト4-36 大文字/小文字変換、トリミング、分割メソッド
#[allow(dead_code)]
pub fn methods_5(){
    let str_value = "ABCDEFXYZ";
    let result = str_value.to_lowercase();// 小文字変換する
    println!("to_lowercase() = {:?}" , result);
    let str_value = "abcdefxyz";
    let result = str_value.to_uppercase(); // 大文字変換する
    println!("to_uppercase() = {:?}" , result);
    let str_value = " Hello Rust ";
    let result = str_value.trim(); // 前後のホワイトスペースをトリミングする
    println!("trim() = {:?}" , result);
    let str_value = "ABC,DEF,XYZ";
    let result = str_value.split(",");// 指定された文字で分割する
    for value in result{
        println!("split() = {:?}" , value);
    }
}
