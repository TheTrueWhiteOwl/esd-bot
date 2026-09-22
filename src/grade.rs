use strum::{Display, EnumIter, EnumString};

#[derive(Display, EnumIter, EnumString)]
pub enum GradeOptions {
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    Alumnus,
}

