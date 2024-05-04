use std::{
    fs,
    path::{Path, PathBuf},
};

/// Gather the data from all the
/// member toml files and
/// put them into a singular file for
/// easier parsing by the page templates.
pub fn collect_member_files(folder_path: &Path) -> anyhow::Result<String> {
    // Guard clause to ensure
    // that it is a proper path
    // and that is also not a
    // broken symlink.
    if !folder_path.try_exists()? {
        anyhow::bail!("The path ({folder_path:?}) is invalid");
    }

    let mut member_file_paths: Vec<PathBuf> = vec![];

    for entry in folder_path.read_dir()? {
        let entry = entry?;

        if entry.file_type()?.is_dir() || if let Some(extension) = entry.path().extension() {
            extension != "toml"
        } else { true } {
            continue;
        }

        member_file_paths.push(entry.path());
    }

    let mut collected_members: String = String::new();

    for member_file_path in member_file_paths {
        let member_file_content = fs::read_to_string(member_file_path)?;
        collected_members.push_str("[[members]]");
        collected_members.push('\n');
        collected_members.push_str(&member_file_content);
    }

    Ok(collected_members)
}

/// Copies files with picture related extensions
/// to the new folder so that users' profile pictures
/// are available for the community template to use.
pub fn copy_profile_pictures(origin_folder: &Path, destination_folder: &Path) -> anyhow::Result<()> {

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn converts_organization_members() {
        let test_folder_path = Path::new("test_folder");
        let example_member_one = r#"name = "example"
bio = "example stuff"
sponsor = "sponsor_link.example"
github = "ExampleOne"
"#;
        let example_member_two = r#"name = "example_two"
bio = "example stuff two"
sponsor = "sponsor_link.example"
github = "ExampleTwo"
"#;
        let expected = r#"[[members]]
name = "example"
bio = "example stuff"
sponsor = "sponsor_link.example"
github = "ExampleOne"
[[members]]
name = "example_two"
bio = "example stuff two"
sponsor = "sponsor_link.example"
github = "ExampleTwo"
"#;
        fs::create_dir_all(test_folder_path).unwrap();
        fs::write(test_folder_path.join("test_one.toml"), example_member_one).unwrap();
        fs::write(test_folder_path.join("test_two.toml"), example_member_two).unwrap();

        let result = collect_member_files(test_folder_path);

        // Clean up the test folder.
        // This happens before the assert statement
        // to ensure things are cleaned up
        // even on (most) failing tests.
        fs::remove_dir_all(test_folder_path).unwrap();
        assert_eq!(result.unwrap(), expected.to_string());
    }

    #[test]
    fn only_parses_toml_files() {
        let test_folder_path = Path::new("parsing_test_folder");
        let example_member = r#"name = "example"
bio = "example stuff"
sponsor = "sponsor_link.example"
github = "ExampleOne"
"#;
        let fake_file = r#"THIS SHOULDN'T BE PARSED!!!"#;
        let expected = r#"[[members]]
name = "example"
bio = "example stuff"
sponsor = "sponsor_link.example"
github = "ExampleOne"
"#;
        fs::create_dir_all(test_folder_path).unwrap();
        fs::write(test_folder_path.join("test.toml"), example_member).unwrap();
        fs::write(test_folder_path.join("fake.svg"), fake_file).unwrap();

        let result = collect_member_files(test_folder_path);

        // Clean up the test folder.
        // This happens before the assert statement
        // to ensure things are cleaned up
        // even on (most) failing tests.
        fs::remove_dir_all(test_folder_path).unwrap();
        assert_eq!(result.unwrap(), expected.to_string());
        
    }

    #[test]
    fn copies_profile_pictures() {
        let test_folder_path = Path::new("pictures_test_folder");
        let result_folder_path = Path::new("result_folder");
        let example_member = r#"name = "example"
bio = "example stuff"
sponsor = "sponsor_link.example"
github = "ExampleOne"
"#;
        let test_pfp = "This is not an actual image";

        fs::create_dir_all(test_folder_path).unwrap();
        fs::write(test_folder_path.join("test.toml"), example_member).unwrap();
        fs::write(test_folder_path.join("fake.png"), test_pfp).unwrap();

        copy_profile_pictures(test_folder_path, result_folder_path).unwrap();

        let result_one = fs::read_to_string(result_folder_path.join("fake.png")).unwrap();
        let result_two = fs::read_to_string(result_folder_path.join("test.toml"));

        assert!(result_two.is_err());
        assert_eq!(result_one, test_pfp);
        
    }
}
