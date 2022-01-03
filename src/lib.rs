use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: usize,
    pub children: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: usize) -> Self {
        TreeNode {
            val,
            children: vec![],
        }
    }

    pub fn add_child_value(&mut self, val: usize) {
        let node = TreeNode::new(val);
        self.children.push(Self::tree_node_wrap(node));
    }

    pub fn add_child_node(&mut self, node: Option<Rc<RefCell<Self>>>) {
        self.children.push(node);
    }

    pub fn tree_node_wrap(node: TreeNode) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(node)))
    }
}

pub struct Solution {}

/* key takeaways
   - serialize
     - traverse the tree using DFS and in pre-order
     - add the number of children to the output so
       it is easier for you to re-build the tree
       back
   - deserialize
     - we need to pick a data structure that's
       easy for us to build the tree back
     - don't over-think this; just build the tree
       recursively
     - we pick VecDeque and pop two entries from
       the front: (val, count). Those two entries
       allow you to build the tree from the top
       down
*/

impl Solution {
    pub fn serialize(root: &Option<Rc<RefCell<TreeNode>>>) -> VecDeque<usize> {
        let mut output: VecDeque<usize> = VecDeque::new();
        Self::dfs(root, &mut output);
        output
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, output: &mut VecDeque<usize>) {
        if let Some(node_rc) = node {
            let node = node_rc.borrow();
            /* capture the val and the count */
            output.push_back(node.val);
            output.push_back(node.children.len());
            for child in &node.children {
                Self::dfs(child, output);
            }
        }
    }

    pub fn deserialize(input: &mut VecDeque<usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if input.len() == 0 {
            return None;
        }
        Self::build_tree(input)
    }

    fn build_tree(input: &mut VecDeque<usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if input.len() == 0 {
            return None;
        }

        let val = input.pop_front().unwrap();
        let count = input.pop_front().unwrap();
        /* leaf */
        if count == 0 {
            return TreeNode::tree_node_wrap(TreeNode::new(val));
        }

        let mut root = TreeNode::new(val);
        /*
          - you have "count" child nodes need to
            be built recursively
        */
        for _ in 0..count {
            root.add_child_node(Self::build_tree(input));
        }
        TreeNode::tree_node_wrap(root)
    }

    pub fn test_fixture_1() -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = TreeNode::new(1);
        let mut c1 = TreeNode::new(3);
        c1.add_child_value(5);
        c1.add_child_value(6);
        root.add_child_node(TreeNode::tree_node_wrap(c1));
        root.add_child_value(2);
        root.add_child_value(4);
        TreeNode::tree_node_wrap(root)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let mut output_array = Solution::serialize(&Solution::test_fixture_1());
        let result = Solution::deserialize(&mut output_array);
        assert_eq!(result, Solution::test_fixture_1());
    }
}
