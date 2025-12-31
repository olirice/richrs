#!/usr/bin/env python3
"""Script to record terminal demos and convert to GIF/PNG."""

import subprocess
import os
import sys
import time
from pathlib import Path
import re
from PIL import Image, ImageDraw, ImageFont
from io import BytesIO
import xml.etree.ElementTree as ET


def ensure_dir(path):
    """Ensure directory exists."""
    Path(path).mkdir(parents=True, exist_ok=True)


def get_ansi_to_rgb(code):
    """Convert ANSI color code to RGB."""
    # Basic 16 colors
    basic_colors = {
        30: (0, 0, 0),        # black
        31: (205, 49, 49),    # red
        32: (13, 188, 121),   # green
        33: (229, 229, 16),   # yellow
        34: (36, 114, 200),   # blue
        35: (188, 63, 188),   # magenta
        36: (17, 168, 205),   # cyan
        37: (229, 229, 229),  # white
        90: (102, 102, 102),  # bright black
        91: (241, 76, 76),    # bright red
        92: (35, 209, 139),   # bright green
        93: (245, 245, 67),   # bright yellow
        94: (59, 142, 234),   # bright blue
        95: (214, 112, 214),  # bright magenta
        96: (41, 184, 219),   # bright cyan
        97: (255, 255, 255),  # bright white
    }
    return basic_colors.get(code, (229, 229, 229))


def parse_ansi_text(text):
    """Parse ANSI text and return list of (text, fg_color, bg_color, bold, italic, etc)."""
    # Pattern to match ANSI escape codes
    ansi_pattern = re.compile(r'\x1b\[([0-9;]*)m')

    segments = []
    current_fg = (229, 229, 229)  # default fg (white)
    current_bg = (30, 30, 46)     # default bg (dark)
    bold = False
    dim = False
    italic = False
    underline = False

    pos = 0
    for match in ansi_pattern.finditer(text):
        # Add text before the escape code
        if match.start() > pos:
            segment_text = text[pos:match.start()]
            if segment_text:
                fg = current_fg
                if dim:
                    fg = tuple(int(c * 0.6) for c in fg)
                segments.append((segment_text, fg, current_bg, bold, italic, underline))

        # Process escape code
        codes = match.group(1).split(';') if match.group(1) else ['0']
        i = 0
        while i < len(codes):
            try:
                code = int(codes[i])
            except ValueError:
                i += 1
                continue

            if code == 0:  # reset
                current_fg = (229, 229, 229)
                current_bg = (30, 30, 46)
                bold = False
                dim = False
                italic = False
                underline = False
            elif code == 1:
                bold = True
            elif code == 2:
                dim = True
            elif code == 3:
                italic = True
            elif code == 4:
                underline = True
            elif code == 7:  # reverse
                current_fg, current_bg = current_bg, current_fg
            elif 30 <= code <= 37 or 90 <= code <= 97:
                current_fg = get_ansi_to_rgb(code)
            elif code == 38:  # 256 color or RGB foreground
                if i + 1 < len(codes):
                    if codes[i + 1] == '5':  # 256 color
                        if i + 2 < len(codes):
                            color_idx = int(codes[i + 2])
                            current_fg = get_256_color(color_idx)
                            i += 2
                    elif codes[i + 1] == '2':  # RGB
                        if i + 4 < len(codes):
                            r, g, b = int(codes[i + 2]), int(codes[i + 3]), int(codes[i + 4])
                            current_fg = (r, g, b)
                            i += 4
            elif 40 <= code <= 47:
                current_bg = get_ansi_to_rgb(code - 10)
            elif code == 48:  # 256 color or RGB background
                if i + 1 < len(codes):
                    if codes[i + 1] == '5':  # 256 color
                        if i + 2 < len(codes):
                            color_idx = int(codes[i + 2])
                            current_bg = get_256_color(color_idx)
                            i += 2
                    elif codes[i + 1] == '2':  # RGB
                        if i + 4 < len(codes):
                            r, g, b = int(codes[i + 2]), int(codes[i + 3]), int(codes[i + 4])
                            current_bg = (r, g, b)
                            i += 4
            i += 1

        pos = match.end()

    # Add remaining text
    if pos < len(text):
        segment_text = text[pos:]
        if segment_text:
            fg = current_fg
            if dim:
                fg = tuple(int(c * 0.6) for c in fg)
            segments.append((segment_text, fg, current_bg, bold, italic, underline))

    return segments


