mod execution_error;
mod node;

pub use self::execution_error::ExecutionError;
pub use self::node::Node;

pub struct FuzzyCmd {
    fuzzy: bool,
    seperator: String,
    tree: Node,
}

impl FuzzyCmd {
    /// Create a new empty FuzzyCmd.
    pub fn new() -> Self {
        Default::default()
    }
    /// Enable fuzziness for matching commands. Default.
    pub fn enable_fuzzy(mut self) -> Self {
        self.fuzzy = true;
        self
    }
    /// Disable fuzziness for matching commands.
    pub fn disable_fuzzy(mut self) -> Self {
        self.fuzzy = false;
        self
    }
    /// Set the seperator for a chain of commands.
    /// The default seperator is `' '` (a space).
    /// But you may, for no apperant reason do something like this:
    /// ```
    /// use fuzzy_cmd::FuzzyCmd;
    /// use std::sync::mpsc::channel;
    ///
    /// let (send, recv) = channel::<&str>();
    /// let mut fuzz = FuzzyCmd::new()
    /// //  ...
    ///     .seperator("THIS IS AWESOME!");
    /// // Add some functionionality
    /// fuzz.add("help")
    ///     .add("me")
    ///     .call(move || {
    ///         send.send("chopchopchop (helicopter sound)").unwrap();
    ///     });
    /// // Now usage is easy..
    /// // To get some help for you, just type
    /// fuzz.exec("helpTHIS IS AWESOME!me");
    /// // And help is on the way!
    /// assert_eq!(recv.recv(), Ok("chopchopchop (helicopter sound)"));
    /// ```
    pub fn seperator(mut self, sep: &str) -> Self {
        self.seperator = sep.to_string();
        self
    }
    /// Execute a command.
    pub fn exec(&mut self, cmd: &str) -> Result<(), ExecutionError> {
        let cmds: Vec<String> = cmd.split(&self.seperator).map(|s| s.to_string()).collect();
        self.tree.exec(&cmds, self.fuzzy)
    }
    /// Add a command to the root.
    /// This adds a top-level command.
    /// # Usage
    /// ```
    /// use fuzzy_cmd::FuzzyCmd;
    ///
    /// let mut fuzz = FuzzyCmd::new();
    /// fuzz.add("some_command");
    /// fuzz.add("another_command");
    /// fuzz.add("yac");
    /// fuzz.add("Done_here!");
    /// ```
    pub fn add(&mut self, cmd: &str) -> &mut Node {
        self.tree.add(cmd)
    }
}

impl Default for FuzzyCmd {
    fn default() -> Self {
        FuzzyCmd {
            fuzzy: true,
            seperator: String::from(" "),
            tree: Node::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        let _ = FuzzyCmd::new();
        let _ = FuzzyCmd::new()
            .enable_fuzzy()
            .seperator("Just some testing");
    }
}
