/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        self.elements.first().ok_or("Queue is empty")
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    // push 操作：
    // 将新元素加入 q2，然后将 q1 中所有元素依次出队并入队到 q2，
    // 接着交换 q1 和 q2，使得 q1 中的顺序为栈的 LIFO（后进先出）顺序。
    pub fn push(&mut self, elem: T) {
        self.q2.enqueue(elem);
        while !self.q1.is_empty() {
            let item = self.q1.dequeue().unwrap();
            self.q2.enqueue(item);
        }
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    // pop 操作直接从 q1 中出队，并转换错误信息为 "Stack is empty"
    pub fn pop(&mut self) -> Result<T, &str> {
        self.q1.dequeue().or_else(|_| Err("Stack is empty"))
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_queue(){
        let mut s = myStack::<i32>::new();
        // 初始栈为空，应返回错误 "Stack is empty"
        assert_eq!(s.pop(), Err("Stack is empty"));
        
        s.push(1);
        s.push(2);
        s.push(3);
        
        // 后进先出，首先弹出 3，再弹出 2
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        
        s.push(4);
        s.push(5);
        
        // 栈不为空
        assert_eq!(s.is_empty(), false);
        
        // 弹出 5、4、1 分别对应入栈顺序（最新的先出）
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        
        // 此时栈为空，pop 应返回错误
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
