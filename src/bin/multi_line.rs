use indoc::indoc;
fn main() {
    let s = indoc!{r#"
        {
            "asdf": "asrt", 
            "sdfgdg": {
                "asd": "qwe",
                "aa": [
                    1, 2, 3 
                ]
            }
        }
    "#};
    println!("{}", s);
}