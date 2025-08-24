use {
    crate::args::*,
    clap::CommandFactory,
};

static INTRO_TEMPLATE: &str = "
**dysk** displays filesystem information in a pretty table.
Complete documentation at https://dystroy.org/dysk

";

static EXAMPLES_TEMPLATE: &str = "
**Examples:**

${examples
**${example-number})** ${example-title}: `${example-cmd}`
${example-comments}
}
";

static EXAMPLES: &[Example] = &[
    Example::new("Standard overview of your usual disks", "dysk", ""),
    Example::new("List all filesystems", "dysk -a", ""),
    Example::new("Display inodes information", "dysk -c +inodes", ""),
    Example::new(
        "Add columns of your choice",
        "dysk -c label+dev+",
        "You may add columns before, after, or at any other place. \
        You can change the column order too. \
        See https://dystroy.org/dysk/table#columns\n",
    ),
    Example::new("See the disk of the current directory", "dysk .", ""),
    Example::new(
        "Filter for low space",
        "dysk -f 'use > 65% | free < 50G'",
        "",
    ),
    Example::new("Filter to exclude SSD disks", "dysk -f 'disk <> SSD'", ""),
    Example::new(
        "Complex filter",
        "dysk -f '(type=xfs & remote=no) | size > 5T'",
        "",
    ),
    Example::new("Export as JSON", "dysk -j", ""),
    Example::new(
        "Sort by free size",
        "dysk -s free",
        "Add `-desc` to the column name to sort in reverse.",
    ),
];

pub fn print(ascii: bool) {
    let mut printer = clap_help::Printer::new(Args::command())
        .with("introduction", INTRO_TEMPLATE)
        .without("author");
    printer.template_keys_mut().push("examples");
    printer.set_template("examples", EXAMPLES_TEMPLATE);
    if ascii {
        printer.skin_mut().limit_to_ascii();
    }
    for (i, example) in EXAMPLES.iter().enumerate() {
        printer
            .expander_mut()
            .sub("examples")
            .set("example-number", i + 1)
            .set("example-title", example.title)
            .set("example-cmd", example.cmd)
            .set_md("example-comments", example.comments);
    }
    printer.print_help();
}

struct Example {
    title: &'static str,
    cmd: &'static str,
    comments: &'static str,
}

impl Example {
    pub const fn new(
        title: &'static str,
        cmd: &'static str,
        comments: &'static str,
    ) -> Self {
        Self {
            title,
            cmd,
            comments,
        }
    }
}
