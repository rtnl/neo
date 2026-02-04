use strum::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Operation {
    FileRead,
    FileCopy,
}
