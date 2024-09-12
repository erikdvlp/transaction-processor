use std::error::Error;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn integration_test() -> Result<(), Box<dyn Error>> {
    let mut input_file = NamedTempFile::new()?;
    let contents = "type,client,tx,amount\ndeposit,1,1,1.0\ndeposit,2,2,2.0";
    input_file.write_all(contents.as_bytes())?;
    let input_path = input_file.path().to_str().unwrap();

    let command = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(input_path)
        .output()?;

    assert!(command.status.success());
    let output = String::from_utf8_lossy(&command.stdout);
    let expected = "client,available,held,total,locked\n1,1.0000,0.0000,1.0000,false\n2,2.0000,0.0000,2.0000,false\n\n";
    assert_eq!(output, expected);

    Ok(())
}
