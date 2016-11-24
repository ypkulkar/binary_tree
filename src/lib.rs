//use std::cmp::Ordering::*;
///use T: std::fmt::Debug;
#[derive(Debug)]
pub struct Node<T> {
  key: T,
  left : Option<Box<Node<T>>>,
  right: Option<Box<Node<T>>>,
}
#[derive(Debug)]
pub struct Tree<T>{
  node: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    /// Creates an empty tree
    pub fn new(key: T) -> Self {
        Node{
        key :key,
        right: None,
        left : None,
        }

        }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {

        if key > self.key{
                 match self.right{
                         Some(ref mut n) => n.as_mut().insert(key),
                         None        => {self.right = Some(Box::new(Node::new(key)));return true;},
                                }
        }
                        else if key < self.key{
                match self.left{
                         Some(ref mut n) => n.as_mut().insert(key),
                         None        => {self.left = Some(Box::new(Node::new(key)));return true;},
                }
           }
        else {
             return false;
        }
        }
    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        if self.key == *key{
        true
        }
        else if *key > self.key{
        match self.right{
                Some(ref n) => n.find(key),
                None        => false,

        }
        }
        else{
        match self.left {
                Some(ref n) => n.find(key),
                None        => false,
                }
             }
        }
        
       
        pub fn preorder<'a>(&'a self, v: &mut Vec<&'a T>){
 
                        v.push(&self.key);
                            
                        match self.left {
                            Some(ref n) => {n.preorder(v);},
                            None => {},
                        }
                
                        match self.right {
                            Some(ref n) => {n.preorder(v);}, 
                            None => {},
                        }
                        
    }
    
    pub fn inorder<'a>(&'a self, v: &mut Vec<&'a T>){
        
                           match self.left {
                                Some(ref n) => {n.inorder(v);},
                                None => {},
                            }
                                        
                            v.push(&self.key);
                            
                            match self.right {
                                Some(ref n) => {n.inorder(v);}, 
                                None => {},
                            }
                        
                       
                    
                    
    }
    pub fn postorder<'a>(&'a self, v: &mut Vec<&'a T>) {
        
                            
                        match self.left {
                            Some(ref n) => {n.postorder(v);},
                            None => {},
                        }
                
                        match self.right {
                            Some(ref n) => {n.postorder(v);}, 
                            None => {},
                        }
                        v.push(&self.key);
    }
}
                        

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree{
        node: None,     }
        }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {

        match self.node{
                Some(ref mut n) => n.insert(key),
                None  => {self.node = Some(Box::new(Node::new(key))); return true;},
                }
 }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.node{
                Some(ref n) =>n.find(key),
                None => {return false;},
                }
        }
    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        //static v  = Vec::new();
         let mut v = Vec::<&T>::new();
             match self.node {
                              Some(ref n) => {n.preorder(&mut v);},
                                None => {},
                           }
        return v;
    }
/// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        //static v  = Vec::new();
         let mut v = Vec::<&T>::new();
             match self.node {
                              Some(ref n) => {n.inorder(&mut v);},
                                None => {},
                           }
        return v;
        }
    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        //static v  = Vec::new();
         let mut v = Vec::<&T>::new();
             match self.node {
                              Some(ref n) => {n.postorder(&mut v);},
                                None => {},
                           }
        return v;
    }
}
