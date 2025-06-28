use std::collections::BTreeMap;
use std::fmt;

use crate::node::Node;

pub struct Trie {
    // The word or partial word associated with this node.
    fragment: String,
    // The number of times exactly this word has occurred.
    count: usize,
    // The number of times this node or any descendent of this node has occurred.
    descendents_count: usize,
    // The nodes of other words which begin with this node's fragment.
    children: BTreeMap<char, Trie>,
    // The sort option defines the order in which child nodes will be returned by [Self::children].
    sort_option: SortOption,
    // The display data defines what additional data is displayed by the [Self::value] method.
    display_data: DisplayData,
}

#[derive(Debug, Clone, Copy)]
pub enum SortOption {
    /// Sort children by their value.
    Value,
    /// Sort children by their value, reversed.
    ValueReversed,
    /// Sort children by the number of times they directly occur, from least to greatest.
    DirectCountAscending,
    /// Sort children by the number of times they directly occur, from greatest to least.
    DirectCountDescending,
    /// Sort children by the number of times they or any descendent of theirs occurs, from least to
    /// greatest.
    TotalCountAscending,
    /// Sort children by the number of times they or any descendent of theirs occurs, from greatest
    /// to least.
    TotalCountDescending,
}

#[derive(Debug, Clone, Copy)]
pub enum DisplayData {
    /// Display no additional data along with the word fragment.
    None,
    /// Display the number of times exactly this word has occurred.
    DirectCount,
    /// Display the number of times this word fragment or any descendent of it has occurred.
    TotalCount,
}

impl Trie {
    /// Returns an empty [Trie] which will sort entries by value and display the number of times
    /// the node has occurred as part of the value.
    pub fn new() -> Self {
        Self::new_with_fragment(String::new(), SortOption::Value, DisplayData::DirectCount)
    }

    /// Returns an empty [Trie] which will sort entries by the given [SortOption] and display
    /// values according to the given [DisplayData].
    pub fn with_sort_and_display(sort_option: SortOption, display_data: DisplayData) -> Self {
        Self::new_with_fragment(String::new(), sort_option, display_data)
    }

    /// Returns a new [Trie] node with the given word fragment and a count of `0`, and with the
    /// given [SortOption] and [DisplayData].
    fn new_with_fragment(
        fragment: String,
        sort_option: SortOption,
        display_data: DisplayData,
    ) -> Self {
        Trie {
            fragment,
            count: 0,
            descendents_count: 0,
            children: BTreeMap::new(),
            sort_option,
            display_data,
        }
    }

