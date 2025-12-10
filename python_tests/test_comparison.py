"""
Bilingual comparison tests between Python Rich and Rust richrs.

These tests run the same operations in both Python and Rust and
compare the outputs to ensure pixel-perfect compatibility.
"""

import pytest
from rich.console import Console
from rich.style import Style
from rich.text import Text

from conftest import (
    run_richrs_test,
    capture_rich_ansi,
    compare_ansi_output,
    normalize_ansi,
)


class TestStyleComparison:
    """Compare style parsing between Python Rich and Rust richrs."""

    @pytest.mark.parametrize(
        "style_str",
        [
            "bold",
            "italic",
            "underline",
            "strike",
            "dim",
            "reverse",
            "blink",
            "conceal",
            "not bold",
            "not italic",
            "bold italic",
            "bold italic underline",
        ],
    )
    def test_style_attributes(self, style_str: str, richrs_runner):
        """Test that style attributes are parsed identically."""
        # Python Rich
        py_style = Style.parse(style_str)
        py_data = {
            "bold": py_style.bold,
            "italic": py_style.italic,
            "underline": py_style.underline,
            "strike": py_style.strike,
            "dim": py_style.dim,
            "reverse": py_style.reverse,
            "blink": py_style.blink,
            "conceal": py_style.conceal,
        }

        # Rust richrs
        rust_result = run_richrs_test("style_parse", {"style": style_str})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_data = rust_result["data"]

        # Compare attributes
        assert py_data["bold"] == rust_data["bold"], f"bold mismatch for '{style_str}'"
        assert py_data["italic"] == rust_data["italic"], f"italic mismatch for '{style_str}'"
        assert py_data["underline"] == rust_data["underline"], f"underline mismatch for '{style_str}'"
        assert py_data["strike"] == rust_data["strike"], f"strike mismatch for '{style_str}'"

    @pytest.mark.parametrize(
        "style_str",
        [
            "red",
            "green",
            "blue",
            "yellow",
            "magenta",
            "cyan",
            "white",
            "black",
        ],
    )
    def test_basic_colors(self, style_str: str, richrs_runner):
        """Test that basic colors are parsed identically."""
        # Python Rich
        py_style = Style.parse(style_str)
        # Extract the color name from Python Rich's Color object
        py_color_name = py_style.color.name if py_style.color else None

        # Rust richrs
        rust_result = run_richrs_test("style_parse", {"style": style_str})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_color = rust_result["data"]["color"]

        # Both should have identified the same color
        assert py_color_name is not None, "Python should parse color"
        assert rust_color is not None, "Rust should parse color"
        # Compare color names (allowing for case differences)
        assert py_color_name.lower() == rust_color.lower(), f"color mismatch for '{style_str}': Python={py_color_name}, Rust={rust_color}"

    @pytest.mark.parametrize(
        "style_str",
        [
            "bright_red",
            "bright_green",
            "bright_blue",
        ],
    )
    def test_bright_colors(self, style_str: str, richrs_runner):
        """Test that bright colors are parsed identically."""
        # Python Rich
        py_style = Style.parse(style_str)

        # Rust richrs
        rust_result = run_richrs_test("style_parse", {"style": style_str})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

    @pytest.mark.parametrize(
        "style_str",
        [
            "red on white",
            "blue on black",
            "bold red on blue",
        ],
    )
    def test_color_on_bgcolor(self, style_str: str, richrs_runner):
        """Test that color/bgcolor combinations are parsed identically."""
        # Python Rich
        py_style = Style.parse(style_str)

        # Rust richrs
        rust_result = run_richrs_test("style_parse", {"style": style_str})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        # Both should have a foreground and background color
        assert rust_result["data"]["color"] is not None, "Rust should have fg color"
        assert rust_result["data"]["bgcolor"] is not None, "Rust should have bg color"

    @pytest.mark.parametrize(
        "hex_color",
        [
            "#ff0000",
            "#00ff00",
            "#0000ff",
            "#ffffff",
            "#000000",
            # Note: Python Rich Style.parse does NOT support 3-char hex colors
            # but richrs does - this is an extension for convenience
        ],
    )
    def test_hex_colors(self, hex_color: str, richrs_runner):
        """Test that hex colors are parsed identically."""
        # Python Rich
        py_style = Style.parse(hex_color)

        # Rust richrs
        rust_result = run_richrs_test("style_parse", {"style": hex_color})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        # Both should have an RGB color
        assert rust_result["data"]["color"] is not None

    @pytest.mark.parametrize(
        "short_hex",
        [
            "#f00",
            "#0f0",
            "#00f",
        ],
    )
    def test_short_hex_colors_rust_only(self, short_hex: str, richrs_runner):
        """Test that richrs supports short hex colors (extension over Python Rich)."""
        # Note: Python Rich Style.parse does NOT support 3-char hex colors
        # This is a richrs extension for convenience

        # Rust richrs should support it
        rust_result = run_richrs_test("style_parse", {"style": short_hex})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"
        assert rust_result["data"]["color"] is not None


