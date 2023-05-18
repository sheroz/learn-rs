// useful resources:
// http://zsiciarz.github.io/24daysofrust/book/vol1/day21.html

use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub fn sha256_sample() {
    let txt = "Hello world!";
    let mut sha = Sha256::new();
    sha.input_str(txt);
    println!("\nSHA256 hash for {}\n{}\n", txt, sha.result_str());
}
