mod methods;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Variant {
    /// The Apollo NCS variant is for backwards compatibility with the Apollo NSC UUID format.
    NSC = 0,

    #[default]
    /// This is the standard, default variant of UUID.
    OSF = 4,

    /// Reserved for Microsoft Corporation backward compatibility.
    DCOM = 6,

    /// Reserved for future use.
    Reserved = 7,
}
