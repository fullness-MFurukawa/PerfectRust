///
/// 13章 テスト 
/// 
use std::fmt::{Debug, Formatter, Display};
/// ## 13-1.テストの基本
/// ### リスト13-1 サンプルターゲット
/// ### サンプルコードで利用するエラー列挙型
#[derive(Debug , Eq , PartialEq)]
pub enum SampleError {
    Msg(String)
}
impl Display for SampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleError::Msg(msg) => write!(f, "{}", msg)
        }
    }
}
/// ## 13-1.テストの基本
/// ### リスト13-1 サンプルターゲット
/// ### 観客を表す構造体
#[derive(Debug , Clone , Eq , PartialEq)]
pub struct Guest{
    age: u32 ,      // 年齢
    campaign: bool  // true:キャンペーン中
}
impl Display for Guest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f , "年齢:{} キャンペーン:{}" , self.age , self.campaign)
    }
}
impl Guest {
    /// ## 13-1.テストの基本
    /// ### リスト13-1 サンプルターゲット
    /// ### イスタンス生成
    pub fn new(_age:u32 , _campaign:bool) -> Self {
        Self {age:_age , campaign:_campaign}
    }
    /// ## 13-1.テストの基本
    /// ### リスト13-1 サンプルターゲット
    /// ### 観覧金額を計算する
    /// ### 戻り値: Result<u32,SampleResult>　
    pub fn calc_fee(self) -> Result<u32 , SampleError> {
        let fee = match self.age {
            0..=4    => 0    , // 0～４歳は無料
            5..=12   => 500  , // 5～12歳は500円
            13..=17  => 700  , // 13～17歳は700円
            18..=64  => 1000 , // 18～64歳は1000円
            65..=120 => 600  , // 65～120歳は600円
            // それ以外の年齢の場合はエラーにする
            _ => return Err(SampleError::Msg(String::from("年齢が不正です。")))
        };
        Ok(self.calc_campaign_fee(fee))
    }
    /// ## 13-1.テストの基本
    /// ### リスト13-1 サンプルターゲット
    //  #### キャンペーン中であれば10%引きにする
    fn calc_campaign_fee(&self ,mut fee: u32) -> u32 {
        if self.campaign && fee != 0 {
            fee = fee * 90 / 100;
        }
        fee
    }
}


#[cfg(test)]
mod tests { 
    use super::*;
    /// ## 13-3.テストドライバの実装
    /// ### リスト13-4 aseert!()マクロの利用
    /// ### 年齢が10歳でキャンペーンでなければ500円を返すことを検証する
    #[test]
    fn calc_fee_case_01() {
        let  guest = Guest::new(10 , false); // インスタンスを生成する
        let result = guest.clone().calc_fee().unwrap(); // 入園料を取得する
        assert!(result  == 500 , "{}" , &guest); // 結果が500円であることを検証する
    }
    /// ## 13-3.テストドライバの実装
    /// ### リスト13-4 aseert!()マクロの利用
    /// ### 年齢が10歳でキャンペーンの場合は450円を返すことを検証する
    #[test]     
    fn calc_fee_case_campaign_01() {
        let  guest = Guest::new(10 , true); // インスタンスを生成する
        let result = guest.clone().calc_fee().unwrap(); // 入園料を検索する
        assert!(result == 450 , "{}" , &guest); // 結果が450円であることを検証する
    }

    /// ## 13-3.テストドライバの実装
    /// ### リスト13-5 aseert_eq!()マクロの利用
    /// ### 年齢が15歳でキャンペーンでなければ700円を返すことを検証する
    #[test]
    fn calc_fee_case_02() {
        let  guest = Guest::new(15 , false); // インスタンスを生成する
        let result = guest.clone().calc_fee().unwrap(); // 入園料を取得する
        assert_eq!(700 , result , "{}" , &guest); // 結果が700円であることを検証する
    }
    /// ## 13-3.テストドライバの実装
    /// ### リスト13-4 aseert!()マクロの利用
    /// ### 年齢が15歳でキャンペーンの場合は630円を返すことを検証する
    #[test]     
    fn calc_fee_case_campaign_02() {
        let  guest = Guest::new(15 , true); // インスタンスを生成する
        let result = guest.clone().calc_fee().unwrap(); // 入園料を検索する
        assert_eq!(630 , result , "{}" , &guest); // 結果が630円であることを検証する
    }

