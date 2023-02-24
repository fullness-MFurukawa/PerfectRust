//!
//! 4章 基本データ型
//! サンプルプログラム
//! 


/// ### 4-7.スライス型
/// #### リスト4-20 スライスの取得
#[allow(dead_code)]
pub fn get() {
    // スライスを取得する文字列配列
    let str_array = ["ABC","DEF","GHI","JKL","MNO","PQR","STU"];
    let slice1:&[&str] = &str_array[3..=5]; // 3～5番目のスライスを取得する
    let slice2 = &str_array[0..2]; // 先頭から2番目のスライスを取得する
    let slice3 = &str_array[..];// 配列全体のスライスを取得する
    println!("slice1 = {:?}" , slice1);
    println!("slice2 = {:?}" , slice2);
    println!("slice3 = {:?}" , slice3);
}

/// ### 4-7.スライス型
/// #### リスト4-22　Range構造体の利用
#[allow(dead_code)]
pub fn range() {
    // スライスを取得する文字列配列
    let int_array = [1 , 2 , 3 , 4 , 5 , 6 , 7];
    // Range構造体を生成する
    let range = std::ops::Range{start:1 , end:3};
    // スライスを取得する
    let slice = &int_array[range];
    println!("slice = {:?}" , slice);
}

/// ### 4-7.スライス型
/// #### リスト4-24 マルチバイトの利用
#[allow(dead_code)]
pub fn multibyte_slice() {
    let company_name = String::from("株式会社フルネス");
    // 株式会社の範囲をスライスで取得する
    let slice = &company_name[..12];
    println!("参照範囲={:?} , 大きさ={}" , slice , slice.len());
    // フルネスの範囲をスライスで取得する
    let slice = &company_name[12..];
    println!("参照範囲={:?} , 大きさ={}" , slice, slice.len());

}
/// ### 4-7.スライス型
/// #### リスト4-25 ファットポインタを確認する
#[allow(dead_code)]
pub fn fat_pointer(){
    let int_array = [0 , 1 , 2 , 3 , 4 , 5 , 6];
    // 配列全体のスライスを取得する
    let slice: &[i32] = &int_array[0..];
    println!("参照範囲={:?} ,ポインタ={:p}, 要素数={}" , slice , slice , slice.len());
    // 1番目から3番目のスライスを取得する
    let slice = &int_array[3..5];
    println!("参照範囲={:?} ,ポインタ={:p}, 要素数={}" , slice , slice , slice.len());
}

/// ### 4-7.スライス型
/// #### リスト4-26 値の取得や状態確認メソッド
#[allow(dead_code)]
pub fn methods_1(){
    let array = [100 , 101, 102 , 103 , 104];
    let slice: &[i32] = &array[0..];
    println!("first()   = {:?}" , slice.first().unwrap()); // 先頭の値を取得する
    println!("last()    = {:?}" , slice.last().unwrap());// 最後の値を取得する
    println!("get(2)    = {:?}" , slice.get(2).unwrap());// 指定インデックスの値を取得する
    println!("is_empty()= {:?}" , slice.is_empty()); // 空のスライスか検証する
    println!("len()     = {:?}" , slice.len());// 配列の要素数を取得する
}

/// ### 4-7.スライス型
/// #### リスト4-27 ソートメソッド
#[allow(dead_code)]
pub fn methods_2(){
    let mut array = [103 , 101 , 100 , 104 , 102];
    let slice: &mut[i32] = &mut array[..];
    slice.reverse();// 逆順ソートする
    println!("reverse() = {:?}" , slice);
    let slice: &mut[i32] = &mut array[..];
    slice.sort(); // ソートする
    println!("sort()    = {:?}" , slice);

}

/// ### 4-7.スライス型
/// #### リスト4-28 データ変換、加工メソッド
#[allow(dead_code)]
pub fn methods_3(){
    let vec = vec!["abc" , "def" , "hij" , "rst" , "uvw" , "xyz"];
    let slice = &vec[..];
    let chks = slice.chunks(3); // 指定サイズに分割したスライスを返す
    for chk in chks {
        println!("chunks()  = {:?}" , chk);
    }
    let r = slice.join("/");// 指定文字で連結した文字列を返す
    println!("join()    = {:?}" , r);
    let it = slice.iter(); // イテレータを返す
    println!("iter()    = {:?}" , it);
    let v = slice.to_vec(); // ベクタを返す
    println!("to_vec()  = {:?}" , v);
    let array = [100 , 101 , 102 , 103 , 104];
    let slice: &[i32] = &array[..];
    // 4の倍数要素を除外したサブスライスのイテレータを返す
    let spts = slice.split(|value|{
        value % 4 == 0 // 要素は4の倍数か?
    });
    for spt in spts{
        println!("split()   = {:?}" , spt);
    }
}