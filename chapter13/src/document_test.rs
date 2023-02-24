///
/// 13章 テスト 
///

use crate::target::Guest;
///
/// ## 13-4.ドキュメントテスト
/// ### リスト13-10 ドキュメントテストの記述
/// ### 年齢が10歳でキャンペーンでなければ500円を返すことを検証する
/// ```
/// use chapter13::document_test::calc_fee_case_01;
/// let result = calc_fee_case_01();
/// assert_eq!(500 , result);
/// ```
#[allow(dead_code)]
pub fn calc_fee_case_01() -> u32 {
    let  guest = Guest::new(10 , false); // インスタンスを生成する
    let result = guest.clone().calc_fee().unwrap(); // 入園料を取得する
    result
}


