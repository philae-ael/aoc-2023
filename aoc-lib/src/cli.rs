use std::fmt::Display;

#[derive(Clone)]
pub struct FileInput {
    pub filename: String,
}

impl From<String> for FileInput {
    fn from(filename: String) -> Self {
        FileInput { filename }
    }
}

impl FileInput {
    pub fn read(&self) -> std::io::Result<String> {
        std::fs::read_to_string(&self.filename)
    }
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum AocPart {
    Part1,
    Part2,
}

impl Display for AocPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AocPart::Part1 => "part 1",
            AocPart::Part2 => "part 2",
        })
    }
}

#[derive(clap::Parser)]
pub struct Args {
    #[arg(value_enum)]
    pub part: AocPart,
    pub input: FileInput,
}
