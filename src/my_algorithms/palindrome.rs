use regex::Regex;
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn is_palindrome(sample: &str) -> bool {

    // make all characters lower case
    let txt = sample.to_string().to_lowercase();

    // regex for removing non-word characters
    let re = Regex::new(r"[\W]").unwrap();

    // removing non-word characters
    let clean_txt = re.replace_all(&txt, "").to_string();
    println!("clean text: {}", clean_txt);

    // reverse the text
    let reverse_txt: String = clean_txt.graphemes(true).rev().collect();

    // compare directions
    clean_txt == reverse_txt
}

#[cfg(test)]
mod tests {
    #[test]
    fn palindrome_test() {

        // palindome samples
        assert!(super::is_palindrome("A man, a plan, a canal: Panama!"));
        assert!(super::is_palindrome("Dammit I’m mad."));
        assert!(super::is_palindrome("Engage le jeu que je le gagne."));
        assert!(super::is_palindrome("I topi non avevano nipoti."));
        assert!(super::is_palindrome("Autore, ero tua."));
        assert!(super::is_palindrome("Socorram-me subi no onibus em Marrocos."));
        assert!(super::is_palindrome("A mala nada na lama."));
        assert!(super::is_palindrome("А тот суп – пустота."));
        assert!(super::is_palindrome("Нажал кабан на баклажан."));

        // non-palindome samples
        assert!(!super::is_palindrome("Lorem ipsum dolor sit amet,"));
        assert!(!super::is_palindrome("consectetur adipiscing elit,"));
        assert!(!super::is_palindrome("sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."));
        assert!(!super::is_palindrome("A plan, a man, a canal: Panama!"));
        assert!(!super::is_palindrome("Нажал баклажан на кабан."));
    }
}
