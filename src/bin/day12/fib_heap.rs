

pub struct FibHeap<T: Ord>
{
    trees: Vec<HeapTree<T>>
}

struct HeapTree<T: Ord>
{
    value: T,
    children: Vec<HeapTree<T>>
}
