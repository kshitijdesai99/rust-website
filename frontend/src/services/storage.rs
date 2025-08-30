use web_sys::{Storage, window};

pub struct StorageService;

impl StorageService {
    fn get_storage() -> Result<Storage, String> {
        window()
            .ok_or("No window object".to_string())?
            .local_storage()
            .map_err(|_| "Failed to access localStorage".to_string())?
            .ok_or("localStorage is not available".to_string())
    }

    pub fn set_item(key: &str, value: &str) -> Result<(), String> {
        let storage = Self::get_storage()?;
        storage
            .set_item(key, value)
            .map_err(|_| format!("Failed to save item: {}", key))
    }

    pub fn get_item(key: &str) -> Result<Option<String>, String> {
        let storage = Self::get_storage()?;
        storage
            .get_item(key)
            .map_err(|_| format!("Failed to get item: {}", key))
    }

    pub fn remove_item(key: &str) -> Result<(), String> {
        let storage = Self::get_storage()?;
        storage
            .remove_item(key)
            .map_err(|_| format!("Failed to remove item: {}", key))
    }

    pub fn clear() -> Result<(), String> {
        let storage = Self::get_storage()?;
        storage
            .clear()
            .map_err(|_| "Failed to clear storage".to_string())
    }
}
