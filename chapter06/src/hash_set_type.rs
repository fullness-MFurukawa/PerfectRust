//!
//! 6章 ライブラリ型 サンプルプログラム
//! 

use std::collections::HashSet;
/// 6-7.HashSet<T,S>
/// リスト6-30 インスタンスの生成
#[allow(dead_code)]
pub fn instantiate(){
    let set_a = HashSet::<i32>::new(); // 整数HashSetを生成する
    let set_b:HashSet<&str> = HashSet::with_capacity(5);// 容量が5の文字列HashSetを生成する
    // vec!マクロを利用してHashSetを生成する
    let set_c:HashSet<i32> = vec![10 , 20 , 30].into_iter().collect();
    println!("len() = {}  , set_a = {:?}" , &set_a.len() , &set_a);
    println!("len() = {}  , set_b = {:?}" , &set_b.len() , &set_b);
    println!("len() = {}  , set_c = {:?}" , &set_c.len() , &set_c);
}

/// 6-7.HashSet<T,S>
/// リスト6-31 要素の追加と削除
#[allow(dead_code)]
pub fn add_and_remove(){
    // イテレータからHashSetを生成する
    let mut set_a:HashSet<_> = vec![10 , 20 , 30].into_iter().collect();
    set_a.extend([50 , 60 , 70]);// イテレータを利用して要素を追加する
    println!("extend() = {:?}" , &set_a);
    let x = set_a.insert(100); // 要素100を追加する
    if x {
        println!("insert() = {:?}" , &set_a);
    }else{
        println!("要素を追加できませんでした。");
    }
    let x= set_a.remove(&100); // 要素100を削除する
    if x {
        println!("remove() = {:?}" , &set_a);
    }else{
        println!("要素を削除できませんでした。");
    }
    set_a.retain(|&v| v % 4 == 0); // 4の倍数となる要素以外を削除する
    println!("retain() = {:?}" , &set_a);
}

/// 6-7.HashSet<T,S>
/// リスト6-32 要素の取得
#[allow(dead_code)]
pub fn get(){
    // イテレータからHashSetを生成する
    let set_a:HashSet<_> = vec![10 , 20 , 30].into_iter().collect();
    let r:String = match set_a.get(&10){ // get()メソッドで要素を取得する
        None => "要素は存在しません。".to_string(),
        Some(x) => format!("要素{}を取得しました。", &x)
    };
    println!("{}" , r);
    let iter_a = set_a.iter();// イテレータを取得する
    for v in iter_a {
        println!("{}" , &v);
    } 
}

/// 6-7.HashSet<T,S>
/// リスト6-33 集合演算
#[allow(dead_code)]
pub fn set_operation(){
    // 集合-A
    let set_a:HashSet<_> = vec![10 , 20 , 30 , 50 , 60].into_iter().collect();
    // 集合-B
    let set_b:HashSet<_> = vec![30 , 40 , 50 , 70 , 80].into_iter().collect();
    let x = set_a.union(&set_b);// 集合-Aと集合-Bの和集合
    println!("和集合 = {:?}" , x);
    let x = set_a.intersection(&set_b);// 集合-Aと集合-Bの積集合
    println!("積集合 = {:?}" , x);
    let x = set_a.difference(&set_b);// 集合-Aと集合-Bの差集合
    println!("差集合 = {:?}" , x);
    let x = set_a.symmetric_difference(&set_b);// 集合-Aと集合-Bの対称差集合
    println!("対称差集合 = {:?}" , x);
}