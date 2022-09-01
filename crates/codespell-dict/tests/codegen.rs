pub const DICT: &str = include_str!("../assets/dictionary.txt");

#[test]
fn codegen() {
    let mut content = vec![];
    generate(&mut content);

    let content = String::from_utf8(content).unwrap();
    let content = codegenrs::rustfmt(&content, None).unwrap();
    snapbox::assert_eq_path("./src/dict_codegen.rs", &content);
}

fn generate<W: std::io::Write>(file: &mut W) {
    writeln!(
        file,
        "// This file is @generated {}",
        file!().replace('\\', "/")
    )
    .unwrap();
    writeln!(file).unwrap();

    let dict = parse_dict(DICT);

    dictgen::generate_table(
        file,
        "WORD_DICTIONARY",
        "&[&str]",
        dict.map(|kv| (kv.0, format!("&{:?}", kv.1))),
    )
    .unwrap();
}

fn parse_dict(raw: &str) -> impl Iterator<Item = (&str, Vec<&str>)> {
    raw.lines().map(|s| {
        let mut parts = s.splitn(2, "->");
        let typo = parts.next().unwrap().trim();
        let corrections = parts
            .next()
            .unwrap()
            .split(',')
            .filter_map(|c| {
                let c = c.trim();
                if c.is_empty() {
                    None
                } else {
                    Some(c)
                }
            })
            .collect();
        (typo, corrections)
    })
}
