//!
//! 6章 ライブラリ型 サンプルプログラム
//! 

/// 6-4.Vec<T>
/// リスト6-17 インスタンスの生成
#[allow(dead_code)]
pub fn instantiate(){
    // with_capacity()メソッドを利用して整数を扱うベクタを生成する
    let vec: Vec<i32> = Vec::with_capacity(5);
    println!("ポインタ = {:p} , 長さ = {:?} , 容量 = {:?}" , &vec , &vec.len() , &vec.capacity());
    // vec![]マクロを利用してタプルを扱うベクタを生成する
    let vec = vec![(10 , 20),(100 , 200)];
    println!("ポインタ = {:p} , 長さ = {:?} , 容量 = {:?}" , &vec , &vec.len() , &vec.capacity());
}

/// 6-4.Vec<T>
/// リスト6-18 要素の追加
#[allow(dead_code)]
pub fn add(){
    let mut vec: Vec<i32> = Vec::with_capacity(5);// 容量が5のミュータブルなベクタを生成する
    for value in 0..10 { // 0から9までの値をベクタに追加する
        vec.push(value);
    }
    println!("内容 = {:?} , 長さ = {:?} , 容量 = {:?}" , &vec , &vec.len() , &vec.capacity());
    let mut vec = vec![(10 , 20),(100 , 200)];// 容量が2のミュータブルなベクタを生成する
    vec.insert(1, (1000 , 2000)); // 1番目に新しい要素を追加する
    println!("内容 = {:?} , 長さ = {:?} , 容量 = {:?}" , &vec , &vec.len() , &vec.capacity());
}

/// 6-4.Vec<T>
/// リスト6-19 要素の取得と変更
#[allow(dead_code)]
pub fn get_and_change(){
    let mut vec = vec![1 , 2 , 3 , 4 , 5]; // ミュータブルなベクタを生成する
    println!("1番目の値 = {:?}" , vec[1]);
    vec[2] *=  100; // 2番目の値を変更する
    println!("{:?}" , &vec)
}

/// 6-4.Vec<T>
/// リスト6-20 要素の削除
#[allow(dead_code)]
pub fn remove(){
    let mut nums = vec![1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9];
    let x = nums.drain(2..5);// ２番目から要素5まで削除する
    println!("削除した要素 = {:?}" , &x.collect::<Vec<_>>());
    println!("削除結果 = {:?}" , &nums);
    let mut nums = vec![1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9];// 奇数の要素を削除する
    let mut index = 0;
    while index < nums.len() {
        if nums[index] % 2 == 1 {
            let num = nums.remove(index);
            println!("削除した要素 = {:?}" , &num);
        } else {
            index += 1;
        }
    }
    println!("偶数のベクタ = {:?}" , &nums);
}

/// 6-4.Vec<T>
/// リスト6-21 要素をソートする
#[allow(dead_code)]
pub fn sort(){
    let mut nums = vec![8 , 5 , 3 , 1 , 4 , 6 , 2 , 5 , 9 , 2];
    nums.sort(); // ベクタの要素を昇順ソートする
    println!("sort() = {:?}" , &nums);
    nums.dedup();// 重複する要素を削除する
    println!("deup() = {:?}" , &nums);
    // 降順ソートする
    use std::cmp::Reverse;
    let mut nums = vec![8 , 5 , 3 , 1 , 4 , 6 , 2 , 5 , 9 , 2];
    nums.sort_by_key(|&element| Reverse(element));
    println!("sort_key() = {:?}" , &nums);
}