use flat_common::{error::Error, result::Result};
use std::{collections::HashMap, fs, io::Write, path::Path};

#[derive(Debug, Clone)]
pub struct ShVars(HashMap<String, String>);

impl ShVars {
    
    /// Create a new instance of `ShVars`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Inherit environment variables.
    /// 
    /// # Example
    /// ```
    /// use flat_engine::ShVars;
    /// 
    /// let mut vars = ShVars::new();
    /// 
    /// vars.inherit();
    /// 
    /// vars.print();
    /// ```
    pub fn inherit(&mut self) -> &mut Self {
        let mut vars = HashMap::new();

        for (key, value) in std::env::vars() {
            vars.insert(key, value);
        }

        self
    }

    /// Open a shell variables file.
    pub fn open(path: &Path) -> Self {
        let mut vars = HashMap::new();

        let file = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(err) => {
                panic!("Failed to read file: {}", err);
            }
        };

        for line in file.lines() {
            if line.is_empty() {
                continue;
            }

            let mut parts = line.split('=');

            let key = match parts.next() {
                Some(key) => key,
                None => continue,
            };

            let value = match parts.next() {
                Some(value) => value,
                None => continue,
            };

            vars.insert(
                key.trim_start().trim_end().to_string(),
                value.trim_start().trim_end().to_string(),
            );
        }
        Self(vars)
    }

    /// Save shell variables to a file.
    pub fn save(&self, path: &Path) {
        let mut file = match fs::File::options().create(true).write(true).open(path) {
            Ok(file) => file,
            Err(err) => {
                panic!("Failed to open file: {}", err);
            }
        };

        for (key, value) in &self.0 {
            let line = format!("{}={}\n", key, value);

            file.write_all(line.as_bytes())
                .expect("Failed to write to file");

            file.write(b"\n").expect("Failed to write to file");
        }
    }

    /// Get a shell variable by key.
    /// 
    /// # Example
    /// ```
    /// use flat_engine::ShVars;
    /// 
    /// let mut vars = ShVars::new();
    /// 
    /// vars.set("key", "value");
    /// 
    /// assert_eq!(vars.get("key").unwrap(), "value");
    /// ```
    pub fn get(&self, key: &str) -> Result<&str> {
        self.0
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| Error::DUMMY)
    }

    /// Set a shell variable.
    /// 
    /// # Example
    /// ```
    /// use flat_engine::ShVars;
    /// 
    /// let mut vars = ShVars::new();
    /// 
    /// vars.set("key", "value");
    /// 
    /// assert_eq!(vars.get("key").unwrap(), "value");
    /// ```
    pub fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }

    /// Unset a shell variable.
    /// 
    /// # Example
    /// ```
    /// use flat_engine::ShVars;
    /// 
    /// let mut vars = ShVars::new();
    /// 
    /// vars.set("key", "value");
    /// 
    /// vars.unset("key");
    /// 
    /// assert_eq!(vars.exists("key"), false);
    /// ```
    pub fn unset(&mut self, key: &str) {
        self.0.remove(key);
    }

    /// Check if a shell variable exists.
    /// 
    /// # Example
    /// ```
    /// use flat_engine::ShVars;
    /// 
    /// let mut vars = ShVars::new();
    /// 
    /// vars.set("key", "value");
    /// 
    /// assert_eq!(vars.exists("key"), true);
    /// ```
    pub fn exists(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Get the length of shell variables.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if shell variables are empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clear shell variables.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Get all keys of shell variables.
    pub fn keys(&self) -> Vec<&String> {
        self.0.keys().collect()
    }

    /// Get all values of shell variables.
    pub fn values(&self) -> Vec<&String> {
        self.0.values().collect()
    }

    /// Get all key-value pairs of shell variables.
    pub fn entries(&self) -> Vec<(&String, &String)> {
        self.0.iter().collect()
    }

    /// Print shell variables to stdout.
    pub fn print(&self) {
        let mut stdout = std::io::stdout().lock();

        for (key, value) in &self.0 {
            let line = format!("{}={}\n", key, value);

            stdout
                .write_all(line.as_bytes())
                .expect("Failed to write to stdout");
        }
    }
}

impl From<HashMap<String, String>> for ShVars {
    fn from(vars: HashMap<String, String>) -> Self {
        Self(vars)
    }
}

impl From<HashMap<&str, &str>> for ShVars {
    fn from(vars: HashMap<&str, &str>) -> Self {
        let mut map = HashMap::new();
        for (key, value) in vars {
            map.insert(key.to_string(), value.to_string());
        }
        Self(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sh_vars() {
        let vars = ShVars::new();

        assert_eq!(vars.len(), 0);

        assert_eq!(vars.is_empty(), true);
    }

    #[test]
    fn test_sh_vars_set() {
        let mut vars = ShVars::new();

        for i in 0..10 {
            vars.set(&format!("key{}", i), &format!("value{}", i));
        }

        for i in 0..10 {
            assert_eq!(
                vars.get(&format!("key{}", i)).unwrap(),
                &format!("value{}", i)
            );
        }

        assert_eq!(vars.len(), 10);

        assert_eq!(vars.is_empty(), false);
    }

    #[test]
    fn test_sh_vars_get() {
        let vars = ShVars::from(HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]));

        for i in 1..4 {
            assert_eq!(
                vars.get(&format!("key{}", i)).unwrap(),
                &format!("value{}", i)
            );
        }

        assert_eq!(vars.len(), 3);

        assert_eq!(vars.is_empty(), false);
    }

    #[test]
    fn test_sh_vars_unset() {
        let mut vars = ShVars::from(HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]));

        vars.unset("key1");

        assert_eq!(vars.exists("key1"), false);

        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_sh_vars_clear() {
        let mut vars = ShVars::from(HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]));

        vars.clear();

        assert_eq!(vars.len(), 0);

        assert_eq!(vars.is_empty(), true);
    }
}
