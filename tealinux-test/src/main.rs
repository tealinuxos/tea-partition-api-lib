mod tests;

fn main() {
    tests::blkid_ret::test_blkid(&"/dev/sda".to_string());
    tests::blkid_ret::test_blkid(&"/dev/fd0".to_string());
}
