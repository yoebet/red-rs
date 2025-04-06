fn guest_base_forms(word: &str) -> Vec<String> {
    let mut forms: Vec<String> = Vec::new();
    let len = word.len();
    if word.ends_with("s") {
        if word.ends_with("es") {
            if word.ends_with("ies") {
                forms.push(word[..word.len() - 3].to_owned() + "y");
            } else {
                forms.push(word[..len - 2].to_owned());
            }
        }
        if !word.ends_with("ss") {
            forms.push(word[..len - 1].to_owned());
        }
    }
    println!("{:?}", forms);
    forms
}

#[test]
fn test_base_forms() {
    let _ = guest_base_forms("trees");
    let _ = guest_base_forms("cheeses");
    let _ = guest_base_forms("tomatoes");
    let _ = guest_base_forms("ferries");
}
