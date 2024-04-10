//! Incomplete. Needs revision

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: &mut [i32]) -> Self {
        let mut new_vals = Vec::new();

        for (index, value) in val.iter_mut().enumerate() {
            let mut temp_val = *value;

            while temp_val > 10 {
                if temp_val == 10 {
                    new_vals.extend(vec![0, 1]);
                    temp_val -= 10;
                } else {
                    temp_val -= 10;
                }

                //     if temp_val == 10 {
                //         // new_vals.push(0);
                //         if let Some(value) = new_vals.get_mut(index + 1) {
                //             // new_vals.push(1);
                //             *value += 1
                //         } else {
                //             new_vals.push(1);
                //         }
                //         temp_val -= 10;
                //     } else {
                //         new_vals.push(9);
                //         temp_val -= 9;
                //     }
            }
            // new_vals.push(temp_val);
        }

        let mut new_list_node = ListNode {
            next: None,
            val: new_vals[0],
        };

        if new_vals.len().gt(&1) {
            new_list_node.next = Some(Box::new(ListNode::new(&mut new_vals[1..])));
        }

        new_list_node
    }
}

struct AddTwoNumbers {
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    lfinal: Option<Box<ListNode>>,
}

impl AddTwoNumbers {
    fn new(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Self {
        Self {
            l1,
            l2,
            lfinal: Default::default(),
        }
    }

    fn append_lfinal(lfinal: &mut Option<Box<ListNode>>, l1: Option<Box<ListNode>>) {
        if let Some(lfinal) = lfinal.as_mut() {
            if lfinal.next.is_some() {
                Self::append_lfinal(&mut lfinal.next, l1);
            } else {
                lfinal.next = l1
            }
        } else {
            *lfinal = l1;
        }
    }

    fn solution(&mut self) -> Option<Box<ListNode>> {
        let lfinal = self.lfinal.clone();

        if let Some(node) = self.l1.as_mut() {
            if let Some(node2) = self.l2.as_mut() {
                Self::append_lfinal(
                    &mut self.lfinal,
                    Some(Box::new(ListNode::new(&mut [node.val + node2.val]))),
                );

                self.l2 = self.l2.clone().unwrap().next;
            } else {
                Self::append_lfinal(
                    &mut self.lfinal,
                    Some(Box::new(ListNode::new(&mut [node.val]))),
                );
            }

            self.l1 = self.l1.clone().unwrap().next;
            self.solution();
        } else if let Some(node2) = &self.l2 {
            Self::append_lfinal(
                &mut self.lfinal,
                Some(Box::new(ListNode::new(&mut [node2.val]))),
            );
            self.l2 = self.l2.clone().unwrap().next;
            self.solution();
        }

        self.lfinal.clone()

        // if let Some(new_node) = new_node.clone() {
        //     return AddTwoNumbers::new(new_node.next, None).solution();
        // }

        // new_node
    }
}

#[cfg(test)]
mod tests {
    use super::{AddTwoNumbers, ListNode};

    #[test]
    fn add_two_numbers() {
        let l1 = ListNode::new(&mut [2, 4, 3]);
        let l2 = ListNode::new(&mut [5, 6, 4]);
        let mut add_two = AddTwoNumbers::new(Some(Box::new(l1)), Some(Box::new(l2)));
        let two = add_two.l1.clone().unwrap().val;
        add_two.solution();
        assert_eq!(
            add_two.lfinal,
            Some(Box::new(ListNode::new(&mut [2, 4, 3])))
        );

        // let mut l2 = ListNode::new(&[2]);
        // let mut l3 = ListNode::new(&[4]);
        // let l4 = ListNode::new(&[3]);
        // l3.next = Some(Box::new(l4));
        // l2.next = Some(Box::new(l3));
        // assert_eq!(l1, l2);
        // println!("{l1:?}");
    }
}
