///
/// 18章 外部クレート活用
/// 
use crate::Result;
use super::entity::Entity;
/// ## 18-5 アプリケーションの構成
/// ### リスト18-21 データ変換機能を表すトレイト
pub trait Converter<E , M> where E:Entity {
    // Entityから他のモデルへ変換する
    fn from_entity(entity: &E) -> Result<M>;
    // 他のモデルからEntityへ変換する
    fn from_model(model: &M) -> Result<E>;  
}