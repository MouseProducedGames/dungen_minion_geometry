// External includes.

// Standard includes.

// Internal includes.

/// Defines whether something is included.
#[derive(Copy, Clone, Debug)]
pub enum Inclusion {
    /// Include the item.
    Include,
    /// Do not include the item.
    Exclude,
}
