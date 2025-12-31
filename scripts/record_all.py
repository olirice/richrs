#!/usr/bin/env python3
"""Record all demos as PNG/GIF for README."""

import subprocess
import os
import re
import sys
import time
import pty
import select
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont


def get_256_color(idx):
    """Get RGB for 256-color palette index."""
    if idx < 16:
        colors = [
            (0, 0, 0), (205, 49, 49), (13, 188, 121), (229, 229, 16),
            (36, 114, 200), (188, 63, 188), (17, 168, 205), (229, 229, 229),
            (102, 102, 102), (241, 76, 76), (35, 209, 139), (245, 245, 67),
            (59, 142, 234), (214, 112, 214), (41, 184, 219), (255, 255, 255)
        ]
        return colors[idx]
    elif idx < 232:
        idx -= 16
        r = (idx // 36) * 51
        g = ((idx // 6) % 6) * 51
        b = (idx % 6) * 51
        return (r, g, b)
    else:
        gray = (idx - 232) * 10 + 8
        return (gray, gray, gray)


def parse_ansi_text(text):
    """Parse ANSI escape sequences and return styled segments."""
    ansi_pattern = re.compile(r'\x1b\[([0-9;]*)m')
    segments = []
    current_fg = (229, 229, 229)
    current_bg = (30, 30, 46)
    bold = dim = italic = underline = False

    pos = 0
    for match in ansi_pattern.finditer(text):
        if match.start() > pos:
            segment_text = text[pos:match.start()]
            if segment_text:
                fg = tuple(int(c * 0.6) for c in current_fg) if dim else current_fg
                segments.append((segment_text, fg, current_bg, bold, italic, underline))

        codes = match.group(1).split(';') if match.group(1) else ['0']
        i = 0
        while i < len(codes):
            try:
                code = int(codes[i])
            except ValueError:
                i += 1
                continue

            if code == 0:
                current_fg = (229, 229, 229)
                current_bg = (30, 30, 46)
                bold = dim = italic = underline = False
            elif code == 1: bold = True
            elif code == 2: dim = True
            elif code == 3: italic = True
            elif code == 4: underline = True
            elif code == 7: current_fg, current_bg = current_bg, current_fg
            elif 30 <= code <= 37:
                basic = {30:(0,0,0),31:(205,49,49),32:(13,188,121),33:(229,229,16),
                        34:(36,114,200),35:(188,63,188),36:(17,168,205),37:(229,229,229)}
                current_fg = basic.get(code, (229,229,229))
            elif 90 <= code <= 97:
                bright = {90:(102,102,102),91:(241,76,76),92:(35,209,139),93:(245,245,67),
                         94:(59,142,234),95:(214,112,214),96:(41,184,219),97:(255,255,255)}
                current_fg = bright.get(code, (255,255,255))
            elif code == 38:
                if i + 1 < len(codes) and codes[i + 1] == '2' and i + 4 < len(codes):
                    current_fg = (int(codes[i+2]), int(codes[i+3]), int(codes[i+4]))
                    i += 4
                elif i + 1 < len(codes) and codes[i + 1] == '5' and i + 2 < len(codes):
                    current_fg = get_256_color(int(codes[i+2]))
                    i += 2
            elif code == 48:
                if i + 1 < len(codes) and codes[i + 1] == '2' and i + 4 < len(codes):
                    current_bg = (int(codes[i+2]), int(codes[i+3]), int(codes[i+4]))
                    i += 4
            i += 1
        pos = match.end()

    if pos < len(text):
        segment_text = text[pos:]
        if segment_text:
            fg = tuple(int(c * 0.6) for c in current_fg) if dim else current_fg
            segments.append((segment_text, fg, current_bg, bold, italic, underline))
    return segments


def process_carriage_returns(text):
    """Process carriage returns to simulate terminal behavior."""
    lines = text.split('\n')
    result = []
    for line in lines:
        if '\r' in line:
            # Process carriage returns - later content overwrites earlier
            parts = line.split('\r')
            # Start with empty line, each part overwrites from beginning
            current = ''
            for part in parts:
                if part:
                    # Overwrite from beginning, keeping rest if current is longer
                    if len(part) >= len(current):
                        current = part
                    else:
                        current = part + current[len(part):]
            result.append(current)
        else:
            result.append(line)
    return '\n'.join(result)


def render_to_image(text, width=600, font_size=14, padding=20, bottom_padding=None):
    """Render ANSI text to a PIL Image."""
    # Process carriage returns first
    text = process_carriage_returns(text)

    # Clean control sequences
    text = re.sub(r'\x1b\[\?25[lh]', '', text)  # hide/show cursor
    text = re.sub(r'\x1b\[\d*[ABCDJK]', '', text)  # cursor movement & erase
    text = re.sub(r'\x1b\[2J', '', text)  # clear screen
    text = re.sub(r'\x1b\[H', '', text)  # cursor home
    text = re.sub(r'\x1b\[\d+;\d+H', '', text)  # cursor position
    text = re.sub(r'\x1b\[\d*[su]', '', text)  # save/restore cursor

    if bottom_padding is None:
        bottom_padding = padding

    try:
        font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf', font_size)
        bold_font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf', font_size)
    except:
        font = ImageFont.load_default()
        bold_font = font

    char_width = font.getbbox('M')[2]
    char_height = int(font_size * 1.5)
    lines = text.split('\n')

    # Remove empty trailing lines
    while lines and not lines[-1].strip():
        lines.pop()

    max_chars = max(len(re.sub(r'\x1b\[[0-9;]*m', '', line)) for line in lines) if lines else 0
    img_width = max(width, max_chars * char_width + padding * 2)
    img_height = len(lines) * char_height + padding + bottom_padding

    bg_color = (30, 30, 46)
    img = Image.new('RGB', (img_width, img_height), bg_color)
    draw = ImageDraw.Draw(img)

    y = padding
    for line in lines:
        segments = parse_ansi_text(line)
        x = padding
        for seg_text, fg, bg, bold, italic, underline in segments:
            seg_text = re.sub(r'\x1b\[[0-9;]*m', '', seg_text)
            if not seg_text: continue
            if bg != bg_color:
                text_width = len(seg_text) * char_width
                draw.rectangle([x, y, x + text_width, y + char_height], fill=bg)
            f = bold_font if bold else font
            draw.text((x, y), seg_text, font=f, fill=fg)
            if underline:
                text_width = len(seg_text) * char_width
                draw.line([x, y + char_height - 2, x + text_width, y + char_height - 2], fill=fg)
            x += len(seg_text) * char_width
        y += char_height
    return img


def capture_frames_from_pty(cmd, duration=8, fps=12):
    """Capture frames from a command running in a PTY."""
    frames = []
    frame_interval = 1.0 / fps

    # Create pseudo-terminal
    master_fd, slave_fd = pty.openpty()

    env = os.environ.copy()
    env['TERM'] = 'xterm-256color'
    env['COLORTERM'] = 'truecolor'

    # Start process
    proc = subprocess.Popen(
        cmd,
        shell=True,
        stdin=slave_fd,
        stdout=slave_fd,
        stderr=slave_fd,
        env=env,
        close_fds=True
    )

    os.close(slave_fd)

    output_buffer = ""
    start_time = time.time()
    last_frame_time = start_time

    try:
        while time.time() - start_time < duration:
            # Check for output
            readable, _, _ = select.select([master_fd], [], [], 0.05)

            if readable:
                try:
                    data = os.read(master_fd, 4096)
                    if data:
                        output_buffer += data.decode('utf-8', errors='replace')
                except OSError:
                    break

            # Capture frame at interval
            current_time = time.time()
            if current_time - last_frame_time >= frame_interval:
                if output_buffer.strip():
                    frames.append(output_buffer)
                last_frame_time = current_time

            # Check if process ended
            if proc.poll() is not None:
                # Read remaining output
                try:
                    while True:
                        readable, _, _ = select.select([master_fd], [], [], 0.1)
                        if not readable:
                            break
                        data = os.read(master_fd, 4096)
                        if not data:
                            break
                        output_buffer += data.decode('utf-8', errors='replace')
                except OSError:
                    pass
                # Add final frame
                if output_buffer.strip():
                    frames.append(output_buffer)
                break
    finally:
        os.close(master_fd)
        if proc.poll() is None:
            proc.terminate()
            proc.wait()

    return frames


def record_static(example_name, output_path, width=600):
    """Record a static PNG from example output."""
    print(f"Recording static: {example_name}")

    cmd = f'./target/release/examples/{example_name}'

    env = os.environ.copy()
    env['TERM'] = 'xterm-256color'
    env['COLORTERM'] = 'truecolor'

    result = subprocess.run(
        cmd,
        shell=True,
        capture_output=True,
        text=True,
        env=env,
        timeout=30
    )

    output = result.stderr + result.stdout
    img = render_to_image(output, width=width)
    img.save(output_path)
    print(f"  Saved: {output_path}")


def record_animated(example_name, output_path, duration=8, fps=10, width=600, fixed_height=None, cmd=None):
    """Record an animated GIF from example output."""
    print(f"Recording animated: {example_name}")

    if cmd is None:
        cmd = f'./target/release/examples/{example_name}'
    frames = capture_frames_from_pty(cmd, duration=duration, fps=fps)

    if not frames:
        print(f"  Warning: No frames captured for {example_name}")
        return

    # Convert frames to images
    images = []
    for frame_text in frames:
        img = render_to_image(frame_text, width=width)
        images.append(img)

    if not images:
        print(f"  Warning: No images generated for {example_name}")
        return

    # Use fixed height or compute a reasonable height (median of non-trivial frames)
    heights = sorted([img.height for img in images])
    if fixed_height:
        target_height = fixed_height
    else:
        # Use median height, but cap at 300px for reasonable GIF size
        target_height = min(heights[len(heights) // 2], 300)

    max_width = max(img.width for img in images)

    bg_color = (30, 30, 46)
    normalized = []
    for img in images:
        # Crop or pad to target dimensions
        new_img = Image.new('RGB', (max_width, target_height), bg_color)
        # Paste at top-left, cropping if image is taller
        paste_height = min(img.height, target_height)
        new_img.paste(img.crop((0, 0, img.width, paste_height)), (0, 0))
        normalized.append(new_img)

    # Remove duplicate consecutive frames (but keep enough for animation)
    deduped = [normalized[0]]
    for img in normalized[1:]:
        # Only dedupe if truly identical - keep animation frames
        if list(img.getdata()) != list(deduped[-1].getdata()):
            deduped.append(img)

    # For animations, ensure we have enough frames
    if len(deduped) < 5 and len(normalized) >= 5:
        # Keep more frames if deduplication was too aggressive
        deduped = normalized[::max(1, len(normalized) // 20)]  # Sample ~20 frames

    # Save as GIF
    frame_duration = int(1000 / fps)
    deduped[0].save(
        output_path,
        save_all=True,
        append_images=deduped[1:],
        duration=frame_duration,
        loop=0
    )
    print(f"  Saved: {output_path} ({len(deduped)} frames)")


def main():
    base_dir = Path(__file__).parent.parent
    assets_dir = base_dir / 'assets'
    assets_dir.mkdir(exist_ok=True)

    os.chdir(base_dir)

    # Build all examples first
    print("Building examples...")
    subprocess.run(
        ['cargo', 'build', '--release', '--features', 'full', '--examples'],
        check=True,
        capture_output=True
    )
    print("Build complete.\n")

    # Static demos (PNG)
    static_demos = [
        ('styling', 'styling.png', 650),
        ('panels', 'panels.png', 550),
        ('tables', 'tables.png', 500),
        ('trees', 'trees.png', 400),
        ('syntax', 'syntax.png', 600),
        ('markdown', 'markdown.png', 650),
    ]

    # Animated demos (GIF) - (name, output, duration, fps, width, height)
    animated_demos = [
        ('progress', 'progress.gif', 6, 10, 600, 160),
        ('spinners', 'spinners.gif', 6, 12, 550, 250),
        ('status', 'status.gif', 8, 10, 550, 220),
        ('live', 'live.gif', 8, 10, 450, 220),
    ]

    # Record static demos
    print("Recording static demos...")
    for example, output_file, width in static_demos:
        output_path = str(assets_dir / output_file)
        record_static(example, output_path, width=width)

    print("\nRecording animated demos...")
    for example, output_file, duration, fps, width, height in animated_demos:
        output_path = str(assets_dir / output_file)
        record_animated(example, output_path, duration=duration, fps=fps, width=width, fixed_height=height)

    # Record main demo (it's a binary, not an example)
    print("\nRecording main demo...")
    record_animated('main demo', str(assets_dir / 'demo.gif'), duration=30, fps=8, width=800, fixed_height=600,
                   cmd='./target/release/richrs')

    print("\nAll demos recorded!")


if __name__ == '__main__':
    main()
