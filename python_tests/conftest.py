"""
Pytest configuration for richrs Python comparison tests.

This module sets up the testing infrastructure for comparing
Rich Python output with richrs Rust output.
"""

import json
import os
import subprocess
import sys
from io import StringIO
from pathlib import Path
from typing import Any

import pytest
from rich.console import Console
from rich.style import Style
from rich.text import Text
from rich.panel import Panel
from rich.table import Table
from rich.tree import Tree


# Path to the richrs test binary
RICHRS_BIN = Path(__file__).parent.parent / "target" / "debug" / "richrs_test_runner"


class RichrsTestRunner:
    """Test runner that communicates with the Rust binary."""

    def __init__(self):
        """Initialize the test runner."""
        self.binary_path = RICHRS_BIN
        self._process = None

    def ensure_binary(self) -> bool:
        """Ensure the binary exists, building if necessary."""
        if not self.binary_path.exists():
            # Try to build the binary
            result = subprocess.run(
                ["cargo", "build", "--bin", "richrs_test_runner"],
                cwd=self.binary_path.parent.parent.parent,
                capture_output=True,
                text=True,
            )
            return result.returncode == 0
        return True

    def run_test(self, test_name: str, test_data: dict[str, Any]) -> dict[str, Any]:
        """
        Run a richrs test and return the result.

        Args:
            test_name: Name of the test to run
            test_data: Input data for the test

        Returns:
            Dictionary containing the test output
        """
        if not self.ensure_binary():
            return {
                "success": False,
                "error": "Failed to build richrs_test_runner binary",
            }

        input_json = json.dumps({"test": test_name, "data": test_data})

        try:
            result = subprocess.run(
                [str(self.binary_path)],
                input=input_json + "\n",
                capture_output=True,
                text=True,
                timeout=30,
            )

            if result.returncode != 0:
                return {
                    "success": False,
                    "error": f"Binary failed: {result.stderr}",
                }

            output = result.stdout.strip()
            if output:
                return json.loads(output)
            return {"success": False, "error": "No output from binary"}

        except subprocess.TimeoutExpired:
            return {"success": False, "error": "Test timed out"}
        except json.JSONDecodeError as e:
            return {"success": False, "error": f"Invalid JSON response: {e}"}
        except Exception as e:
            return {"success": False, "error": str(e)}


# Global test runner instance
_test_runner = RichrsTestRunner()


def run_richrs_test(test_name: str, test_data: dict[str, Any]) -> dict[str, Any]:
    """
    Run a richrs test and return the result.

    Args:
        test_name: Name of the test to run
        test_data: Input data for the test

    Returns:
        Dictionary containing the test output
    """
    return _test_runner.run_test(test_name, test_data)


def capture_rich_output(renderable: Any, width: int = 80) -> str:
    """
    Capture Rich output to a string.

    Args:
        renderable: Any Rich renderable object
        width: Console width for rendering

    Returns:
        The rendered output as a string
    """
    console = Console(
        file=None,
        width=width,
        force_terminal=True,
        color_system="truecolor",
        record=True,
    )
    console.print(renderable, end="")
    return console.export_text()


def capture_rich_ansi(renderable: Any, width: int = 80) -> str:
    """
    Capture Rich ANSI output to a string.

    Args:
        renderable: Any Rich renderable object
        width: Console width for rendering

    Returns:
        The rendered output with ANSI codes as a string
    """
    output = StringIO()
    console = Console(
        file=output,
        width=width,
        force_terminal=True,
        color_system="truecolor",
    )
    console.print(renderable, end="")
    return output.getvalue()


def normalize_ansi(s: str) -> str:
    """
    Normalize ANSI escape sequences for comparison.

    This helps with minor differences in escape code ordering.
    """
    # Strip trailing whitespace and newlines
    return s.rstrip()


def compare_ansi_output(python_output: str, rust_output: str) -> bool:
    """
    Compare Python Rich output with Rust richrs output.

    Returns True if they match (allowing for minor differences).
    """
    # First try exact match
    if normalize_ansi(python_output) == normalize_ansi(rust_output):
        return True

    # TODO: Add more sophisticated comparison that handles equivalent
    # but differently-ordered ANSI codes
    return False


@pytest.fixture
def rich_console():
    """Fixture providing a Rich console for testing."""
    return Console(
        width=80,
        force_terminal=True,
        color_system="truecolor",
    )


@pytest.fixture
def test_data_dir():
    """Fixture providing the test data directory path."""
    return Path(__file__).parent / "test_data"


@pytest.fixture
def richrs_runner():
    """Fixture providing the Rust test runner."""
    return _test_runner
