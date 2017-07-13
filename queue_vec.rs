#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
}

impl <T> Queue<T> {
    fn new() -> Self {
        Queue { qdata: Vec::new() }
    }

    fn push(&mut self, data: T) {
        self.qdata.push(data);
    }

    fn pop(&mut self) -> Option<T> {
         if self.qdata.len() == 0 {return None}
         Some(self.qdata.remove(0))   
            
        
    }

}

fn main(){
    let mut q = Queue::new();
    q.push(1);
    println!("now, queue is {:?}", q);
    q.push(2);
    println!("now, queue is {:?}", q);
    q.pop();
    println!("now, queue is {:?}", q);
    q.pop();
    println!("now, queue is {:?}", q);
    // q.pop();
    // println!("now, queue is {:?}", q.qdata);
    
}