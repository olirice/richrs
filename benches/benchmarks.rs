//! Benchmarks for richrs components.

#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use richrs::prelude::*;

fn benchmark_style_parse(c: &mut Criterion) {
    c.bench_function("style_parse_simple", |b| {
        b.iter(|| Style::parse(black_box("bold red")))
    });

    c.bench_function("style_parse_complex", |b| {
        b.iter(|| Style::parse(black_box("bold italic underline red on white")))
    });
}

fn benchmark_markup_parse(c: &mut Criterion) {
    c.bench_function("markup_parse_simple", |b| {
        b.iter(|| {
            let markup = Markup::from(black_box("[bold]Hello[/]"));
            markup.to_text()
        })
    });

    c.bench_function("markup_parse_complex", |b| {
        b.iter(|| {
            let markup =
                Markup::from(black_box("[bold red]Hello[/] [italic blue]World[/]"));
            markup.to_text()
        })
    });
}

fn benchmark_text_operations(c: &mut Criterion) {
    c.bench_function("text_append", |b| {
        b.iter(|| {
            let mut text = Text::new();
            for i in 0..100 {
                text.append_plain(&format!("item{i} "));
            }
            text
        })
    });

    c.bench_function("text_to_segments", |b| {
        let mut text = Text::from_str("Hello world, this is a test string");
        text.stylize(0, 5, Style::new().bold());
        text.stylize(6, 11, Style::new().italic());

        b.iter(|| text.to_segments())
    });
}

fn benchmark_panel_render(c: &mut Criterion) {
    c.bench_function("panel_render_simple", |b| {
        let panel = Panel::new("Hello, World!");
        b.iter(|| panel.render(black_box(80)))
    });

    c.bench_function("panel_render_with_title", |b| {
        let panel = Panel::new("Hello, World!")
            .title("Title")
            .subtitle("Subtitle");
        b.iter(|| panel.render(black_box(80)))
    });
}

fn benchmark_table_render(c: &mut Criterion) {
    c.bench_function("table_render_small", |b| {
        let mut table = Table::new();
        table.add_column(Column::new("A"));
        table.add_column(Column::new("B"));
        table.add_row_cells(["1", "2"]);
        table.add_row_cells(["3", "4"]);

        b.iter(|| table.render(black_box(80)))
    });

    c.bench_function("table_render_medium", |b| {
        let mut table = Table::new();
        for col in ["A", "B", "C", "D", "E"] {
            table.add_column(Column::new(col));
        }
        for _ in 0..10 {
            table.add_row_cells(["x", "y", "z", "w", "v"]);
        }

        b.iter(|| table.render(black_box(120)))
    });
}

criterion_group!(
    benches,
    benchmark_style_parse,
    benchmark_markup_parse,
    benchmark_text_operations,
    benchmark_panel_render,
    benchmark_table_render,
);
criterion_main!(benches);
