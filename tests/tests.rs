#[cfg(test)]
mod cli {
    use assert_cmd::Command;
    use std::collections::HashMap;
    use std::fs;
    use std::io;
    use std::path::Path;
    use tempfile::TempDir;

    /// Create a re-name command for testing.
    fn bin() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Unable to create command.")
    }

    /// Create a test directory containing source files.
    fn dir<'a, I>(files: I) -> Result<TempDir, io::Error>
    where
        I: IntoIterator<Item = &'a &'a str>,
    {
        let temp_dir = tempfile::Builder::new().prefix("test_re-name").tempdir()?;
        let root = temp_dir.path();

        for file in files {
            fs::write(root.join(file), file)?;
        }

        Ok(temp_dir)
    }

    /// Assert that all source files exist and contain their name.
    fn assert_files(root: &Path, map: &HashMap<&str, &str>) {
        for (&from, _) in map {
            assert!(from == fs::read_to_string(root.join(from)).expect("Unable to read file."));
        }
    }

    /// Assert that all destination files exist and contain their source name.
    fn assert_moves(root: &Path, map: &HashMap<&str, &str>) {
        for (&from, &to) in map {
            assert!(from == fs::read_to_string(root.join(to)).expect("Unable to read file."));
        }
    }

    /// A standard invocation of re-name which is expected to succeed and move files accordingto the file_map.
    fn standard_test(source: &str, destination: &str, flags: &[&str], file_map: HashMap<&str, &str>) {
        let d = dir(file_map.keys()).expect("Unable to create files.");
        assert_files(d.path(), &file_map);
        bin()
            .args([source, destination])
            .args(flags)
            .current_dir(d.path())
            .assert()
            .success();
        assert_moves(d.path(), &file_map);
    }

    #[test]
    fn no_move() {
        let source = "z";
        let destination = "z1";
        let flags: [&str; 0] = [];
        let file_map = HashMap::from([("a", "a"), ("b", "b"), ("c", "c"), ("d", "d")]);

        standard_test(source, destination, &flags, file_map);
    }

    #[test]
    fn plain_text_move() {
        let source = "a";
        let destination = "a1";
        let flags: [&str; 0] = [];
        let file_map = HashMap::from([("a", "a1"), ("b", "b"), ("c", "c"), ("d", "d")]);

        standard_test(source, destination, &flags, file_map);
    }

    #[test]
    fn escaped_plain_text_move() {
        let source = "a\\.rs";
        let destination = "a1.rs";
        let flags: [&str; 0] = [];
        let file_map = HashMap::from([("a.rs", "a1.rs"), ("b.rs", "b.rs"), ("c.rs", "c.rs"), ("d.rs", "d.rs")]);

        standard_test(source, destination, &flags, file_map);
    }

    #[test]
    fn simple_regex_move() {
        let source = "(\\w)\\.rs";
        let destination = "$1.rr";
        let flags: [&str; 0] = [];
        let file_map = HashMap::from([("a.rs", "a.rr"), ("b.rs", "b.rr"), ("cc.rs", "cc.rs"), ("dd.rs", "dd.rs")]);

        standard_test(source, destination, &flags, file_map);
    }
}
