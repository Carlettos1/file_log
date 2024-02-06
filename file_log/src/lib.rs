use std::{
    env,
    fmt::Debug,
    fs,
    io::{self, Write},
};

use once_cell::sync::Lazy;

///
/// pub for access in log! macro
#[doc(hidden)]
pub static LOGGER: Lazy<Index> = Lazy::new(Index::default);

#[macro_export]
macro_rules! log {
    ($name:tt, $ext:tt, $($arg:tt)*) => {
        file_log::LOGGER.write_log($name, $ext, format!($($arg)*)).unwrap();
    };
    ($name:tt, $($arg:tt)*) => {
        file_log::LOGGER.write_log($name, "log", format!($($arg)*)).unwrap();
    };
}

const INDEX: &str = "log_index";
const FILE_LOG_INDEX_ENV_VAR: &str = "FILE_LOG_INDEX";

#[doc(hidden)]
#[derive(Debug)]
pub struct Index(usize);

/// Represents an index used for file logging.
///
/// The `Index` struct provides methods for managing and manipulating the index value.
impl Index {
    /// Increments the index value by 1.
    pub fn next(&mut self) {
        self.0 += 1;
    }

    /// Retrieves the index value from the environment variable, if it exists.
    /// If the environment variable is not set, it retrieves the index value from the index file.
    ///
    /// # Returns
    ///
    /// The `Index` value obtained from the environment or index file.
    fn get() -> Index {
        match env::var(FILE_LOG_INDEX_ENV_VAR) {
            // There is an env, prioritize its value
            Ok(index) => Index(index.parse().unwrap_or_default()),
            // There is no env, create use index_file
            Err(_) => Index(
                fs::read_to_string(INDEX)
                    .map(|i| i.parse().unwrap_or_default())
                    .unwrap_or_default(),
            ),
        }
    }

    ///
    /// Save the index to the index file
    fn save(&self) {
        fs::write(INDEX, format!("{}", self.0)).unwrap();
    }

    /// Returns a copy of the index value.
    ///
    /// # Returns
    ///
    /// The index value.
    pub fn index(&self) -> usize {
        self.0
    }

    /// Writes a log entry to a file with the specified extension.
    ///
    /// # Parameters
    ///
    /// - `log`: The name of the log file.
    /// - `extension`: The file extension.
    /// - `data`: The data to be written to the file.
    ///
    /// # Returns
    ///
    /// An `io::Result` indicating the success or failure of the write operation.
    pub fn write_log<C: AsRef<[u8]>>(&self, log: &str, extension: &str, data: C) -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{log}_{}.{extension}", self.0))?;
        file.write_all(data.as_ref())?;
        file.write_all("\n".as_bytes())
    }
}

impl Default for Index {
    fn default() -> Self {
        // Get the index from the env, if it exists, else get it from the index file.
        let mut index = Index::get();
        // Increment the index
        index.next();
        // If the env exists, doesn't make sense to save the index to the index file
        if env::var(FILE_LOG_INDEX_ENV_VAR).is_err() {
            // Save the index to the index file
            index.save();
        }
        index
    }
}

pub fn index() -> usize {
    LOGGER.index()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_next() {
        let mut index = Index::default();
        let initial_value = index.index();
        index.next();
        assert_eq!(index.index(), initial_value + 1);
    }

    #[test]
    fn test_index_write_log() {
        let index = Index::default();
        let log_name = "test_log";
        let extension = "txt";
        let data = "Test log data";
        let result = index.write_log(log_name, extension, data);
        assert!(result.is_ok());

        // Verify that the log file was created
        let file_path = format!(
            "{log}_{}.{extension}",
            index.index(),
            log = log_name,
            extension = extension
        );
        assert!(fs::metadata(&file_path).is_ok());

        // Clean up the log file
        fs::remove_file(file_path).unwrap();
    }
}
