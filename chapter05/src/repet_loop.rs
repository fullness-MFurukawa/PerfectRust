//!
//! 5章制御式 サンプルプログラム
//!

/// ### 5-5.無限ループ
/// #### リスト5-14 キー入力された駅名を返す
#[allow(dead_code)]
fn enter_station() -> String {
    println!("駅名を入力してください");
    let mut input = String::new(); // ミュータブルなStringを生成する
    let _x = std::io::stdin().read_line(&mut input); // キーボード入力された値を取得する
    input.trim().to_owned() // トリミングした結果を返す
}
/// ### 5-5.無限ループ
/// #### リスト5-14 loop式の利用
#[allow(dead_code)]
pub fn loop_1(){
    // 駅名の配列
    let stations =  ["品川" , "大崎" , "五反田" , "目黒" , "恵比寿" , "渋谷"];
    loop{ // loop式
        let station = enter_station(); // キー入力された駅名を取得
        if station.eq("end"){ // loop式を抜ける
            println!("終了しました。");
            break;
        }
        if ! stations.contains(&station.as_str()) { // 入力された駅名の存在しない?
            println!("駅名:{}は存在しません。" , &station);
            continue;
        }
        let mut count = 1;
        for s in stations { 
            if s.ne(&station){ // 入力された駅名と異なる?
                count += 1;
                continue;
            }else{ // 入力された駅名と同じ
                break;
            }
        }
        println!("駅名:{}は{}番目の駅です。" , &station , count);
    }
}