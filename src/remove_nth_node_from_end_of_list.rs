// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// Implementation straight forward. LeetCode suggests a single traversal is possible, but this
    /// is not very intuitive. For one, it may be possible to build a vector of pointers as the
    /// list is traversed, and then reverse the vector and return the nth -1 index. However, this
    /// is very impractical in situations where linked lists are large. Consider very large linked
    /// lists used as graph db's or similar.
    ///
    /// * `nth_node`: This is the reverse node index.
    fn remove_nth_node(&mut self, nth_node: usize) -> Option<Box<ListNode>> {
        let count = self.count_nodes();

        let index = count - nth_node as i32;

        if index <= 0 {
            return None;
        }

        if index == 1 {
            return Some(Box::new(self.clone()));
        } else {
            let mut second_last_node = self.select_node((index - 1) as usize);
            let node_to_be_removed = self.select_node(index as usize);
            if let Some(insertion_node) = second_last_node.as_mut() {
                if let Some(next_node) = node_to_be_removed {
                    insertion_node.next = next_node.next.clone();
                    return next_node.next;
                } else {
                    insertion_node.next = None;
                    return None;
                }
            }
        }

        None
    }

    /// Helper function providing the length of linked list from the current node.
    fn count_nodes(&mut self) -> i32 {
        if let Some(node) = self.next.as_mut() {
            1 + node.count_nodes()
        } else {
            1
        }
    }

    /// Function for selecting a node at a specific index.
    ///
    /// * `count`: index relative to current node
    fn select_node(&self, count: usize) -> Option<Box<ListNode>> {
        if count == 0 {
            return None;
        }
        if count == 1 {
            return Some(Box::new(self.clone()));
        } else if let Some(node) = &self.next {
            return node.select_node(count - 1);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;

    #[test]
    fn test_list_node_count() {
        let mut list_node = ListNode::new(1);
        let mut second_list_node = Box::new(ListNode::new(2));
        let mut third_list_node = Box::new(ListNode::new(3));
        let fourth_list_node = Box::new(ListNode::new(4));
        third_list_node.next = Some(fourth_list_node);
        second_list_node.next = Some(third_list_node);
        list_node.next = Some(second_list_node);
        assert_eq!(list_node.count_nodes(), 4);
    }

    #[test]
    fn test_node_legacy() {
        let mut list_node = ListNode::new(1);
        let mut second_list_node = Box::new(ListNode::new(2));
        let mut third_list_node = Box::new(ListNode::new(3));
        let fourth_list_node = Box::new(ListNode::new(4));
        third_list_node.next = Some(fourth_list_node);
        second_list_node.next = Some(third_list_node.clone());
        list_node.next = Some(second_list_node);
        assert_eq!(list_node.remove_nth_node(2), Some(third_list_node.clone()));
    }

    #[test]
    fn test_two_nodes() {
        let mut list_node = ListNode::new(1);
        let second_list_node = Box::new(ListNode::new(2));
        list_node.next = Some(second_list_node);
        assert_eq!(list_node.remove_nth_node(1), Some(Box::new(list_node)));
    }

    #[test]
    fn test_single_node() {
        let mut list_node = ListNode::new(1);
        assert_eq!(list_node.remove_nth_node(1), None);
    }
}
