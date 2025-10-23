use pareg::FromArg;

#[derive(Debug, Clone, Copy, Default, FromArg)]
pub enum OutFmt {
    #[default]
    Text,
}