    /// Returns a new [Trie] populated with the given words, with children sorted lexigraphically
    /// and displaying word counts in the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::Trie;
    /// use simple_tree::Node;
    ///
    /// let trie = Trie::from(vec!["foo", "bar", "baz", "foo", "baz", "foo", "b"]);
    ///
    /// assert_eq!(format!("{}", trie), "
    /// ├── b\t1
    /// │   └── ba\t0
    /// │       ├── bar\t1
    /// │       └── baz\t2
    /// └── f\t0
    ///     └── fo\t0
    ///         └── foo\t3");
    /// ```
    pub fn from<'a>(words: impl IntoIterator<Item = &'a str>) -> Self {
        Self::new().populate(words)
    }

    /// Returns a new [Trie] populated with the given words, with children according to the given
    /// [SortOption] and supplementary data displayed according to the given [DisplayData].
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::trie::{DisplayData, SortOption, Trie};
    /// use simple_tree::Node;
    ///
    /// let trie = Trie::from_with_sort_and_display(vec!["foo", "bar", "baz", "foo", "baz", "foo", "b"], SortOption::TotalCountDescending, DisplayData::TotalCount);
    ///
    /// assert_eq!(format!("{}", trie), "
    /// ├── b\t4
    /// │   └── ba\t3
    /// │       ├── baz\t2
    /// │       └── bar\t1
    /// └── f\t3
    ///     └── fo\t3
    ///         └── foo\t3");
    /// ```
    pub fn from_with_sort_and_display<'a>(
        words: impl IntoIterator<Item = &'a str>,
        sort_option: SortOption,
        display_data: DisplayData,
    ) -> Self {
        Self::with_sort_and_display(sort_option, display_data).populate(words)
    }

    /// Populates the given [Trie] with the given words, then returns it.
    fn populate<'a>(mut self, words: impl IntoIterator<Item = &'a str>) -> Self {
        words.into_iter().for_each(|w| {
            self.add(w.to_string()).unwrap();
        });
        self
    }

    /// Set the sort option for this [Trie] node and all of its descendents to the given
    /// [SortOption].
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::trie::{DisplayData, SortOption, Trie};
    /// use simple_tree::Node;
    ///
    /// let mut trie = Trie::from_with_sort_and_display(
    ///     vec!["hey", "hello", "hey", "hi", "ha", "ha"],
    ///     SortOption::Value,
    ///     DisplayData::DirectCount,
    /// );
    /// assert_eq!(format!("{}", trie), "
    /// └── h\t0
    ///     ├── ha\t2
    ///     ├── he\t0
    ///     │   ├── hel\t0
    ///     │   │   └── hell\t0
    ///     │   │       └── hello\t1
    ///     │   └── hey\t2
    ///     └── hi\t1");
    ///
    /// trie.set_sort_option(SortOption::ValueReversed);
    /// assert_eq!(format!("{}", trie), "
    /// └── h\t0
    ///     ├── hi\t1
    ///     ├── he\t0
    ///     │   ├── hey\t2
    ///     │   └── hel\t0
    ///     │       └── hell\t0
    ///     │           └── hello\t1
    ///     └── ha\t2");
    ///
    /// trie.set_sort_option(SortOption::DirectCountAscending);
    /// assert_eq!(format!("{}", trie), "
    /// └── h\t0
    ///     ├── he\t0
    ///     │   ├── hel\t0
    ///     │   │   └── hell\t0
    ///     │   │       └── hello\t1
    ///     │   └── hey\t2
    ///     ├── hi\t1
    ///     └── ha\t2");
    ///
    /// trie.set_sort_option(SortOption::DirectCountDescending);
    /// assert_eq!(format!("{}", trie), "
    /// └── h\t0
    ///     ├── ha\t2
    ///     ├── hi\t1
    ///     └── he\t0
    ///         ├── hey\t2
    ///         └── hel\t0
    ///             └── hell\t0
    ///                 └── hello\t1");
    ///
    /// trie.set_sort_option(SortOption::TotalCountAscending);
    /// assert_eq!(format!("{}", trie), "
    /// └── h\t0
    ///     ├── hi\t1
    ///     ├── ha\t2
    ///     └── he\t0
    ///         ├── hel\t0
    ///         │   └── hell\t0
    ///         │       └── hello\t1
    ///         └── hey\t2");
    ///
    /// trie.set_sort_option(SortOption::TotalCountDescending);
    /// assert_eq!(format!("{}", trie), "
    /// └── h\t0
    ///     ├── he\t0
    ///     │   ├── hey\t2
    ///     │   └── hel\t0
    ///     │       └── hell\t0
    ///     │           └── hello\t1
    ///     ├── ha\t2
    ///     └── hi\t1");
    /// ```
    pub fn set_sort_option(&mut self, sort_option: SortOption) {
        self.sort_option = sort_option;
        self.children
            .values_mut()
            .for_each(|c| c.set_sort_option(sort_option))
    }

    /// Set the display data for this [Trie] node and all of its descendents to the given
    /// [DisplayData].
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::trie::{DisplayData, SortOption, Trie};
    /// use simple_tree::Node;
    ///
    /// let mut trie = Trie::from_with_sort_and_display(
    ///     vec!["hey", "hello", "hey", "hi", "ha", "ha"],
    ///     SortOption::Value,
    ///     DisplayData::None,
    /// );
    /// assert_eq!(format!("{}", trie), "
    /// └── h
    ///     ├── ha
    ///     ├── he
    ///     │   ├── hel
    ///     │   │   └── hell
    ///     │   │       └── hello
    ///     │   └── hey
    ///     └── hi");
    ///
    /// trie.set_display_data(DisplayData::DirectCount);
    /// assert_eq!(format!("{}", trie), "
    /// └── h\t0
    ///     ├── ha\t2
    ///     ├── he\t0
    ///     │   ├── hel\t0
    ///     │   │   └── hell\t0
    ///     │   │       └── hello\t1
    ///     │   └── hey\t2
    ///     └── hi\t1");
    ///
    /// trie.set_display_data(DisplayData::TotalCount);
    /// assert_eq!(format!("{}", trie), "
    /// └── h\t6
    ///     ├── ha\t2
    ///     ├── he\t3
    ///     │   ├── hel\t1
    ///     │   │   └── hell\t1
    ///     │   │       └── hello\t1
    ///     │   └── hey\t2
    ///     └── hi\t1");
    /// ```
    pub fn set_display_data(&mut self, display_data: DisplayData) {
        self.display_data = display_data;
        self.children
            .values_mut()
            .for_each(|c| c.set_display_data(display_data))
    }

    /// Returns the next `char` in the given word following `self.fragment`, if it exists.
    fn next_char(&self, word: &str) -> Option<char> {
        word.chars().nth(self.fragment.len())
    }

    /// Returns an iterator over the potential next characters mapping to children of this [Trie]
    /// node.
    fn options(&self) -> impl Iterator<Item = &char> {
        self.children.keys()
    }

    /// Returns a reference to the child [Trie] node corresponding to the given `char`, if it
    /// exists.
    fn get(&self, ch: char) -> Option<&Trie> {
        self.children.get(&ch)
    }

    /// Returns a mutable reference to the child [Trie] node corresponding to the given `char`, if
    /// it exists.
    fn get_mut(&mut self, ch: char) -> Option<&mut Trie> {
        self.children.get_mut(&ch)
    }

    /// Returns a reference to the [Trie] node corresponding to the given word fragment, if it
    /// exists in the (sub)trie rooted at `self`.
    fn find(&self, remaining: &str) -> Option<&Trie> {
        if remaining.is_empty() {
            return Some(self);
        }
        let (first, rest) = first_rest(remaining)?;
        self.get(first)?.find(rest)
    }

    /// Returns a mutable reference to the [Trie] node corresponding to the given word fragment, if
    /// it exists in the (sub)trie rooted at `self`.
    fn find_mut(&mut self, remaining: &str) -> Option<&mut Trie> {
        if remaining.is_empty() {
            return Some(self);
        }
        let (first, rest) = first_rest(remaining)?;
        self.get_mut(first)?.find_mut(rest)
    }

    /// Returns the number of times the given word has been added to the [Trie].
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::Trie;
    /// use simple_tree::Node;
    ///
    /// let trie = Trie::from(vec!["foo", "bar", "baz", "foo", "baz", "foo", "b"]);
    /// assert_eq!(trie.occurrences("foo"), 3);
    /// assert_eq!(trie.occurrences("bar"), 1);
    /// assert_eq!(trie.occurrences("baz"), 2);
    /// assert_eq!(trie.occurrences("b"), 1);
    ///
    /// assert_eq!(trie.occurrences("f"), 0);
    /// assert_eq!(trie.occurrences("fo"), 0);
    /// assert_eq!(trie.occurrences("ba"), 0);
    /// ```
    pub fn occurrences(&self, word: &str) -> usize {
        self.find(word).map_or(0, |n| n.count)
    }

    /// Returns the total number of times this node or any descendent of it has been added to the
    /// [Trie].
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::Trie;
    /// use simple_tree::Node;
    ///
    /// let mut trie = Trie::new();
    ///
    /// assert_eq!(trie.add(String::from("hey")), Ok(1));
    /// assert_eq!(trie.add(String::from("hello")), Ok(1));
    /// assert_eq!(trie.add(String::from("he")), Ok(1));
    /// assert_eq!(trie.add(String::from("hi")), Ok(1));
    /// assert_eq!(trie.add(String::from("hello")), Ok(2));
    ///
    /// assert_eq!(trie.total_occurrences(), 5);
    ///
    /// let h_node = trie.children().next().unwrap();
    /// assert_eq!(h_node.total_occurrences(), 5);
    ///
    /// let he_node = h_node.children().next().unwrap();
    /// assert_eq!(he_node.total_occurrences(), 4);
    ///
    /// let hel_node = he_node.children().next().unwrap();
    /// assert_eq!(hel_node.total_occurrences(), 2);
    /// ```
    pub fn total_occurrences(&self) -> usize {
        self.descendents_count
    }

    /// Adds the given word to the [Trie] and returns the total number of times it now occurs.
    ///
    /// If the given word is already in the trie, then increment its occurrences count by 1.
    /// Otherwise, add nodes to the trie for the word and any prefixes which are not yet present in
    /// the trie, and set the occurrence count of the new node for the word itself to be 1.
    ///
    /// Examples
    ///
    /// ```
    /// use simple_tree::implementations::Trie;
    /// use simple_tree::Node;
    ///
    /// let mut trie = Trie::new();
    ///
    /// assert_eq!(trie.add(String::from("hello")), Ok(1));
    /// assert_eq!(trie.add(String::from("hi")), Ok(1));
    /// assert_eq!(trie.add(String::from("hey")), Ok(1));
    /// assert_eq!(trie.add(String::from("hi")), Ok(2));
    ///
    /// assert_eq!(format!("{}", trie), "
    /// └── h\t0
    ///     ├── he\t0
    ///     │   ├── hel\t0
    ///     │   │   └── hell\t0
    ///     │   │       └── hello\t1
    ///     │   └── hey\t1
    ///     └── hi\t2")
    /// ```
    pub fn add(&mut self, word: String) -> Result<usize, &'static str> {
        if !word.starts_with(&self.fragment) {
            return Err("cannot add word to node with incompatible prefix");
        }
        self.descendents_count += 1;
        let Some(next_char) = self.next_char(&word) else {
            self.count += 1;
            return Ok(self.count);
        };
        Ok(self
            .children
            .entry(next_char)
            .or_insert_with(|| {
                Self::new_with_fragment(
                    word[..self.fragment.len() + 1].to_string(),
                    self.sort_option,
                    self.display_data,
                )
            })
            .add(word)
            .unwrap())
    }
}

