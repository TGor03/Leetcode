// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;

    while l1.is_some() || l2.is_some() || carry > 0 {
        let val1 = l1.take().map(|node| {
            l1 = node.next;
            node.val
        }).unwrap_or(0);

        let val2 = l2.take().map(|node| {
            l2 = node.next;
            node.val
        }).unwrap_or(0);

        let sum = val1 + val2 + carry;
        carry = sum / 10;

        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
    }

    dummy_head.next
}

fn main() {
    // Example usage:
    // Create two linked lists
    let l1 = Some(Box::new(ListNode::new(2)));
    let l2 = Some(Box::new(ListNode::new(5)));

    // Add the two linked lists
    let result = add_two_numbers(l1, l2);

    // Print the result (you can define a helper function to print the linked list)
    print_linked_list(result);
}

fn print_linked_list(head: Option<Box<ListNode>>) {
    let mut current = &head;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = &node.next;
    }
    println!("None");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = &head;
        while let Some(node) = current {
            result.push(node.val);
            current = &node.next;
        }
        result
    }

    #[test]
    fn test_add_two_numbers_case1() {
        let l1 = Some(Box::new(ListNode::new(2)));
        let l2 = Some(Box::new(ListNode::new(5)));
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![7]);
    }

    #[test]
    fn test_add_two_numbers_case2() {
        let l1 = Some(Box::new(ListNode::new(9)));
        let l2 = Some(Box::new(ListNode::new(9)));
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![8, 1]);
    }

    #[test]
    fn test_add_two_numbers_case3() {
        let l1 = Some(Box::new(ListNode::new(1)));
        let l2 = Some(Box::new(ListNode::new(9)));
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![0, 1]);
    }

    // Add more test cases here...

    #[test]
    fn test_add_two_numbers_case18() {
        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(0)));
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![0]);
    }

    #[test]
    fn test_add_two_numbers_case19() {
        let l1 = None;
        let l2 = Some(Box::new(ListNode::new(5)));
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![5]);
    }

    #[test]
    fn test_add_two_numbers_case20() {
        let l1 = Some(Box::new(ListNode::new(9)));
        let l2 = None;
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![9]);
    }
}