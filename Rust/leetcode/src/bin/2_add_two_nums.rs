// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  add(l1, l2, 0)
}

pub fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
  let mut sum = carry;

  let l1_current = if let Some(node) = l1{
    sum += node.val;
    node.next
  } else {
    None
  };

  let l2_current = if let Some(node) = l2{
    sum += node.val;
    node.next
  } else {
    None
  };

  let mut result = ListNode::new(sum%10);

  if l1_current.is_some() || l2_current.is_some() || sum>9 {
    result.next = add(l1_current, l2_current, sum/10);
  }  

  Some(Box::new(result))
}

pub fn add_two_numbers_2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut l1_current = l1;
  let mut l2_current = l2;
  let mut carry = 0;
  let mut result = Some(Box::new(ListNode::new(0)));
  let mut current = result.as_mut();

  while l1_current.is_some() || l2_current.is_some() {
    let mut sum = carry;
    if let Some(node) = l1_current{
      sum += node.val;
      l1_current = node.next;
    }

    if let Some(node) = l2_current{
      sum += node.val;
      l2_current = node.next;
    }

    carry = sum/10;

    if let Some(node) = current {
      node.next = Some(Box::new(ListNode::new(sum % 10)));
      current = node.next.as_mut();
    }
  }

  if carry>0 {
    current.unwrap().next = Some(Box::new(ListNode::new(carry)));
  }

  result.unwrap().next
}


pub fn add_two_numbers_1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let val1 = l1.unwrap();
    let val2 = l2.unwrap();
    let sum = val1.val +val2.val;

    if val1.next.is_none() && val2.next.is_none(){
      if sum>9{
        return Some(Box::new(ListNode {
                                        val: 1,
                                        next: Some(Box::new(ListNode::new(sum%10)))}));
      }
      return Some(Box::new(ListNode::new(sum)));
    }

    if val1.next.is_none(){
      return val2.next;
    }

    if val2.next.is_none(){
      return val1.next;
    }

    if sum>9{
      return Some(Box::new(ListNode {
                                      val: 1,
                                      next: Some(Box::new(ListNode { 
                                                                      val: sum%10,
                                                                      next: add_two_numbers_1(val1.next, val2.next)
                                                                   }))}));
    }
    Some(Box::new(ListNode { val: sum, next: add_two_numbers_1(val1.next, val2.next)}))
}

fn main() {
    let l1 = Some(Box::new(ListNode {
                                        val: 2,
                                        next: Some(Box::new(ListNode {
                                                                        val: 4,
                                                                        next: Some(Box::new(ListNode::new(3)))
                                                                      }))
                                    }));
                                    
    let l2 = Some(Box::new(ListNode {
                                        val: 5,
                                        next: Some(Box::new(ListNode {
                                                                        val: 6,
                                                                        next: Some(Box::new(ListNode::new(4)))
                                                                      }))
                                    }));
                                    
    println!("Hello, world! {:?}", add_two_numbers(l1, l2));
}
