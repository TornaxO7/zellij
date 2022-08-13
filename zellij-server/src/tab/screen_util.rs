use std::collections::BTreeMap;

use super::Tab;

pub type TabID = usize;

pub struct Tabs<T> {
    /// Here are all tabs stored
    tabs: BTreeMap<TabID, Tab>,

    /// The values are the order of the interaction with the tabs
    history: BTreeMap<T, Vec<TabID>>,

    /// The values are the tab-indexes which are focused
    focused: BTreeMap<T, TabID>,
}

impl<T: Ord> Tabs<T> {
    pub fn new() -> Self {
        Self {
            tabs: BTreeMap::new(),
            history: BTreeMap::new(),
            focused: BTreeMap::new(),
        }
    }

    /// Adds a new tab and applies automatically its index
    pub fn add_tab(&mut self, tab: Tab) {
        self.tabs.insert(tab.uid, tab);
    }

    /// Checks if there're any tabs
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    /// Returns a mutable reference to the tab with the given index
    pub fn get_tab(&mut self, tab_index: TabID) -> Option<&mut Tab> {
        self.tabs.get_mut(&tab_index)
    }

    /// Returns the amount of tabs
    pub fn amount_tabs(&self) -> usize {
        self.tabs.len()
    }

    /// Returns the focused tab by the given key
    pub fn get_focused_tab(&self, key: T) -> Option<&Tab> {
        self.focused.get(&key)
            .and_then(|tab_index: &TabID| self.tabs.get(tab_index))
    }

    /// Pops the previous focused (selected by its key) tab and returns it
    pub fn pop_prev_tab(&mut self, key: T) -> Option<&Tab> {
        self.history.get_mut(&key)
            .and_then(|history: &mut Vec<TabID>| history.pop())
            .and_then(|tab_index: TabID| self.tabs.get(&tab_index))
    }

    /// Returns the next free index, where a neew [`Tab`] can be added.
    /// Currently, this is right after the last currently existing tab, or `0` if
    /// no tabs exist in this screen yet.
    pub fn get_free_tab_index(&self) -> usize {
        if let Some(index) = self.tabs.keys().last() {
            *index + 1
        } else {
            0
        }
    }

    pub fn add_key(&mut self, key: T) {
        self.history.insert(key, Vec::new());
        self.focused.insert(key, self.get_free_tab_index());
    }

    pub fn remove_key(&mut self, key: &T) {
        self.history.remove(key);
        self.focused.remove(key);

        self.tabs.iter_mut().for_each(|(_, tab)| {
        });
    }

    pub fn get_mut_tabs(&self) -> &mut BTreeMap<TabID, Tab> {
        &mut self.tabs
    }
}