    /// ## 13-3.テストドライバの実装
    /// ### リスト13-6 aseert_ne!()マクロの利用
    /// ### 年齢が18歳でキャンペーンでなければ700円を返さないことを検証する
    #[test]
    fn calc_fee_case_03() {
        let  guest = Guest::new(18 , false); // インスタンスを生成する
        let result = guest.clone().calc_fee().unwrap(); // 入園料を取得する
        assert_ne!(700 , result , "{}" , &guest); // 結果が700円ではないことを検証する
    }
    /// ## 13-3.テストドライバの実装
    /// ### リスト13-6 aseert!()マクロの利用
    /// ### 年齢が15歳でキャンペーンの場合は630円を返さないことを検証する
    #[test]     
    fn calc_fee_case_campaign_03() {
        let  guest = Guest::new(15 , true); // インスタンスを生成する
        let result = guest.clone().calc_fee().unwrap(); // 入園料を検索する
        assert_eq!(630 , result , "{}" , &guest); // 結果が630円ではないことを検証する
    }

    /// ## 13-3.テストドライバの実装
    /// ### リスト13-7 エラーを検証する
    /// ### 年齢が125歳でSampleErrorが返されることを検証する
    #[test]
    fn calc_fee_case_wrong_age() {
        let  guest = Guest::new(125 , false); // インスタンスを生成する
        let result = guest.clone().calc_fee().unwrap_err(); // 入園料を取得する
        // 想定されるエラーを生成する
        let expected_error = SampleError::Msg(String::from("年齢が不正です。"));
        assert_eq!(expected_error , result , "{}" , &guest); // SampleErrorであることを検証する
    }

    /// ## 13-3.テストドライバの実装
    /// ### リスト13-9 #[should_panic]アトリビュートの利用
    /// ### 年齢が125歳でSampleErrorが返されることを検証する
    #[test]
    #[should_panic(expected="不正な年齢が指定されました。")]
    fn calc_fee_case_should_panic() {
        let  guest = Guest::new(125 , false); // インスタンスを生成する
        match guest.calc_fee(){
            Ok(result) =>  assert_ne!(700 , result ) ,
            Err(_) => panic!() // エラーならパニックにする
        }
    }
    
    /// ## 13-3.テストドライバの実装
    /// ### リスト13-8 非公開メソッドのテスト
    /// ### 金額が1000円の場合の値引額900円が返されることを検証する
    #[test]
    fn calc_campaign_fee_case_01() {
        let guest = Guest::new(0 , true); // インスタンスを生成する
        let result = guest.calc_campaign_fee(1000);
        assert_eq!(900 , result); // 金額が900円であることを検証する
    }

    /// ## 13-3.テストドライバの実装
    /// ### リスト13-8 非公開メソッドのテスト
    /// ### 金額が1000円の場合の値引額900円が返されることを検証する
    #[test]
    fn use_debu_assert_eq() {
        let guest = Guest::new(0 , true); // インスタンスを生成する
        let result = guest.calc_campaign_fee(1000);
        debug_assert_eq!(900 , result); // 金額が900円であることを検証する
    }

    /// ## 13-3.テストドライバの実装
    /// ### リスト13-10 dbg!()マクロの利用
    #[test]
    fn use_dbg() {
        let guest = Guest::new(0 , true); 
        dbg!(&guest);// dbg!()マクロでGusetの内容を出力する
        let result = guest.calc_campaign_fee(1000);
        assert_eq!(900 , result);
    }

}

