use {
    crate::col::ALL_COLS,
    termimad::{
        MadSkin,
        minimad::OwningTemplateExpander,
    },
    std::io::{
        self,
        Write,
    },
};

static MD: &str = r#"
The `--cols` launch argument lets you specify the columns of the **dysk** table.

You can give the explicit list of all columns: `dysk -c dev+fs`

You can add columns to the default ones: `dysk -c +dev+size`

Complete syntax at https://dystroy.org/dysk/table

|:-:|:-:|:-:|:-
|column | aliases | default | content
|:-:|:-:|:-:|-
${column
|${name}|${aliases}|${default}|${description}
}
|-
"#;

/// Print an help text describing columns
pub fn write<W: Write>(
    w: &mut W,
    color: bool,
    ascii: bool,
) -> io::Result<()> {
    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");
    for &col in ALL_COLS {
        expander
            .sub("column")
            .set("name", col.name())
            .set("aliases", col.aliases().join(", "))
            .set("default", if col.is_default() { "x" } else { "" })
            .set("description", col.description());
    }
    let mut skin = if color {
        MadSkin::default()
    } else {
        MadSkin::no_style()
    };
    if ascii {
        skin.limit_to_ascii();
    }
    skin.write_owning_expander_md(w, &expander, MD)
}
