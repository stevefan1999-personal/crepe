use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use crepe::crepe;

#[cfg(feature = "heapless")]
const FIXED_CAPACITY: usize = 256;

crepe! {
    @input
    struct Edge(u16, u16, u16);

    @output
    struct Reachable(u16, u16);

    @output
    struct ReverseReachable(u16, u16);

    @output
    struct TwoHop(u16, u16);

    @output
    struct ThreeHop(u16, u16);

    @output
    struct CheapTwoHop(u16, u16);

    @output
    struct Endpoint(u16);

    @output
    struct Node(u16);

    Node(x) <- Edge(x, _, _);
    Node(y) <- Edge(_, y, _);

    TwoHop(x, z) <- Edge(x, y, _), Edge(y, z, _);
    ThreeHop(x, w) <- TwoHop(x, z), Edge(z, w, _);
    CheapTwoHop(x, z) <- Edge(x, y, w1), Edge(y, z, w2), (w1 + w2 <= 5);

    Reachable(x, y) <- Edge(x, y, _);
    Reachable(x, z) <- Reachable(x, y), Edge(y, z, _);
    ReverseReachable(y, x) <- Reachable(x, y);

    Endpoint(x) <- Node(x), !Edge(x, _, _);
}

fn graph_edges(nodes: u16) -> Vec<Edge> {
    let mut edges = Vec::new();
    for node in 0..nodes {
        if node + 1 < nodes {
            edges.push(Edge(node, node + 1, 1 + node % 3));
        }
        if node + 2 < nodes {
            edges.push(Edge(node, node + 2, 2 + node % 5));
        }
    }
    edges
}

#[cfg(feature = "std")]
fn run_std(edges: &[Edge]) -> usize {
    let mut runtime = Crepe::<crepe::StdCrepeCollections>::new_with_collections();
    runtime.extend(edges.iter().copied());
    let (reachable, reverse, two_hop, three_hop, cheap_two_hop, endpoints, nodes) = runtime.run();
    reachable.len()
        + reverse.len()
        + two_hop.len()
        + three_hop.len()
        + cheap_two_hop.len()
        + endpoints.len()
        + nodes.len()
}

#[cfg(feature = "fnv")]
fn run_fnv(edges: &[Edge]) -> usize {
    let mut runtime = Crepe::<crepe::FnvCrepeCollections>::new_with_collections();
    runtime.extend(edges.iter().copied());
    let (reachable, reverse, two_hop, three_hop, cheap_two_hop, endpoints, nodes) = runtime.run();
    reachable.len()
        + reverse.len()
        + two_hop.len()
        + three_hop.len()
        + cheap_two_hop.len()
        + endpoints.len()
        + nodes.len()
}

#[cfg(feature = "hashbrown")]
fn run_hashbrown(edges: &[Edge]) -> usize {
    let mut runtime = Crepe::<crepe::HashbrownCrepeCollections>::new_with_collections();
    runtime.extend(edges.iter().copied());
    let (reachable, reverse, two_hop, three_hop, cheap_two_hop, endpoints, nodes) = runtime.run();
    reachable.len()
        + reverse.len()
        + two_hop.len()
        + three_hop.len()
        + cheap_two_hop.len()
        + endpoints.len()
        + nodes.len()
}

#[cfg(feature = "heapless")]
fn run_heapless(edges: &[Edge]) -> usize {
    let mut runtime =
        Crepe::<crepe::HeaplessCrepeCollections<FIXED_CAPACITY>>::new_with_collections();
    runtime.extend(edges.iter().copied());
    let (reachable, reverse, two_hop, three_hop, cheap_two_hop, endpoints, nodes) = runtime.run();
    reachable.iter().count()
        + reverse.iter().count()
        + two_hop.iter().count()
        + three_hop.iter().count()
        + cheap_two_hop.iter().count()
        + endpoints.iter().count()
        + nodes.iter().count()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("collection_backends/complex_graph");

    for nodes in [12u16, 16, 20] {
        let edges = graph_edges(nodes);

        #[cfg(feature = "std")]
        group.bench_with_input(BenchmarkId::new("std_hash", nodes), &edges, |b, edges| {
            b.iter(|| black_box(run_std(black_box(edges))));
        });

        #[cfg(feature = "fnv")]
        group.bench_with_input(BenchmarkId::new("fnv_hash", nodes), &edges, |b, edges| {
            b.iter(|| black_box(run_fnv(black_box(edges))));
        });

        #[cfg(feature = "hashbrown")]
        group.bench_with_input(
            BenchmarkId::new("hashbrown_fnv", nodes),
            &edges,
            |b, edges| {
                b.iter(|| black_box(run_hashbrown(black_box(edges))));
            },
        );

        #[cfg(feature = "heapless")]
        group.bench_with_input(BenchmarkId::new("heapless", nodes), &edges, |b, edges| {
            b.iter(|| black_box(run_heapless(black_box(edges))));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
