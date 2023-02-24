pub mod domain;
pub mod infrastructure;
pub mod application;
pub mod view_commons;
pub mod error;
use error::AppError;

/// ## 18-5 アプリケーションの構成
/// ### リスト18-32 Result型
pub type Result<T> = anyhow::Result<T , AppError>;

/* 
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