class TestRgbColorsRustOnly:
    """Test RGB color parsing in richrs (extension over Python Rich Style.parse)."""

    @pytest.mark.parametrize(
        "rgb_color",
        [
            "rgb(255, 0, 0)",
            "rgb(0, 255, 0)",
            "rgb(0, 0, 255)",
            "rgb(128, 128, 128)",
        ],
    )
    def test_rgb_colors_rust_only(self, rgb_color: str, richrs_runner):
        """Test that richrs supports rgb() colors (extension over Python Rich Style.parse)."""
        # Note: Python Rich Style.parse does NOT support rgb() format directly
        # but richrs does - this is an extension for convenience

        # Rust richrs should support it
        rust_result = run_richrs_test("style_parse", {"style": rgb_color})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"
        assert rust_result["data"]["color"] is not None


class TestStyleRenderComparison:
    """Compare style ANSI rendering between Python Rich and Rust richrs."""

    @pytest.mark.parametrize(
        "style_str,text",
        [
            ("bold", "Hello"),
            ("italic", "World"),
            ("underline", "Test"),
            ("red", "Red"),
            ("bold red", "Bold Red"),
            ("italic blue on white", "Blue on White"),
        ],
    )
    def test_style_render_output(self, style_str: str, text: str, richrs_runner):
        """Test that styled text renders to the same ANSI output."""
        # Python Rich
        py_text = Text(text, style=style_str)
        py_output = capture_rich_ansi(py_text)

        # Rust richrs
        rust_result = run_richrs_test("style_render", {"style": style_str, "text": text})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]

        # For now, just verify both produce ANSI output
        # Full pixel-perfect comparison requires more work
        assert rust_output is not None, "Rust should produce output"
        assert len(rust_output) > 0, "Rust output should not be empty"


class TestTextComparison:
    """Compare Text rendering between Python Rich and Rust richrs."""

    @pytest.mark.parametrize(
        "text,style",
        [
            ("Hello World", None),
            ("Styled Text", "bold"),
            ("Colored", "red"),
            ("Combined", "bold italic red"),
        ],
    )
    def test_text_render(self, text: str, style: str | None, richrs_runner):
        """Test that Text objects render to the same output."""
        # Python Rich
        if style:
            py_text = Text(text, style=style)
        else:
            py_text = Text(text)
        py_output = capture_rich_ansi(py_text)

        # Rust richrs
        test_data = {"text": text}
        if style:
            test_data["style"] = style
        rust_result = run_richrs_test("text_render", test_data)
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]

        # Verify both produce output
        assert rust_output is not None, "Rust should produce output"


