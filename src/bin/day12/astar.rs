use std::collections::{BinaryHeap, HashMap};

pub trait SearchProblem<'a> {
    type Node: PartialEq + Eq + std::fmt::Debug + std::hash::Hash + Clone;
    type SuccessorIter: Iterator<Item=Self::Node>;

    fn start(&self) -> Self::Node;
    fn goal(&self) -> Self::Node;

    fn distance_heuristic(&self, n: &Self::Node) -> usize;
    fn successors(&'a self, n: Self::Node) -> Self::SuccessorIter;
}

struct NodeCost<'a, P: SearchProblem<'a>> {
    problem: &'a P,
    node: P::Node,
    path_cost: usize
}

impl<'a, P:SearchProblem<'a>> NodeCost<'a, P> {
    fn new(problem: &'a P, node: P::Node, path_cost: usize) -> Self {
        Self { problem, node, path_cost }
    }
}

impl<'a, P:SearchProblem<'a>> NodeCost<'a, P> {
    fn heuristic_cost(&self) -> usize {
        self.problem.distance_heuristic(&self.node)
    }

    fn cost_estimate(&self) -> usize {
        self.path_cost + self.heuristic_cost()
    }
}

impl<'a,P:SearchProblem<'a>> PartialEq for NodeCost<'a, P> {
    fn eq(&self, other: &Self) -> bool {
        self.cost_estimate() == other.cost_estimate()
    }
}

impl<'a, P:SearchProblem<'a>> Eq for NodeCost<'a, P> {
}

impl<'a, P:SearchProblem<'a>> PartialOrd for NodeCost<'a, P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(usize::cmp(&self.cost_estimate(), &other.cost_estimate()).reverse())
    }
}

impl<'a,P:SearchProblem<'a>> Ord for NodeCost<'a, P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn astar<'a,P:SearchProblem<'a>>(p: &'a P) -> Option<usize> {
    let mut iter_count = 0;
    let mut open = BinaryHeap::from([NodeCost::new(p, p.start(), 0)]);
    let mut min_costs : HashMap<P::Node, usize> = HashMap::new();
    min_costs.insert(p.start(), 0);

    while let Some(current) = open.pop() {
        iter_count += 1;
        if current.node == p.goal() {
            //println!("Solve search in {} iterations", iter_count);
            return Some(current.path_cost)
        }

        for succ in p.successors(current.node) {
            let succ_cost = current.path_cost + 1;
            if let Some(prev_cost) = min_costs.get(&succ) {
                if *prev_cost <= succ_cost {
                    continue;
                }
            }
            min_costs.insert(succ.clone(), succ_cost);
            open.push(NodeCost::new(p, succ, succ_cost));
        }
    }

    //println!("No solution after {} iterations", iter_count);
    None
}
