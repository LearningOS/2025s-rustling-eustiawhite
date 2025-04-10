/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 在当前节点上插入一个值
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left_child) = self.left {
                    left_child.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right_child) = self.right {
                    right_child.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {
                // 重复的元素不做处理（保持树中只存一份）
            }
        }
    }

    // 在当前节点上查找一个值，返回 true 表示找到了
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => {
                if let Some(ref left_child) = self.left {
                    left_child.search(value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(ref right_child) = self.right {
                    right_child.search(value)
                } else {
                    false
                }
            }
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 插入一个值到 BST 中
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // 在 BST 中查找一个值
    fn search(&self, value: T) -> bool {
        if let Some(ref node) = self.root {
            node.search(value)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 检查在树为空时搜索某个值，应该返回 false
        assert_eq!(bst.search(1), false);

        // 插入几个值
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 检查插入的值都能被搜索到
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 搜索未插入的值应返回 false
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 插入重复值
        bst.insert(1);
        bst.insert(1);

        // 值 1 应当存在
        assert_eq!(bst.search(1), true);

        // 同时，根节点的左右子树不应存在重复节点
        match bst.root {
            Some(ref node) => {
                // 如果左右子树存在，说明插入了重复节点（错误情况），这里应该没有左右子树
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