def get_256_color(idx):
    """Get RGB for 256-color palette."""
    if idx < 16:
        # Standard colors
        colors = [
            (0, 0, 0), (205, 49, 49), (13, 188, 121), (229, 229, 16),
            (36, 114, 200), (188, 63, 188), (17, 168, 205), (229, 229, 229),
            (102, 102, 102), (241, 76, 76), (35, 209, 139), (245, 245, 67),
            (59, 142, 234), (214, 112, 214), (41, 184, 219), (255, 255, 255)
        ]
        return colors[idx]
    elif idx < 232:
        # 216 colors (6x6x6 cube)
        idx -= 16
        r = (idx // 36) * 51
        g = ((idx // 6) % 6) * 51
        b = (idx % 6) * 51
        return (r, g, b)
    else:
        # Grayscale
        gray = (idx - 232) * 10 + 8
        return (gray, gray, gray)


def render_text_to_image(text, width=700, font_size=14, padding=20):
    """Render ANSI text to a PIL Image."""
    # Clean up the text - remove cursor control codes
    text = re.sub(r'\x1b\[\?25[lh]', '', text)  # hide/show cursor
    text = re.sub(r'\x1b\[\d*[ABCD]', '', text)  # cursor movement
    text = re.sub(r'\x1b\[\d*K', '', text)       # erase line
    text = re.sub(r'\r', '', text)               # carriage return

    # Load a monospace font
    try:
        font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf", font_size)
        bold_font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf", font_size)
    except:
        font = ImageFont.load_default()
        bold_font = font

    # Calculate character dimensions
    char_width = font.getbbox("M")[2]
    char_height = int(font_size * 1.4)

    # Split into lines
    lines = text.split('\n')

    # Calculate image dimensions
    max_chars = max(len(re.sub(r'\x1b\[[0-9;]*m', '', line)) for line in lines) if lines else 0
    img_width = max(width, max_chars * char_width + padding * 2)
    img_height = len(lines) * char_height + padding * 2

    # Create image with dark background
    bg_color = (30, 30, 46)
    img = Image.new('RGB', (img_width, img_height), bg_color)
    draw = ImageDraw.Draw(img)

    # Draw each line
    y = padding
    for line in lines:
        segments = parse_ansi_text(line)
        x = padding
        for seg_text, fg, bg, bold, italic, underline in segments:
            # Skip escape sequences that weren't parsed
            seg_text = re.sub(r'\x1b\[[0-9;]*m', '', seg_text)
            if not seg_text:
                continue

            # Draw background if not default
            if bg != bg_color:
                text_width = len(seg_text) * char_width
                draw.rectangle([x, y, x + text_width, y + char_height], fill=bg)

            # Draw text
            f = bold_font if bold else font
            draw.text((x, y), seg_text, font=f, fill=fg)

            # Draw underline if needed
            if underline:
                text_width = len(seg_text) * char_width
                draw.line([x, y + char_height - 2, x + text_width, y + char_height - 2], fill=fg)

            x += len(seg_text) * char_width
        y += char_height

    return img


def capture_command_output(cmd, timeout=30):
    """Capture command output with ANSI codes."""
    env = os.environ.copy()
    env['TERM'] = 'xterm-256color'
    env['COLORTERM'] = 'truecolor'

    result = subprocess.run(
        cmd,
        shell=True,
        capture_output=True,
        text=True,
        timeout=timeout,
        env=env
    )

    # Combine stdout and stderr
    output = result.stderr + result.stdout
    return output


def record_static_png(cmd, output_path, width=700):
    """Record a static PNG from command output."""
    print(f"Recording: {cmd}")
    output = capture_command_output(cmd)
    img = render_text_to_image(output, width=width)
    img.save(output_path)
    print(f"Saved: {output_path}")


def record_animated_gif(cmd, output_path, duration=8, fps=12, width=700, font_size=14, padding=20):
    """Record an animated GIF by capturing frames."""
    import pty
    import select
    import time

    print(f"Recording animation: {cmd}")

    frames = []
    frame_times = []

    # Use termtosvg to record and capture frames
    svg_path = output_path.replace('.gif', '.svg')

    # Record with termtosvg
    record_cmd = f'termtosvg -c "{cmd}" -g 80x24 -D 100 {svg_path}'

    env = os.environ.copy()
    env['TERM'] = 'xterm-256color'

    result = subprocess.run(
        record_cmd,
        shell=True,
        capture_output=True,
        text=True,
        timeout=duration + 10,
        env=env
    )

    if os.path.exists(svg_path):
        # Convert SVG to GIF using ffmpeg or imagemagick
        # First try to extract frames from SVG
        try:
            convert_svg_to_gif(svg_path, output_path, fps=fps)
        except Exception as e:
            print(f"SVG conversion failed: {e}")
            # Fallback: just save a static PNG
            output = capture_command_output(cmd)
            img = render_text_to_image(output, width=width)
            img.save(output_path.replace('.gif', '.png'))
    else:
        print(f"Failed to record: {result.stderr}")


def convert_svg_to_gif(svg_path, gif_path, fps=12):
    """Convert SVG animation to GIF using cairosvg and pillow."""
    import cairosvg

    # Read SVG content
    with open(svg_path, 'r') as f:
        svg_content = f.read()

    # Parse SVG to extract animation frames
    # termtosvg creates animated SVGs with <text> elements that change over time
    # We'll render multiple frames by modifying the SVG

    tree = ET.parse(svg_path)
    root = tree.getroot()

    # Get SVG dimensions
    width = int(float(root.get('width', '800').replace('px', '')))
    height = int(float(root.get('height', '600').replace('px', '')))

    # For simplicity, render the full SVG as a single frame
    # (Full animation extraction is complex)
    png_data = cairosvg.svg2png(url=svg_path, output_width=width, output_height=height)
    img = Image.open(BytesIO(png_data))
    img.save(gif_path.replace('.gif', '.png'))

    print(f"Saved static image (SVG animation extraction not fully supported): {gif_path.replace('.gif', '.png')}")


def main():
    """Main function to record all demos."""
    base_dir = Path(__file__).parent.parent
    assets_dir = base_dir / 'assets'
    ensure_dir(assets_dir)

    # Build all examples first
    print("Building examples...")
    subprocess.run(
        ['cargo', 'build', '--release', '--features', 'full', '--examples'],
        cwd=base_dir,
        check=True
    )

    # Static demos (PNG)
    static_demos = [
        ('styling', 'styling.png'),
        ('panels', 'panels.png'),
        ('tables', 'tables.png'),
        ('trees', 'trees.png'),
        ('syntax', 'syntax.png'),
        ('markdown', 'markdown.png'),
    ]

    # Animated demos (GIF)
    animated_demos = [
        ('progress', 'progress.gif'),
        ('spinners', 'spinners.gif'),
        ('status', 'status.gif'),
        ('live', 'live.gif'),
    ]

    # Record static demos
    for example, output_file in static_demos:
        cmd = f'cargo run --release --features full --example {example}'
        output_path = str(assets_dir / output_file)
        record_static_png(cmd, output_path, width=700)

    # Record animated demos using termtosvg
    for example, output_file in animated_demos:
        cmd = f'./target/release/examples/{example}'
        output_path = str(assets_dir / output_file)
        record_animated_gif(cmd, output_path, duration=10, fps=12)

    print("\nAll demos recorded!")


if __name__ == '__main__':
    main()
