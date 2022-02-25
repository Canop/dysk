use {
    crate::col::ALL_COLS,
    termimad::{
        minimad::OwningTemplateExpander,
        MadSkin,
    },
};

static MD: &str = r#"
The `--cols` launch argument lets you specify the columns of the **lfs** table.

You can give the explicit list of all columns: `lfs -c dev+fs`

You can add columns to the default ones: `lfs -c +dev+size`

Complete syntax at https://dystroy.org/lfs/table

|:-:|:-:|:-:|:-
|column | aliases | default | content
|:-:|:-:|:-:|-
${column
|${name}|${aliases}|${default}|${description}
}
|-
"#;

/// Print an help text describing columns
pub fn print(color: bool) {
    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");
    for &col in ALL_COLS {
        expander.sub("column")
            .set("name", col.name())
            .set("aliases", col.aliases().join(", "))
            .set("default", if col.is_default() { "x" } else { "" })
            .set("description", col.description());
    }
    let skin = if color {
        MadSkin::default()
    } else {
        MadSkin::no_style()
    };
    skin.print_owning_expander_md(&expander, MD);
}

