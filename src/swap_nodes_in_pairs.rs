// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl ListNode {
    // This function has a time complexity of O(n/2) ~ O(n)
    #[allow(dead_code)]
    fn swap_pairs(&mut self) -> Option<Box<Self>> {
        // >>>>>>>>>>>> swapping logic start
        let mut current_node = self.clone();
        let mut next_node: Option<Box<ListNode>> = None;

        if let Some(self_next_node) = self.next.as_mut() {
            current_node.next = self_next_node.next.clone();
            next_node = Some(self_next_node.clone());
        }

        if let Some(mut next_node) = next_node {
            next_node.next = Some(Box::new(current_node));
            *self = *next_node.clone();
        }
        // swapping logic end <<<<<<<<<<<<<<<

        // This section ensures that swapping continues down the list, but that swapping is only
        // enabled for pairs, i.e, only every pair is flipped and all odd nodes are skipped.
        // Note that the next value is skipped and a recursive call is to be made to node.swap();
        if let Some(node) = self.next.as_mut() {
            if let Some(node) = node.next.as_mut() {
                node.swap_pairs();
            }
        }
        None
    }
}
#[derive(Debug)]
struct SwapNodesInPairs {
    #[allow(dead_code)]
    starter_node: Box<ListNode>,
}

impl SwapNodesInPairs {
    #[allow(dead_code)]
    fn new(starter_node: Box<ListNode>) -> Self {
        Self { starter_node }
    }

    #[allow(dead_code)]
    fn solve(&mut self) {
        self.starter_node.swap_pairs();
    }
}

#[cfg(test)]
mod tests {
    use crate::swap_nodes_in_pairs::SwapNodesInPairs;

    use super::ListNode;

    #[test]
    fn test_node_swap() {
        let mut list_node = ListNode::new(1);
        list_node.swap_pairs();
        assert_eq!(list_node.val, 1);
    }

    #[test]
    fn test_node_swap_2() {
        let mut list_node = Box::new(ListNode::new(1));
        let mut list_node_2 = Box::new(ListNode::new(2));
        let list_node_3 = Box::new(ListNode::new(3));
        list_node_2.next = Some(list_node_3.clone());
        list_node.next = Some(list_node_2.clone());
        list_node.swap_pairs();
        assert_eq!(list_node.val, list_node_2.val);
    }

    #[test]
    fn test_node_swap_6() {
        let mut list_node = Box::new(ListNode::new(1));
        let mut list_node_2 = Box::new(ListNode::new(2));
        let mut list_node_3 = Box::new(ListNode::new(3));
        let mut list_node_4 = Box::new(ListNode::new(4));
        let mut list_node_5 = Box::new(ListNode::new(5));
        let list_node_6 = Box::new(ListNode::new(6));
        list_node_5.next = Some(list_node_6);
        list_node_4.next = Some(list_node_5);
        list_node_3.next = Some(list_node_4.clone());
        list_node_2.next = Some(list_node_3.clone());
        list_node.next = Some(list_node_2.clone());
        let mut swap_nodes = SwapNodesInPairs::new(list_node.clone());
        swap_nodes.solve();
        assert_eq!(swap_nodes.starter_node.val, list_node_2.val);
        assert_eq!(
            swap_nodes.starter_node.next.as_mut().unwrap().val,
            list_node.val
        );
        assert_eq!(
            swap_nodes.starter_node.next.unwrap().next.unwrap().val,
            list_node_4.val
        );
    }
}
