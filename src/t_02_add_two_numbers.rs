// 两数相加
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
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut v = vec![];

    let mut addition = 0;

    let mut l1 = &l1;
    let mut l2 = &l2;

    loop {
        if *l1 == None && *l2 == None && addition == 0 {
            break;
        }

        let mut v1 = 0;
        let mut v2 = 0;

        if let Some(b1) = l1 {
            v1 = b1.val;
            l1 = &b1.next;
        }

        if let Some(b2) = l2 {
            v2 = b2.val;
            l2 = &b2.next;
        }

        println!("v1 = {v1}, v2 = {v2}");

        let mut next = ListNode::new(v1 + v2 + addition);

        if next.val > 9 {
            next.val -= 10;
            addition = 1;
        } else {
            addition = 0;
        }
        println!("next = {next:?}");
        v.push(next);
    }

    for item in &v {
        println!("item = {item:?}");
    }

    let mut a: Option<Box<ListNode>> = None;
    while let Some(mut b) = v.pop() {
        if a == None {
            a = Some(Box::new(b));
        } else {
            b.next = a;
            a = Some(Box::new(b));
        }
    }
    println!("a = {a:?}");
    a
}
