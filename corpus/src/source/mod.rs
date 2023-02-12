pub mod gutenberg;

/// Implements a common interface for sources
pub trait Source {
    /// Returns a random block of text from the source. This should be used as the main
    /// way to get prompts that users type out.
    fn get_random_prompt(&self) -> String;
}