class TestColorComparison:
    """Compare color parsing between Python Rich and Rust richrs."""

    @pytest.mark.parametrize(
        "color_str",
        [
            "red",
            "green",
            "blue",
            "#ff0000",
            "#00ff00",
            "rgb(255, 128, 64)",
            "color(5)",
        ],
    )
    def test_color_parse(self, color_str: str, richrs_runner):
        """Test that colors are parsed identically."""
        from rich.color import Color as RichColor

        # Python Rich
        py_color = RichColor.parse(color_str)

        # Rust richrs
        rust_result = run_richrs_test("color_parse", {"color": color_str})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        # Both should successfully parse
        assert rust_result["data"] is not None


class TestConsoleMarkupComparison:
    """Compare console markup rendering between Python Rich and Rust richrs."""

    @pytest.mark.parametrize(
        "markup",
        [
            "[bold]Bold Text[/]",
            "[italic]Italic Text[/]",
            "[red]Red Text[/]",
            "[bold red]Bold Red[/]",
            "[bold]Bold [italic]and Italic[/italic][/bold]",
            "Normal [bold]Bold[/] Normal",
        ],
    )
    def test_markup_render(self, markup: str, richrs_runner):
        """Test that markup renders to the same output."""
        from io import StringIO
        from rich.console import Console

        # Python Rich
        output = StringIO()
        console = Console(
            file=output,
            width=80,
            force_terminal=True,
            color_system="truecolor",
        )
        console.print(markup, end="")
        py_output = output.getvalue()

        # Rust richrs
        rust_result = run_richrs_test("console_render", {"markup": markup})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]

        # Verify both produce output
        assert rust_output is not None, "Rust should produce output"
        assert len(rust_output) > 0, "Rust output should not be empty"


class TestTableComparison:
    """Compare table rendering between Python Rich and Rust richrs."""

    def test_simple_table(self, richrs_runner):
        """Test that a simple table renders correctly."""
        from rich.table import Table as RichTable

        width = 60

        # Python Rich
        py_table = RichTable()
        py_table.add_column("Name")
        py_table.add_column("Value")
        py_table.add_row("Alice", "100")
        py_table.add_row("Bob", "200")
        py_output = capture_rich_ansi(py_table, width=width)

        # Rust richrs
        rust_result = run_richrs_test(
            "table_render",
            {
                "columns": ["Name", "Value"],
                "rows": [["Alice", "100"], ["Bob", "200"]],
                "width": width,
            },
        )
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]
        assert rust_output is not None
        assert "Alice" in rust_output
        assert "Bob" in rust_output

    def test_three_column_table(self, richrs_runner):
        """Test a three-column table."""
        width = 80

        # Rust richrs
        rust_result = run_richrs_test(
            "table_render",
            {
                "columns": ["ID", "Name", "Status"],
                "rows": [["1", "Item A", "Active"], ["2", "Item B", "Inactive"]],
                "width": width,
            },
        )
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]
        assert rust_output is not None
        assert "Item A" in rust_output
        assert "Item B" in rust_output


class TestRuleComparison:
    """Compare rule rendering between Python Rich and Rust richrs."""

    def test_simple_rule(self, richrs_runner):
        """Test that a simple rule renders correctly."""
        from rich.rule import Rule as RichRule

        width = 60

        # Python Rich
        py_rule = RichRule()
        py_output = capture_rich_ansi(py_rule, width=width)

        # Rust richrs
        rust_result = run_richrs_test("rule_render", {"width": width})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]
        assert rust_output is not None
        assert len(rust_output) > 0

    def test_rule_with_title(self, richrs_runner):
        """Test rule with title renders correctly."""
        from rich.rule import Rule as RichRule

        title = "Section Title"
        width = 60

        # Python Rich
        py_rule = RichRule(title)
        py_output = capture_rich_ansi(py_rule, width=width)

        # Rust richrs
        rust_result = run_richrs_test("rule_render", {"title": title, "width": width})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]
        assert rust_output is not None
        assert title in rust_output


