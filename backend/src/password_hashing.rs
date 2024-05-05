use sha2::Digest;

// deterministic!
pub(crate) fn hash_password(password: &str) -> String {
    let mut hasher = sha2::Sha512::new();
    hasher.update(password);
    let result = hasher.finalize();
    result
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join("")
}
