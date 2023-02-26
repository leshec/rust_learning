//! Simulating  files one step at a time.

/// Represent a "file",
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    /// New files area ssumed to be empty, but a name is required.
    pub fn new(name: &str) -> File {
        return File {
            name: String::from(name),
            data: Vec::new(),
        };
    }

    /// Returns the file's length in bytes.
    pub fn len(&self) -> usize {
        return self.data.len();
    }

    /// Returns the file's length in bytes.
    pub fn name(&self) -> String {
        return self.name.clone();
    }
}

fn main() {
    let f1 = File::new("f1.txt");

    let f1_name = f1.name();
    let f1_length = f1.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
