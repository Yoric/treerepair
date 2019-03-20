use std::iter::FromIterator;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct Building;
pub struct Ready;

pub struct Forest<T> {
    graphs: Vec<DAG<T>>,
}
pub struct ForestBuilder<T> {
    forest: Forest<T>,
}
impl<T> ForestBuilder<T> {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn add_dag(&mut self) {
        unimplemented!()
    }
}
impl<T> ForestBuilder<T> {
    pub fn done(self) -> Result<Forest<T>, ()> {
        unimplemented!()
    }
}
impl<T> Forest<T> {
    pub fn iter(&self) -> impl Iterator<Item = &DAG<T>> {
        self.graphs.iter()
    }
    pub fn into_iter(self) -> impl Iterator<Item = DAG<T>> {
        self.graphs.into_iter()
    }
}
impl<T> FromIterator<DAG<T>> for Forest<T> {
    fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item=DAG<T>> {
        unimplemented!()
    }
}

#[derive(Clone, Copy)]
pub struct NodeIndex<T> {
    index: usize,
    phantom: PhantomData<T>
}
impl<T> NodeIndex<T> {
    fn cast<U>(self) -> NodeIndex<U> {
        NodeIndex {
            index: self.index,
            phantom: PhantomData,
        }
    }
}
pub struct Node<T> {
    index: NodeIndex<T>,
    label: T,
    children: Vec<NodeIndex<T>>,
    parent: Option<NodeIndex<T>>,
}
impl<T> Node<T> {
    pub fn label(&self) -> &T {
        &self.label
    }
    pub fn with<F, U>(self, f: F) -> Node<U> where F: Fn(T) -> U {
        Node {
            index: self.index.cast(),
            label: f(self.label),
            children: self.children.into_iter().map(NodeIndex::cast).collect(),
            parent: self.parent.map(NodeIndex::cast),
        }
    }
}

pub struct DAG<T> {
    nodes: Vec<Node<T>>,
}
impl<T> DAG<T> {
    pub fn insert_node(&mut self, label: T) -> NodeIndex<T> {
        unimplemented!()
    }
    pub fn add_children(&mut self, node: NodeIndex<T>, nodes: &[NodeIndex<T>]) {
        unimplemented!()
    }
    pub fn into_iter(self) -> impl Iterator<Item = Node<T>> {
        self.nodes.into_iter()
    }
}
impl<T> FromIterator<Node<T>> for DAG<T> {
    fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item=Node<T>> {
        unimplemented!()
    }
}

