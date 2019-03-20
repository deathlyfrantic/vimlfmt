/// A command that modifies another command (e.g. `silent`). The following commands can be
/// considered modifiers:
///
/// - `aboveleft` (and `leftabove)`
/// - `belowright` (and `rightbelow)`
/// - `browse`
/// - `botright`
/// - `confirm`
/// - `keepmarks`
/// - `keepalt`
/// - `keepjumps`
/// - `keeppatterns`
/// - `hide`
/// - `lockmarks`
/// - `noautocmd`
/// - `noswapfile`
/// - `sandbox`
/// - `silent`
/// - `tab`
/// - `topleft`
/// - `unsilent`
/// - `vertical`
/// - `verbose`
///
/// Note that some of these commands can be invoked by themselves, and therefore are not _always_
/// modifiers.
#[derive(Debug, PartialEq, Clone)]
pub struct Modifier {
    /// The name of the modifier, e.g. `aboveleft` or `noswapfile`.
    pub name: String,
    /// Whether this modifier was invoked with a bang. This can only be true for `silent` - it will
    /// be false in all other cases.
    pub bang: bool,
    /// The count argument to this modifier. Defaults to zero. Only `tab` and `silent` can have a
    /// non-zero value.
    pub count: usize,
}

impl Modifier {
    pub(crate) fn new(name: &str) -> Modifier {
        Modifier {
            name: name.to_string(),
            bang: false,
            count: 0,
        }
    }
}
