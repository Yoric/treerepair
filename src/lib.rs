//! An implementation of TreeRePair http://www.eti.uni-siegen.de/ti/veroeffentlichungen/12-repair.pdf

// This implementation consists in the following steps:
//
// 1. Import the tree to compress.
// 2.

pub mod forest;

use forest::*;

#[derive(Default)]
pub struct Options {
    /// If specified, reduce all nodes (including lists) to ensure that they have at
    /// most `max_rank` children. The original paper recomments a value of 4.
    max_rank: Option<usize>,
}
impl Options {
    /// Specify a max rank for children.
    ///
    /// If `Some(v)`, reduce all nodes (including lists) to ensure that they have
    /// at most `max_rank` children. The original paper recommends a value of 4.
    ///
    /// Default: `None`.
    pub fn with_max_rank(self, max_rank: Option<usize>) -> Self {
        Options {
            max_rank,
            ..self
        }
    }
}

pub enum ExtendedLabel<T> {
    Original(T),
    DiGraph(DiGraph<T>),
}

pub struct DiGraph<T> {
    index: NodeIndex<ExtendedLabel<T>>,
    parent: NodeIndex<ExtendedLabel<T>>,
    position: usize,
    child: NodeIndex<ExtendedLabel<T>>,
}

pub struct TreeRePair<T> {
    options: Options,
    forest: Forest<ExtendedLabel<T>>,
    digraphs: Vec<DiGraph<ExtendedLabel<T>>>,
}
impl<T> TreeRePair<T> {
    pub fn new(options: Options, forest: Forest<T>) -> Self {
        let forest = forest.into_iter()
            .map(|dag|
                dag.into_iter()
                    .map(|node| {
                        node.with(ExtendedLabel::Original)
                    })
                .collect()
            )
            .collect();
        TreeRePair {
            options,
            forest,
            digraphs: Vec::new(),
        }
    }

    /// Attempt to compress further.
    ///
    /// Return the number of nodes squeezed. This number always decreases.
    pub fn squeeze(&mut self) -> usize {
        unimplemented!()
    }

    pub fn forest(&self) -> &Forest<ExtendedLabel<T>> {
        unimplemented!()
    }
}