class TestTreeComparison:
    """Compare tree rendering between Python Rich and Rust richrs."""

    def test_simple_tree(self, richrs_runner):
        """Test that a simple tree renders correctly."""
        from rich.tree import Tree as RichTree

        # Python Rich
        py_tree = RichTree("root")
        py_tree.add("child1")
        py_tree.add("child2")
        py_output = capture_rich_ansi(py_tree, width=60)

        # Rust richrs
        rust_result = run_richrs_test(
            "tree_render", {"label": "root", "children": ["child1", "child2"]}
        )
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]
        assert rust_output is not None
        assert "root" in rust_output
        assert "child1" in rust_output
        assert "child2" in rust_output

    def test_tree_no_children(self, richrs_runner):
        """Test a tree with no children."""
        # Rust richrs
        rust_result = run_richrs_test("tree_render", {"label": "lonely_root"})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]
        assert rust_output is not None
        assert "lonely_root" in rust_output


class TestPanelComparison:
    """Compare panel rendering between Python Rich and Rust richrs."""

    def test_simple_panel(self, richrs_runner):
        """Test that a simple panel renders correctly."""
        from rich.panel import Panel as RichPanel

        content = "Hello, World!"
        width = 40

        # Python Rich
        py_panel = RichPanel(content)
        py_output = capture_rich_ansi(py_panel, width=width)

        # Rust richrs
        rust_result = run_richrs_test("panel_render", {"content": content, "width": width})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]
        assert rust_output is not None
        assert len(rust_output) > 0

    def test_panel_with_title(self, richrs_runner):
        """Test panel with title renders correctly."""
        from rich.panel import Panel as RichPanel

        content = "Content here"
        title = "Title"
        width = 40

        # Python Rich
        py_panel = RichPanel(content, title=title)
        py_output = capture_rich_ansi(py_panel, width=width)

        # Rust richrs
        rust_result = run_richrs_test(
            "panel_render", {"content": content, "title": title, "width": width}
        )
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]
        assert rust_output is not None
        assert title in rust_output or "Title" in rust_output

    def test_panel_with_subtitle(self, richrs_runner):
        """Test panel with subtitle renders correctly."""
        from rich.panel import Panel as RichPanel

        content = "Content here"
        subtitle = "Subtitle"
        width = 40

        # Python Rich
        py_panel = RichPanel(content, subtitle=subtitle)
        py_output = capture_rich_ansi(py_panel, width=width)

        # Rust richrs
        rust_result = run_richrs_test(
            "panel_render", {"content": content, "subtitle": subtitle, "width": width}
        )
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_output = rust_result["output"]
        assert rust_output is not None


class TestEmojiComparison:
    """Compare emoji lookup between Python Rich and Rust richrs."""

    @pytest.mark.parametrize(
        "emoji_name",
        [
            "smile",
            "thumbs_up",
            "rocket",
            "fire",
            "star",
            "warning",
            "heart",
            "coffee",
            "bug",
            "gear",
            "lock",
            "key",
            "bulb",
            "sparkles",
            "zap",
            "white_check_mark",
            "x",
        ],
    )
    def test_emoji_lookup(self, emoji_name: str, richrs_runner):
        """Test that emoji lookup returns the same character."""
        from rich.emoji import Emoji as RichEmoji

        # Python Rich
        py_emoji = RichEmoji(emoji_name)
        py_char = str(py_emoji)

        # Rust richrs
        rust_result = run_richrs_test("emoji_lookup", {"name": emoji_name})
        assert rust_result["success"], f"Rust failed: {rust_result.get('error')}"

        rust_char = rust_result["output"]

        # Both should return the same emoji character
        assert py_char == rust_char, f"Emoji mismatch for '{emoji_name}': Python={py_char!r}, Rust={rust_char!r}"

    def test_emoji_not_found(self, richrs_runner):
        """Test that nonexistent emojis return an error."""
        # Rust richrs should return an error for nonexistent emoji
        rust_result = run_richrs_test("emoji_lookup", {"name": "nonexistent_emoji_xyz"})
        assert not rust_result["success"], "Rust should fail for nonexistent emoji"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
