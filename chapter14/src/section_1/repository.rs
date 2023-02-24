//!
//! 14章 PostgreSQL
//! 

use anyhow::Result;
/// ## 14-4.CRUD操作
/// ### リスト14-7 CRUD操作メソッドを定義したトレイト
pub trait Repository<T , PK , UPD> {
    // 全件取得する
    fn select_all(&mut self) -> Result<Vec<T>>{ 
        todo!() 
    }
    // 指定された主キーに一致するレコードを取得する
    fn select_by_id(&mut self, _id: PK) -> Result<T>{ 
        todo!() 
    }
    // 新しいレコードを追加する
    fn insert(&mut self, _row: T) -> Result<UPD>{ 
        todo!() 
    }
    // 指定された主キー値に一致するレコードを更新する
    fn update_by_id(&mut self , _id: PK) -> Result<UPD>{ 
        todo!() 
    }
    // 指定された主キー値に一致するレコードを削除する
    fn delete_by_id(&mut self , _id: PK) -> Result<UPD>{ 
        todo!() 
    }
}
