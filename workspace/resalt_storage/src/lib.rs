mod interface;
mod util;

pub use interface::*;
pub use util::*;

pub struct StorageCloneWrapper {
    pub storage: Box<dyn StorageImpl>,
}

impl Clone for StorageCloneWrapper {
    fn clone(&self) -> Self {
        StorageCloneWrapper {
            storage: self.storage.clone(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
