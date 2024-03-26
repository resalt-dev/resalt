use log::debug;
use resalt_models::*;
use std::io::{Read, Write};

/// Dev storage which saves everything to filesystem instead of using a database
/// NOTE! NOT MEANT FOR PRODUCTION!!!
///
/// Structure:
/// e.g. PATH=./files
///

#[derive(Clone)]
pub struct StorageFiles {
    path: String, // MUST BE WITHOUT TRAILING SLASH
}

impl StorageFiles {
    pub fn connect(path: &str) -> Result<StorageFiles, String> {
        let path = path.trim_end_matches('/');
        let storage = StorageFiles {
            path: path.to_string(),
        };

        Ok(storage)
    }

    fn save_file_string(&self, path: &str, data: &str) -> Result<(), String> {
        let path = format!("{}/{}.data", self.path, path);

        // Create parent folders if does not exist
        let parts = path.split('/').collect::<Vec<&str>>();
        let mut w = String::new();
        for part in parts.iter().take(parts.len() - 1) {
            w.push_str(part);
            w.push('/');
            // Create folder if it doesn't exist
            let exists = std::path::Path::new(&w).exists();
            if !exists {
                debug!("Creating folder: {}", w);
                std::fs::create_dir(&w).map_err(|e| format!("{:?}", e))?;
            }
        }

        // Write file
        // debug!("Writing file: {}", path);
        let mut file = std::fs::File::create(path.clone()).map_err(|e| format!("{:?}", e))?;
        file.write_all(data.as_bytes())
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn load_file_string(&self, path: &str) -> Result<String, String> {
        let path = format!("{}/{}.data", self.path, path);
        let mut file = std::fs::File::open(path).map_err(|e| format!("{:?}", e))?;
        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|e| format!("{:?}", e))?;
        Ok(data)
    }

    fn check_file_exists(&self, path: &str) -> Result<bool, String> {
        let path = format!("{}/{}.data", self.path, path);
        // debug!("Checking if file exists: {}", path);
        let exists = std::path::Path::new(&path).exists();
        Ok(exists)
    }

    fn list_file_names(&self, path: &str) -> Result<Vec<String>, String> {
        let path = format!("{}/{}", self.path, path);
        let mut file_names: Vec<String> = Vec::new();
        let entries = match std::fs::read_dir(path).map_err(|e| format!("{:?}", e)) {
            Ok(entries) => entries,
            Err(_) => {
                return Ok(Vec::new());
            }
        };
        for entry in entries {
            let entry = entry.map_err(|e| format!("{:?}", e))?;
            let file_name = entry
                .file_name()
                .into_string()
                .map_err(|e| format!("{:?}", e))?;
            let file_name = file_name.trim_end_matches(".data").to_string();
            file_names.push(file_name);
        }
        Ok(file_names)
    }

    fn delete_file(&self, path: &str) -> Result<(), String> {
        let path = format!("{}/{}.data", self.path, path);
        std::fs::remove_file(path).map_err(|e| format!("{:?}", e))?;
        Ok(())
    }
}

impl StorageImpl for StorageFiles {
    fn clone(&self) -> Box<dyn StorageImpl> {
        Box::new(Clone::clone(self))
    }

    fn get(&self, key: &str) -> Result<Option<String>, String> {
        let path = format!("kv/{}", key);
        let exists = self.check_file_exists(&path)?;
        if !exists {
            return Ok(None);
        }

        let value: String = self.load_file_string(&path)?;
        Ok(Some(value))
    }

    fn set(&self, key: &str, value: &str) -> Result<(), String> {
        let path = format!("kv/{}", key);
        self.save_file_string(&path, value)?;
        Ok(())
    }

    fn del(&self, key: &str) -> Result<(), String> {
        let path = format!("kv/{}", key);
        self.delete_file(&path)?;
        Ok(())
    }

    fn keys(&self, pattern: &str) -> Result<Vec<String>, String> {
        let keys = self.list_file_names("kv")?;
        let pattern = pattern.replace("*", ".*");
        let regex = regex::Regex::new(&pattern).map_err(|e| format!("{:?}", e))?;
        let keys = keys
            .into_iter()
            .filter(|key| regex.is_match(key))
            .collect::<Vec<String>>();
        Ok(keys)
    }
}

// #[cfg(test)]
// mod tests {
//     use resalt_models::storage::{test_storage_impl_authtoken, test_storage_impl_users};

//     use crate::StorageFiles;

//     fn get_temp_storage() -> (StorageFiles, String) {
//         let random_path_under_tmp = std::env::temp_dir()
//             .join(format!("resalt-files-test-{}", uuid::Uuid::new_v4()))
//             .to_str()
//             .unwrap()
//             .to_string();

//         (
//             StorageFiles::connect(&random_path_under_tmp).unwrap(),
//             random_path_under_tmp,
//         )
//     }

//     fn cleanup_temp_storage(path: &str) {
//         std::fs::remove_dir_all(path).unwrap();
//     }

//     #[test]
//     fn test_users() {
//         let data = get_temp_storage();
//         test_storage_impl_users(&data.0);
//         cleanup_temp_storage(&data.1);
//     }

//     #[test]
//     fn test_authtokens() {
//         let data = get_temp_storage();
//         test_storage_impl_authtoken(&data.0);
//         cleanup_temp_storage(&data.1);
//     }
// }
