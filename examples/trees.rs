//! Trees demo
use richrs::prelude::*;

fn main() -> Result<()> {
    let mut console = Console::new();

    console.print("")?;

    let mut tree = Tree::new("project");
    tree.add(
        TreeNode::new("src")
            .with_child(TreeNode::new("main.rs"))
            .with_child(TreeNode::new("lib.rs")),
    );
    tree.add(
        TreeNode::new("tests")
            .with_child(TreeNode::new("integration.rs")),
    );
    tree.add(TreeNode::new("Cargo.toml"));
    tree.add(TreeNode::new("README.md"));

    console.write_segments(&tree.render())?;
    console.print("")?;

    Ok(())
}
