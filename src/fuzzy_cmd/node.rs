use fuzzy_cmd::ExecutionError;

/// A command node.
/// A node contains either a function __or__ a list of subnodes.
///
/// See [FuzzyCmd](::fuzzy_cmd::FuzzyCmd) for more usage examples!
pub struct Node {
    next: Next,
}

/// The next enum.
/// Either a function to execute or a subnode to dive into.
pub enum Next {
    Fn(Box<FnMut()>),
    Sub(Vec<(String, Box<Node>)>),
}

impl Node {
    /// Create a new Node.
    /// The new node contains nothing but an function to panic when called.
    /// You should probably add more nodes, or add a real function using [call](Node::call).
    pub fn new() -> Self {
        Default::default()
    }
    /// Append a command to this map.
    /// Returns the new Node for easy chaining.
    /// # Example
    /// ```
    /// use fuzzy_cmd::Node;
    ///
    /// let mut n = Node::new();
    /// n.add("test").add("all");
    /// n.add("make").add("some");
    /// // Results in:
    /// // ┌> test -> all -> panic
    /// // n
    /// // └> make -> some -> panic
    /// // See the call function to handle the panic
    /// ```
    pub fn add(&mut self, cmd: &str) -> &mut Node {
        let cmd = cmd.to_string();
        let el = (cmd, Box::new(Node::new()));
        match self.next {
            Next::Fn(_) => {
                self.next = Next::Sub(vec![el]);
            }
            Next::Sub(ref mut v) => {
                v.push(el);
            }
        };
        if let Next::Sub(ref mut v) = self.next {
            let len = v.len();
            &mut v[len - 1].1
        } else {
            panic!("Bug!")
        }
    }
    /// Set this node's function.
    /// # Example
    /// ```
    /// use fuzzy_cmd::Node;
    ///
    /// let mut node = Node::new();
    /// node.call(|| {
    ///    println!("I was called!");
    /// })
    /// ```
    pub fn call<F: FnMut() + 'static>(&mut self, f: F) {
        self.next = Next::Fn(Box::from(f));
    }
    /// Calls this node's function if it has one.
    /// If not it calls at most one subnodes exec method.
    /// Which subnode that is is determined by the first element of `cmd`.
    /// - If `match_fuzzy` is `true`, the next command must be prefix of the subnode's command.
    /// **TODO:** Respect single match / multi match!
    pub(crate) fn exec<'a>(&mut self, cmd: &'a [String], match_fuzzy: bool) -> Result<(), ExecutionError> {
        match self.next {
            Next::Fn(ref mut f) => Ok(f()),
            Next::Sub(ref mut subs) => {
                let mut sub_to_call = None;
                for (s, sub) in subs {
                    if match_fuzzy && s.starts_with(&cmd[0])
                        || !match_fuzzy && s.eq_ignore_ascii_case(&cmd[0])
                    {
                        match sub_to_call {
                            None => sub_to_call = Some(sub),
                            Some(_) => return Err(ExecutionError::new(cmd)),
                        }
                    }
                }
                match sub_to_call {
                    Some(sub) => sub.exec(&cmd[1..], match_fuzzy),
                    None => {
                        return Err(ExecutionError::new(cmd));
                    }
                }
            }
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            next: Next::Fn(Box::new(|| {
                panic!("This node is empty. This is probably not what you want!");
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        let mut n = Node::new();
        n.call(|| {
            println!("Fun!");
        });
        n.add("test").call(|| {
            println!("Dies ist ein Test");
        })
    }
    #[test]
    #[should_panic]
    fn empty() {
        let mut n = Node::new();
        n.exec(&[String::new()], false).unwrap();
    }
    #[test]
    fn exec() {
        use std::sync::mpsc::channel;
        let (send, recv) = channel::<&str>();

        let mut n = Node::new();
        n.add("help").call(move || {
            send.send("Got some help!").unwrap();
        });
        n.exec(&[String::from("help")], false).unwrap();
        assert_eq!(recv.recv(), Ok("Got some help!"));

        let (send, recv) = channel::<&str>();
        let mut n = Node::new();
        {
            let test = n.add("test");
            let s = send.clone();
            test.add("all").call(move || {
                s.send("test all").unwrap();
            });

            let s = send.clone();
            test.add("docs").call(move || {
                s.send("test docs").unwrap();
            });

            let s = send.clone();
            test.add("nothing").call(move || {
                s.send("test nothing").unwrap();
            });
        }
        {
            let help = n.add("help");
            let s = send.clone();
            help.add("all").call(move || {
                s.send("help all").unwrap();
            });
            let s = send.clone();
            help.add("something").call(move || {
                s.send("help something").unwrap();
            });
        }

        n.exec(&[String::from("test"), String::from("all")], false)
            .unwrap();
        n.exec(&[String::from("help"), String::from("all")], false)
            .unwrap();
        n.exec(&[String::from("test"), String::from("nothing")], false)
            .unwrap();

        n.exec(&[String::from("t"), String::from("a")], true)
            .unwrap();
        n.exec(&[String::from("h"), String::from("a")], true)
            .unwrap();
        n.exec(&[String::from("te"), String::from("not")], true)
            .unwrap();

        assert_eq!(recv.recv(), Ok("test all"));
        assert_eq!(recv.recv(), Ok("help all"));
        assert_eq!(recv.recv(), Ok("test nothing"));
        assert_eq!(recv.recv(), Ok("test all"));
        assert_eq!(recv.recv(), Ok("help all"));
        assert_eq!(recv.recv(), Ok("test nothing"));
    }
    #[test]
    #[should_panic]
    fn fuzzy() {
        let mut n = Node::new();
        {
            let test = n.add("test");
            test.add("all").call(move || {});
            test.add("docs").call(move || {});
            test.add("nothing").call(move || {});
        }
        {
            let help = n.add("help");
            help.add("all").call(move || {});
            help.add("something").call(move || {});
        }
        n.exec(&[String::from("te"), String::from("a")], false).unwrap();
        n.exec(&[String::from("help"), String::from("some")], false).unwrap();
    }
}
