//!
//! 6章 ライブラリ型 サンプルプログラム
//! 

/// 6-1.String型
/// リスト6-1 インスタンスの生成
#[allow(dead_code)]
pub fn instantiate(){
    let s1 = String::new(); // 空のインスタンスを生成する
    println!("new()={:?},len={},capacity={}" , &s1 , &s1.len() , &s1.capacity());
    let s2 = String::from("Hello Rust."); // 指定文字列からインスタンスを生成する
    println!("from()={:?},len = {},capacity= {}" , &s2 , &s2.len() , &s2.capacity());
    let s3 = String::with_capacity(5); // 容量を指定してインスタンスを生成する
    println!("with_capacity(20)={:?},len={},capacity={}" , &s3 , &s3.len() , &s3.capacity());
}
/// 6-1.String型
/// リスト6-2 文字、文字列の追加する
#[allow(dead_code)]
pub fn add(){
    let mut s = String::from("abc");
    s.push('d'); // 文字をを末尾に追加する
    println!("push()    =   {:?}" , &s);
    s.insert(1,'d');// 指定の位置に文字を追加する
    println!("insert()  =   {:?}" , &s);

    let mut s = String::from("山田太郎");
    s.push_str(",東京都新宿区");// 末尾に文字列を追加する
    println!("push_str()    = {:?}" , &s);
    let mut s = String::from("山田太郎");
    s.insert_str(12,",東京都新宿区");// 指定の位置に文字列を追加する
    println!("insert_str()  = {:?}" , &s);
}

/// 6-1.String型
/// リスト6-3 文字、文字列を置換する
#[allow(dead_code)]
pub fn replace(){
    let s = String::from("山田太郎 , 山崎花子");
    let r = &s.replace("山", "吉"); // 「山」の部分を「吉」に置換する
    println!("replace()        = {:?}" , &r);
    
    let s = String::from("山田太郎 , 山崎花子");
    let r = &s.replacen("山", "吉" , 1); // 最初の「山」を「吉」に置換する
    println!("replacen()       = {:?}" , &r);
    
    let mut s = String::from("山田太郎");
    let offset = &s.find("太").unwrap_or(s.len()); // 「太」までの位置を取得する
    s.replace_range(..offset, "鈴木"); //  先頭から「鈴木」に置換する
    println!("replace_range()  = {:?}" , &s);
}
/// 6-1.String型
/// リスト6-4 文字、文字列を削除する
#[allow(dead_code)]
pub fn remove(){
    let mut s = String::from("abcdefgxyz");
    s.clear(); // すべての文字をクリアする
    println!("値 = {:?} , 容量={:?}" , &s , &s.capacity());
    let mut s = String::from("abcdefgxyz");
    let r = &s.drain(1..3); // 必要な部分文字以外切り捨てる
    println!("r = {:?}" , &r.as_str());
    let mut s = String::from("abcdefgxyz");
    let r = &s.pop();// 最後の文字を削除する
    println!("s = {:?} , r = {:?}" , &s , &r.unwrap()); 
    let mut s = String::from("abcdefgxyz");
    let r = s.remove(3);// 指定位置の文字を削除する
    println!("s = {:?} , r = {:?}" , &s , &r);
}
/// 6-1.String型
/// リスト6-5 検索
#[allow(dead_code)]
pub fn find(){
    // 結果を出力するクロージャ
    let find_result= |ret:Option <usize> , method_name: &str|{
        if ret.is_none(){
            println!("{} = パターンが見つかりません。", &method_name);
        }else{
            println!("{} = {}番目で見つかりました。" , &method_name , &ret.unwrap());
        }
        ret
    };
    let s = String::from("abcdefgxyz_xyz");
    //文字列の先頭から検索した結果を出力する
    find_result(s.find("xyz"),"find()");
    // 文字列の最後から検索した結果を出力する
    find_result(s.rfind("xyz"),"rfind()");
}

/// 6-1.String型
/// リスト6-6 マッチング
#[allow(dead_code)]
pub fn matching(){
    let s = String::from("abcdefgxyz_xyz");
    // パターンに一致した文字列を取得する
    let r: Vec<_> = s.matches("xyz").collect();
    if r.is_empty() {
        println!("maths() = パターンが見つかりません。");
    }else{
        for value in r{
            println!("maths() = {:?}" , &value);
        }
    }

    // パターンに一致した文字列と位置をタプルで取得する
    let r:Vec<_> = s.match_indices("hij").collect();
    if r.is_empty() {
        println!("match_indices() = パターンが見つかりません。");
    }else{
        for value in r{
            println!("match_indices() = {},{}" , &value.0 , &value.1);
        }
    }
}

/// 6-1.String型
/// リスト6-7 文字、文字列を取得する
#[allow(dead_code)]
pub fn get(){
    let s = String::from("abcdefgxyz");
    let r = &s.get(0..3); // 先頭から3文字取得する
    println!("get(0..3) = {:?}" , &r.unwrap());
    let mut s = String::from("abcdefgxyz");
    let r = s.get_mut(0..5); // 先頭から5文字取得する
    println!("get_mut(0..5) = {:?}" , &r.unwrap());
}