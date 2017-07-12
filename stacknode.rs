#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Clone, Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl <T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode {
            val: val, next:None
        }
    }
}

impl <T> Stack<T> {
    fn new() -> Stack<T> {
        Stack{top: None}
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            },
            None => None,
        }
    }
}

fn main() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct{
        a: i32,
    }

    let a = TestStruct{a: 5};
    let b = TestStruct{a: 8};

    let mut sn = Stack::<&TestStruct>::new();

    assert_eq!(sn.pop(), None);

    sn.push(&a);
    sn.push(&b);
    println!("stackNode is {:?}", sn);

    assert_eq!(sn.pop(), Some(&b));
    println!("stackNode is {:?}", sn);

    assert_eq!(sn.pop(), Some(&b));
    println!("stackNode is {:?}", sn);


}