impl Node for Trie {
    /// Returns a displayable representation of this trie node, which includes the word or fragment
    /// associated with the node along with the count of the number of times that exact word has
    /// occurred (which may be 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::Trie;
    /// use simple_tree::Node;
    ///
    /// let root = Trie::from(vec!["foo", "bar", "baz"]);
    /// assert_eq!(format!("{}", root.value()), "");
    ///
    /// let b_node = root.children().next().unwrap();
    /// assert_eq!(format!("{}", b_node.value()), "b\t0");
    ///
    /// let ba_node = b_node.children().next().unwrap();
    /// assert_eq!(format!("{}", ba_node.value()), "ba\t0");
    ///
    /// let bar_node = ba_node.children().next().unwrap();
    /// assert_eq!(format!("{}", bar_node.value()), "bar\t1");
    /// ```
    fn value(&self) -> impl fmt::Display {
        if self.fragment.is_empty() {
            return String::new();
        }
        match self.display_data {
            DisplayData::None => self.fragment.to_string(),
            DisplayData::DirectCount => format!("{}\t{}", &self.fragment, self.count),
            DisplayData::TotalCount => format!("{}\t{}", &self.fragment, self.descendents_count),
        }
    }

    /// Returns an iterator over the [Trie] nodes whose
    /// associated with the node along with the count of the number of times that exact word has
    /// occurred (which may be 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::Trie;
    /// use simple_tree::Node;
    ///
    /// let root = Trie::from(vec!["foo", "bar", "baz"]);
    /// assert_eq!(format!("{}", root.value()), "");
    ///
    /// let b_node = root.children().next().unwrap();
    /// assert_eq!(format!("{}", b_node.value()), "b\t0");
    ///
    /// let ba_node = b_node.children().next().unwrap();
    /// assert_eq!(format!("{}", ba_node.value()), "ba\t0");
    ///
    /// let bar_node = ba_node.children().next().unwrap();
    /// assert_eq!(format!("{}", bar_node.value()), "bar\t1");
    /// ```
    fn children(&self) -> impl Iterator<Item = &Self> {
        // To make all return types the same, and allow the trait to keep using an impl instead of
        // Box<dyn ...>, collect into a vec and then return it as an iterator.
        match self.sort_option {
            SortOption::Value => self.children.values().collect::<Vec<_>>().into_iter(),
            SortOption::ValueReversed => {
                self.children.values().rev().collect::<Vec<_>>().into_iter()
            }
            SortOption::DirectCountAscending => {
                let mut result = self.children.values().collect::<Vec<_>>();
                result.sort_by(|&a, &b| a.count.cmp(&b.count));
                result.into_iter()
            }
            SortOption::DirectCountDescending => {
                let mut result = self.children.values().collect::<Vec<_>>();
                result.sort_by(|&a, &b| b.count.cmp(&a.count));
                result.into_iter()
            }
            SortOption::TotalCountAscending => {
                let mut result = self.children.values().collect::<Vec<_>>();
                result.sort_by(|&a, &b| a.descendents_count.cmp(&b.descendents_count));
                result.into_iter()
            }
            SortOption::TotalCountDescending => {
                let mut result = self.children.values().collect::<Vec<_>>();
                result.sort_by(|&a, &b| b.descendents_count.cmp(&a.descendents_count));
                result.into_iter()
            }
        }
    }
}

impl fmt::Display for Trie {
    /// Format using the default [Node::fmt] implementation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Node::fmt(self, f)
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

/// Splits the given word fragment into the first `char` and the remaining string.
///
/// Returns `None` if the given string is empty.
fn first_rest(fragment: &str) -> Option<(char, &str)> {
    let mut ch_ind_iter = fragment.char_indices();
    let (_, first) = ch_ind_iter.next()?;
    let rest = ch_ind_iter.as_str();
    Some((first, rest))
}
