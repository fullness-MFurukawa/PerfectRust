///
/// 8章 構造体のサンプルコード
/// 


/// ## 8-1.名前付きフィールド型
/// ### リスト8-1 顧客を表す構造体
#[allow(dead_code)]
struct Customer {
    id: u32 ,       //  顧客番号を表すフィールド
    name: String ,  //  氏名を表すフィールド
    address: String,//  住所を表すフィールド
    email: String   //  メールアドレスを表すフィールド
}

/// ## 8-1.名前付きフィールド型
/// ### リスト8-2 インスタンスの生成    
#[allow(dead_code)]
pub fn generate(){
    let customer = Customer { // Customer構造体のインスタンス生成とフィールドに初期値設定する
        id: 100,
        name: String::from("山田太郎"),
        address: String::from("東京都新宿区"),
        email: String::from("yamada@sample.com")
    };
    // フィールドの内容を出力する
    println!("id = {}" ,customer.id);
    println!("name = {}" ,customer.name);
    println!("address = {}" , customer.address);
    println!("email = {}" , customer.email);
}

/// ## 8-1.名前付きフィールド型
/// ### リスト8-3 参照型フィールド
#[allow(dead_code)]
pub struct Member<'a> {
    id:         u32  ,      //  会員番号を表すフィールド
    name:       &'a str ,   //  氏名を表すフィールド
    address:    &'a str ,   //  住所を表すフィールド
    email:      &'a str     //  メールアドレスを表すフィールド
}

