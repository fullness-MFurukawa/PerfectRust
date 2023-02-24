/// ## 13-6.テストの分離
/// ### リスト13-16 分離したテストモジュール
use chapter13::target::{Guest,SampleError};
use simple_test_case::test_case;


///
/// 正常系のテスト
///
#[test_case(0 ,  false , 0 ; "case01 agr:0 campaign:false expected:0")]
#[test_case(0 ,  true ,  0 ; "case02 agr:0 campaign:true expected:0")]
#[test_case(4 ,  false , 0 ; "case03 agr:4 campaign:false expected:0")]
#[test_case(4 ,  true ,  0 ; "case04 agr:0 campaign:false expected:0")]
#[test_case(5 ,  false , 500 ; "case05 agr:5 campaign:false expected:500")]
#[test_case(5 ,  true ,  450 ; "case06 agr:0 campaign:true expected:450")]
#[test_case(12 , false , 500 ; "case07 agr:12 campaign:false expected:500")]
#[test_case(12 , true ,  450 ; "case08 agr:12 campaign:true expected:450")]
#[test]
/// ## 13-5.外部クレートの利用
/// ### リスト13-12 #[test_case]アトリビュートの利用 
/// ### 入場料金計算テストケース01～ 08
/// ### 引数 age:年齢
/// ### 引数 campaign:キャンペーン
/// ### 引数 expected:実行結果
fn calc_fee_test01_08(age: u32 , campaign: bool , expected: u32) {
    let guest = Guest::new(age, campaign);
    assert_eq!( guest.calc_fee().unwrap() , expected);
}
///
/// エラー系のテスト
/// 
#[test_case(121,  false ; "case09 arg:121 campaign:false")]
#[test_case(121 , true  ; "case10 arg:121 campaign:true")]
#[test]
/// ## 13-5.外部クレートの利用
/// ### リスト13-12 #[test_case]アトリビュートの利用 
/// ### 入場料金計算テストケース09～ 10
/// ### 引数 age:年齢
/// ### 引数 campaign:キャンペーン
///  入場料金計算テストケース09～ 10
pub fn calc_fee_test09_10(age: u32,campaign: bool) {
    let expected = 
    SampleError::Msg(String::from("年齢が不正です。"));
    let guest = Guest::new(age, campaign);
    assert_eq!( guest.calc_fee().err().unwrap() , expected);
}
