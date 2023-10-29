#[cfg(test)]
mod tests {
    use IDP2_yuhan_rust::{load, read, create, update, delete};

    use std::fs;
    use std::path::Path;

    const TEST_DB_FILE: &str = "diabetes.db";
    const TEST_CSV_FILE: &str = "diabetes.csv";
    #[test]
    pub fn test_load() {
        if Path::new(TEST_DB_FILE).exists() {
            fs::remove_file(TEST_DB_FILE).expect("Failed to remove test database file");
        }

        let result = load(TEST_CSV_FILE);
        assert!(result.is_ok(), "test_load failed: {:?}", result);
        assert!(Path::new(TEST_DB_FILE).exists());
    }

    #[test]
    pub fn test_read() {
        assert!(read().is_ok());
    }

    #[test]
    pub fn test_create() {
        assert!(create(4, 94, 78, 31, 85, 33.1, 0.446, 22, 1).is_ok());
    }

    #[test]
    pub fn test_update() {
        let result = update(1, 4, 94, 78, 31, 85, 33.1, 0.446, 22, 1);
        assert!(result.is_ok(), "test_update failed: {:?}", result);
    }

    #[test]
    pub fn test_delete() {
        assert!(delete(10).is_ok());
    }
}

fn main(){
    tests::test_load();
    tests::test_read();
    tests::test_create();
    tests::test_update();
    tests::test_delete();
}