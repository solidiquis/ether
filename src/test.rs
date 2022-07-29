#[test]
fn test_raw_arguments() {
    use super::run;

    let good_args = vec![
        "ether".to_string(),
        "enc".to_string(),
        "-i".to_string(),
        "Cthulhu Fhtagn".to_string(),
        "-k".to_string(),
        "R'lyeh".to_string(),
    ];

    assert!(run(good_args).is_ok());

    let mut bad_args = vec![
        "-i".to_string(),
        "Cthulhu Fhtagn".to_string(),
        "-k".to_string(),
        "R'lyeh".to_string(),
    ];

    assert!(run(bad_args).is_err());

    bad_args = vec![
        "enc".to_string(),
        "-i".to_string(),
        "-k".to_string(),
        "R'lyeh".to_string(),
    ];

    assert!(run(bad_args).is_err());
}

#[test]
fn test_path_arguments() {
    use super::run;

    // Encrypt 
    let mut good_args = vec![
        "ether".to_string(),
        "enc".to_string(),
        "-I".to_string(),
        "./assets/test_plain_text.txt".to_string(),
        "-K".to_string(),
        "./assets/test_valid_key.txt".to_string(),
    ];

    assert!(run(good_args).is_ok());

    // Decrypt
    good_args = vec![
        "ether".to_string(),
        "dec".to_string(),
        "-I".to_string(),
        "./assets/test_cipher_text.txt".to_string(),
        "-K".to_string(),
        "./assets/test_valid_key.txt".to_string(),
    ];

    assert!(run(good_args).is_ok());

    let mut bad_args = vec![
        "ether".to_string(),
        "dec".to_string(),
        "-I".to_string(),
        "./assets/not_a_valid_file.txt".to_string(),
        "-K".to_string(),
        "./assets/test_valid_key.txt".to_string(),
    ];

    assert!(run(bad_args).is_err());

    // Encryption with file containing invalid key
    bad_args = vec![
        "ether".to_string(),
        "enc".to_string(),
        "-I".to_string(),
        "./assets/test_plain_text.txt".to_string(),
        "-K".to_string(),
        "./assets/test_invalid_key.txt".to_string(),
    ];

    assert!(run(bad_args).is_err());
}
