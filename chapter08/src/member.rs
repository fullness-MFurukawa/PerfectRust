///
/// 8章 構造体のサンプルコード
/// 

/// 8-4.メソッド
/// ## 会員を表す構造体
#[allow(dead_code)]
pub struct Member<'a> {
    id:         u32  ,      //  会員番号を表すフィールド
    name:       &'a str ,   //  氏名を表すフィールド
    address:    &'a str ,   //  住所を表すフィールド
    email:      &'a str     //  メールアドレスを表すフィールド
}
impl<'a> Member<'a> {
    /// インスタンスの生成
    #[allow(dead_code)]
    fn new(id: u32 , name:&'a str , address:&'a str ,email:&'a str) -> Self {
        Self {id , name , address , email}
    }
    /// nameフィールドの値を返すメソッド
    /// # 引数 なし
    /// # 戻り値 nameフィールドの値(クローン)
    #[allow(dead_code)]
    fn get_name(&self) -> &str {
        self.name.clone()
    }
    /// nameフィールドの値を変更する
    /// # 引数 nameフィールドの値
    /// # 戻り値 なし
    #[allow(dead_code)]
    fn set_name(&mut self , name: &'a str) -> (){
        self.name = name;
    }
}   
/// ## 8-4.メソッド
/// ### リスト8-10 ライフタイム注釈
#[allow(dead_code)]
pub fn use_method(){
    // インスタンスの生成
    let mut customer = Member::new(100,"山田太郎", "東京都新宿区", "yamada@sample.com");
    customer.set_name("鈴木花子"); // メソッドでnameフィールドの値を変更する
    println!("{}" , customer.get_name());// メソッド実行結果を出力する
}
