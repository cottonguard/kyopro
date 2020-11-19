pub fn strongly_connected_components(g: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut ord = Vec::with_capacity(g.len());
    let mut vis = vec![false; g.len()];
    for u in 0..g.len() {
        if !vis[u] {
            dfs_f(g, u, &mut ord, &mut vis);
        }
    }
    let mut comps = Vec::new();
    let mut h = vec![Vec::new(); g.len()];
    for u in 0..g.len() {
        for &v in &g[u] {
            h[v].push(u);
        }
    }
    vis.iter_mut().for_each(|v| *v = false);
    for u in ord.into_iter().rev() {
        if !vis[u] {
            let mut comp = Vec::new();
            dfs_c(&h, u, &mut comp, &mut vis);
            comps.push(comp);
        }
    }
    comps
}
fn dfs_f(g: &[Vec<usize>], u: usize, ord: &mut Vec<usize>, vis: &mut [bool]) {
    vis[u] = true;
    for &v in &g[u] {
        if !vis[v] {
            dfs_f(g, v, ord, vis);
        }
    }
    ord.push(u);
}
fn dfs_c(h: &[Vec<usize>], u: usize, comp: &mut Vec<usize>, vis: &mut [bool]) {
    vis[u] = true;
    comp.push(u);
    for &v in &h[u] {
        if !vis[v] {
            dfs_c(h, v, comp, vis);
        }
    }
}
