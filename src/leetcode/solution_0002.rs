use crate::data::structure::ListNode;
//2.两数相加
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>)
    -> Option<Box<ListNode>> {
    carried(l1, l2, 0)
}

pub fn carried(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: carried(
                l1.and_then(|x| { carry += x.val; x.next }),
                l2.and_then(|x| { carry += x.val; x.next }),
                carry / 10
            ),
            val: carry % 10
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::data::structure::ListNode;
    use crate::leetcode::solution_0002::add_two_numbers;

    #[test]
    fn example01() {
        let (l1, l2) = prepare_data1();

        // println!("{:?}", show_me(l1));
        // println!("{:?}", show_me(l2));
        assert_eq!(show_me(add_two_numbers(l1, l2)), [7, 0, 8]);
        println!("example01: pass");
    }

    #[test]
    fn example02() {
        let (l1, l2) = prepare_data2();

        // println!("{:?}", show_me(l1));
        // println!("{:?}", show_me(l2));
        assert_eq!(show_me(add_two_numbers(l1, l2)), [0]);
        println!("example02: pass");
    }

    #[test]
    fn example03() {
        let (l1, l2) = prepare_data3();

        // println!("{:?}", show_me(l1));
        // println!("{:?}", show_me(l2));
        assert_eq!(show_me(add_two_numbers(l1, l2)), [8,9,9,9,0,0,0,1]);
        println!("example03: pass");
    }



    fn show_me(l: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        let mut flag = l;

        loop {
            match flag {
                Some(a) => {
                    res.push(a.val);
                    flag = a.next;
                },
                None => break,
            }
        }

        res
    }

    fn prepare_data1() -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut node1 = ListNode::new(2);
        let mut node2 = ListNode::new(4);
        let node3 = ListNode::new(3);

        node2.next = Some(Box::new(node3.clone()));
        node1.next = Some(Box::new(node2.clone()));
        let l1 = Some(Box::new(node1.clone()));

        let mut node4 = ListNode::new(5);
        let mut node5 = ListNode::new(6);
        let node6 = ListNode::new(4);

        node5.next = Some(Box::new(node6.clone()));
        node4.next = Some(Box::new(node5));
        let l2 = Some(Box::new(node4.clone()));

        (l1, l2)
    }

    fn prepare_data2() -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let node1 = ListNode::new(0);
        let l1 = Some(Box::new(node1.clone()));
        let l2 = Some(Box::new(node1.clone()));

        (l1, l2)
    }

    fn prepare_data3() -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut node1 = ListNode::new(9);
        let mut node2 = ListNode::new(9);
        let mut node3 = ListNode::new(9);
        let mut node4 = ListNode::new(9);
        let mut node5 = ListNode::new(9);
        let mut node6 = ListNode::new(9);
        let node7 = ListNode::new(9);

        node6.next = Some(Box::new(node7.clone()));
        node5.next = Some(Box::new(node6.clone()));
        node4.next = Some(Box::new(node5.clone()));

        let l2 = Some(Box::new(node4.clone()));

        node3.next = Some(Box::new(node4.clone()));
        node2.next = Some(Box::new(node3.clone()));
        node1.next = Some(Box::new(node2.clone()));

        let l1 = Some(Box::new(node1.clone()));

        (l1, l2)
    }
}