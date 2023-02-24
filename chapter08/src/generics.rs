///
/// 8章 構造体のサンプルコード
/// 

/// 8-6.ジェネリクス
/// ## 顧客を表す構造体
#[allow(dead_code)]
#[derive(Debug,Clone)]
struct Customer<T> {
    id: T ,    //  顧客番号を表すフィールド
    name: String ,  //  氏名を表すフィールド
    address: String,//  住所を表すフィールド
    email: String   //  メールアドレスを表すフィールド
}
impl<T> Customer<T>{ // Customer構造体の実装ブロック
    /// ## 8-6.ジェネリクス
    /// ### リスト8-17 ジェネリック型の利用
    /// ### 引数 顧客番号:id,氏名:name,住所:address,メールアドレス:email
    #[allow(dead_code)]
    fn new(id: T , name: String , address: String , email: String) -> Self {
        Self { id , name , address , email }
    }

    /// ## 8-6.ジェネリクス
    /// ### リスト8-18 idの値を変更するメソッド
    #[allow(dead_code)]
    fn change_id(&mut self , value: T) {
        self.id = value;
    }
}

/// ## 8-6.ジェネリクス
/// ### リスト8-18 インスタンス生成
#[allow(dead_code)]
pub fn use_new() {
    // idフィールドをu64型にしたCustomerの生成
    let customer = Customer::<u64>::new(100,String::from("山田太郎"),
        String::from("東京都新宿区"),String::from("yamada@sample.com"));
    println!("{:?}" , customer);
}

/// ## 8-6.ジェネリクス
/// ### リスト8-19 メソッド
#[allow(dead_code)]
pub fn use_change_id() {
    // idフィールドをu64型にしたCustomerの生成
    let mut customer = Customer::<u64>::new(100,String::from("山田太郎"),
        String::from("東京都新宿区"),String::from("yamada@sample.com"));
    customer.change_id(200);
    println!("{:?}" , customer);
}