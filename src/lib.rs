use std::mem;

/// A cell containing a value ([T]), and the last (previous) value stored in the cell.
///
/// # Examples
///
/// ```
/// use memcell::MemoryCell;
///
/// let mut cell = MemoryCell::new(5_u32);
///
/// let new_value = 10;
/// cell.update(new_value);
///
/// assert_eq!(cell.current(), &10);
/// assert_eq!(cell.last(), &Some(5));
/// ```
#[derive(Debug, Clone)]
pub struct MemoryCell<T> {
    current: T,
    last_val: Option<T>,
}

impl<T> MemoryCell<T> {
    /// Set the current value as the last value, then set the current value to the given argument.
    ///
    /// ```
    /// use memcell::MemoryCell;
    ///
    /// let mut cell = MemoryCell::new(5_u32);
    ///
    /// let new_value = 10;
    /// cell.update(new_value);
    ///
    /// assert_eq!(cell.current(), &10);
    /// assert_eq!(cell.last(), &Some(5));
    /// ```
    pub fn update(&mut self, new: T) {
        self.last_val = Some(mem::replace(&mut self.current, new));
    }

    /// Take the current value contained within this `MemoryCell`.
    ///
    /// ```
    /// use memcell::MemoryCell;
    ///
    /// let cell = MemoryCell::new("Joe");
    /// let data = cell.take_current();
    ///
    /// assert_eq!(data, "Joe");
    /// ```
    pub fn take_current(self) -> T {
        self.current
    }

    /// Take the previous value contained within this `MemoryCell`.
    ///
    /// ```
    /// use memcell::MemoryCell;
    ///
    /// let mut cell = MemoryCell::new(5);
    /// cell.update(10);
    ///
    /// assert_eq!(cell.take_last(), Some(5));
    /// ```
    pub fn take_last(self) -> Option<T> {
        self.last_val
    }

    /// Take both the current and last value of this `MemoryCell` and return them in a tuple.
    ///
    /// ```
    /// use memcell::MemoryCell;
    ///
    /// let mut cell = MemoryCell::new(5);
    ///
    /// cell.update(10);
    ///
    /// let (new, old) = cell.take_both();
    ///
    /// assert_eq!(new, 10);
    /// assert_eq!(old, Some(5));
    /// ```
    pub fn take_both(self) -> (T, Option<T>) {
        (self.current, self.last_val)
    }

    /// Get whether this `MemoryCell` contains a previous value.
    ///
    /// ```
    /// use memcell::MemoryCell;
    ///
    /// let mut cell = MemoryCell::new(5);
    ///
    /// assert!(!cell.has_previous());
    ///
    /// cell.update(10);
    ///
    /// assert!(cell.has_previous());
    /// ```
    pub const fn has_previous(&self) -> bool {
        self.last_val.is_some()
    }

    /// Get the current value contained within this `MemoryCell`.
    ///
    /// ```
    /// use memcell::MemoryCell;
    ///
    /// let cell = MemoryCell::new("Joe");
    /// let data = cell.current();
    ///
    /// assert_eq!(data, &"Joe");
    /// ```
    pub const fn current(&self) -> &T {
        &self.current
    }

    /// Get the last value contained in this `MemoryCell`.
    ///
    /// ```
    /// use memcell::MemoryCell;
    ///
    /// let mut cell = MemoryCell::new(5);
    /// cell.update(10);
    ///
    /// assert_eq!(cell.last(), &Some(5));
    /// ```
    pub const fn last(&self) -> &Option<T> {
        &self.last_val
    }

    /// Create a new `MemoryCell` with the given value.
    pub const fn new(current: T) -> Self {
        Self {
            current,
            last_val: None,
        }
    }

    /// Create a new `MemoryCell` containing the given previous value.
    pub const fn with_last(current: T, last_val: Option<T>) -> Self {
        Self { current, last_val }
    }
}

#[cfg(test)]
mod tests {
    use crate::MemoryCell;

    #[test]
    fn update_cell() {
        let old_value = 5;
        let mut cell = MemoryCell::new(old_value);

        let new_value = 10;
        cell.update(new_value);

        assert_eq!(cell.current, new_value);
        assert_eq!(cell.last_val, Some(old_value));
    }
}
