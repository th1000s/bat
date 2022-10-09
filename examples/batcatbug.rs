fn main() {
    // syntect with features "default-themes" and "default-syntaxes"
    use syntect::easy::HighlightLines;
    use syntect::highlighting::ThemeSet;
    use syntect::parsing::SyntaxSet;

    let cat = "`😸️";

    let ext = "js"; // not with rs nor py nor cpp nor sh nor rb

    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    {
        let synset = SyntaxSet::load_defaults_newlines();
        let synref = synset.find_syntax_by_extension(ext).unwrap();

        let mut h = HighlightLines::new(synref, theme);

        let l = h.highlight_line(cat, &synset).unwrap();
        println!("Ok (pure syntect): {:#?}", l);
    }

    print!("\n\n    #####################\n\n");

    {
        let assets = bat::assets::HighlightingAssets::from_binary();

        let synset = assets.get_syntax_set().unwrap();
        let synref = synset.find_syntax_by_extension(ext).unwrap();

        let mut h = HighlightLines::new(synref, theme);

        let l = h.highlight_line(cat, &synset).unwrap();
        println!("Bad (using bat assets): {:#?}", l);
    }
}
