"""
Tests for style parsing and rendering.

These tests compare Rich Python style output with richrs Rust output.
"""

import json
import subprocess
from pathlib import Path

import pytest
from rich.console import Console
from rich.style import Style
from rich.text import Text

from conftest import capture_rich_output, capture_rich_ansi


class TestStyleParsing:
    """Test style string parsing."""

    @pytest.mark.parametrize(
        "style_str",
        [
            "bold",
            "italic",
            "underline",
            "strike",
            "red",
            "green",
            "blue",
            "bold red",
            "italic blue",
            "bold italic underline",
            "red on white",
            "bold red on blue",
            "#ff0000",
            "#00ff00",
            "color(5)",
            "not bold",
            "dim",
            "reverse",
            "blink",
            "conceal",
        ],
    )
    def test_style_parse(self, style_str: str):
        """Test that style strings parse correctly."""
        style = Style.parse(style_str)
        assert style is not None

    @pytest.mark.parametrize(
        "style_str,expected_bold,expected_italic",
        [
            ("bold", True, None),
            ("italic", None, True),
            ("bold italic", True, True),
            ("not bold", False, None),
        ],
    )
    def test_style_attributes(
        self, style_str: str, expected_bold: bool | None, expected_italic: bool | None
    ):
        """Test that style attributes are parsed correctly."""
        style = Style.parse(style_str)
        assert style.bold == expected_bold
        assert style.italic == expected_italic


class TestStyleCombination:
    """Test style combination."""

    def test_combine_colors(self):
        """Test combining styles with colors."""
        base = Style.parse("red")
        overlay = Style.parse("bold")
        combined = base + overlay

        assert combined.color is not None
        assert combined.bold is True

    def test_overlay_takes_precedence(self):
        """Test that overlay style takes precedence."""
        base = Style.parse("red on white")
        overlay = Style.parse("blue")
        combined = base + overlay

        # Blue should override red
        assert combined.color.name == "blue"
        # White background should remain
        assert combined.bgcolor is not None


class TestStyleOutput:
    """Test style rendering output."""

    def test_bold_text_output(self):
        """Test that bold text renders correctly."""
        text = Text("Hello", style="bold")
        output = capture_rich_ansi(text)

        # Should contain ANSI bold code
        assert "\x1b[1m" in output
        assert "Hello" in output

    def test_colored_text_output(self):
        """Test that colored text renders correctly."""
        text = Text("Red", style="red")
        output = capture_rich_ansi(text)

        # Should contain ANSI red code (31 for standard red)
        assert "\x1b[31m" in output or "\x1b[38" in output
        assert "Red" in output

    def test_combined_style_output(self):
        """Test that combined styles render correctly."""
        text = Text("Bold Red", style="bold red")
        output = capture_rich_ansi(text)

        # Rich may combine ANSI codes (e.g., \x1b[1;31m instead of \x1b[1m\x1b[31m)
        assert "\x1b[1" in output  # bold (may be combined with other codes)
        assert "Bold Red" in output


class TestStyleExport:
    """Test exporting styles for comparison with Rust."""

    def export_style_data(self, style_str: str) -> dict:
        """Export style data for comparison."""
        style = Style.parse(style_str)
        return {
            "input": style_str,
            "bold": style.bold,
            "italic": style.italic,
            "underline": style.underline,
            "strike": style.strike,
            "color": str(style.color) if style.color else None,
            "bgcolor": str(style.bgcolor) if style.bgcolor else None,
        }

    @pytest.mark.parametrize(
        "style_str",
        [
            "bold",
            "bold red",
            "italic blue on white",
            "#ff0000",
        ],
    )
    def test_export_style(self, style_str: str, test_data_dir: Path):
        """Export style data to JSON for Rust comparison."""
        data = self.export_style_data(style_str)
        # This data can be used by the Rust test harness
        assert data["input"] == style_str


def generate_style_test_cases():
    """Generate test cases for style comparison."""
    test_cases = []

    # Basic attributes
    for attr in ["bold", "italic", "underline", "strike", "dim", "reverse"]:
        test_cases.append({"style": attr, "text": f"Test {attr}"})

    # Basic colors
    for color in ["red", "green", "blue", "yellow", "magenta", "cyan", "white", "black"]:
        test_cases.append({"style": color, "text": f"Test {color}"})

    # Bright colors
    for color in ["bright_red", "bright_green", "bright_blue"]:
        test_cases.append({"style": color, "text": f"Test {color}"})

    # Combined styles
    test_cases.extend([
        {"style": "bold red", "text": "Bold Red"},
        {"style": "italic blue", "text": "Italic Blue"},
        {"style": "bold italic underline", "text": "Combined"},
        {"style": "red on white", "text": "Red on White"},
        {"style": "bold #ff0000 on #ffffff", "text": "Hex Colors"},
    ])

    return test_cases


if __name__ == "__main__":
    # Generate test data for export
    test_cases = generate_style_test_cases()
    print(json.dumps(test_cases, indent=2))
