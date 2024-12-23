use std::borrow::Cow;

use tap::Pipe;

#[derive(Debug)]
pub struct NodeSet<'a>(Cow<'a, [bool]>);

#[derive(Debug)]
pub struct EdgeMap(Vec<bool>);

pub fn parse(input: &crate::int::Graph) -> (EdgeMap, NodeSet<'static>) {
    let (mut nodes, mut edges) = (vec![false; 676], vec![false; 456_976]);
    for edge in input.all_edges() {
        let (from, to) = ((edge.from as usize), (edge.to as usize));
        edges[from * 676 + to] = true;
        nodes[from] = true;
        nodes[to] = true;
    }
    (
        EdgeMap(edges),
        Cow::<'static, [bool]>::Owned(nodes).pipe(NodeSet),
    )
}

pub(crate) fn cliques(edges: &EdgeMap, nodes: &NodeSet) -> impl Iterator<Item = Vec<u16>> {
    let mut res: Vec<Vec<u16>> = Vec::with_capacity(520);
    for pc in nodes.iter() {
        let conns = edges.get(pc);
        for set in &mut res {
            if set.iter().copied().all(|other| conns.contains(other)) {
                set.push(pc);
            }
        }
        let mut new = Vec::with_capacity(16);
        new.push(pc);
        res.push(new);
    }
    res.into_iter()
}

impl EdgeMap {
    pub(crate) fn get(&self, from: u16) -> NodeSet<'_> {
        self.0[((from as usize) * 676)..((from as usize + 1) * 676)]
            .pipe_ref(Cow::Borrowed)
            .pipe(NodeSet)
    }

    pub fn iter(&self) -> impl Iterator<Item = (u16, NodeSet<'_>)> {
        (0..676).map(|k| (k, self.get(k)))
    }

    pub fn prefixed_by(&self, c: u8) -> impl Iterator<Item = (u16, NodeSet<'_>)> {
        let c = u16::from(c - b'a');
        ((c * 26)..((c + 1) * 26)).map(|k| (k, self.get(k)))
    }
}

impl NodeSet<'_> {
    #[must_use]
    pub fn contains(&self, id: u16) -> bool {
        self.0[id as usize]
    }

    #[allow(
        clippy::cast_possible_truncation,
        reason = "check it once at the begining"
    )]
    pub fn iter(&self) -> impl Iterator<Item = u16> + '_ {
        debug_assert!(u16::try_from(self.0.len()).is_ok());
        self.0
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, b)| *b)
            .map(|(idx, _)| idx as u16)
    }
}

impl FromIterator<crate::int::Edge> for EdgeMap {
    fn from_iter<T: IntoIterator<Item = crate::int::Edge>>(iter: T) -> Self {
        let mut v = vec![false; 456_976];
        for edge in iter {
            let (from, to) = ((edge.from as usize), (edge.to as usize));
            v[from * 676 + to] = true;
        }

        Self(v)
    }
}
