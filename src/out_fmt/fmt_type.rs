use pareg::FromArg;

#[derive(Debug, Clone, Copy, Default, FromArg)]
pub enum FmtType {
    #[default]
    Text,
    Latex,
}
