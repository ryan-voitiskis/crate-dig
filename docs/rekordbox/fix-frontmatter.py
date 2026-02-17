#!/usr/bin/env python3
"""Bulk-fix frontmatter topics and modes across the rekordbox corpus."""

import os
import re
import sys

CORPUS_DIR = os.path.dirname(os.path.abspath(__file__))

CONTROLLED_TOPICS = {
    "analysis", "backup", "beatgrid", "browsing", "cloud",
    "collaborative-playlists", "collection", "color", "comments",
    "compatibility", "connection", "cue-points", "devices", "dvs",
    "edit", "effects", "equipment", "export", "file-formats", "genre",
    "history", "hot-cue", "import", "interface", "key", "library",
    "lighting", "link", "memory-cue", "metadata", "midi", "mixing",
    "mobile", "onelibrary", "pads", "performance", "phrase", "playback",
    "playlists", "preferences", "pro-dj-link", "rating", "recording",
    "sampler", "search", "sequencer", "slicer", "stems", "streaming",
    "subscription", "system-requirements", "track-suggestion", "usb",
    "video", "waveform", "xml",
}

# Map ad-hoc topic -> list of controlled vocab replacements
TOPIC_MAP = {
    # Direct lowercase normalizations
    "hot cue": ["hot-cue"],
    "hot cues": ["hot-cue"],
    "Hot Cue": ["hot-cue"],
    "cue points": ["cue-points"],
    "cue-point": ["cue-points"],
    "memory cues": ["memory-cue"],
    "beat grid": ["beatgrid"],
    "beat-grid": ["beatgrid"],
    "pro-dj link": ["pro-dj-link"],
    "PRO DJ LINK": ["pro-dj-link"],
    "file formats": ["file-formats"],
    "file-format": ["file-formats"],
    "system requirements": ["system-requirements"],
    "track suggestion": ["track-suggestion"],
    "collaborative playlists": ["collaborative-playlists"],

    # Specific service names -> streaming
    "TIDAL": ["streaming"],
    "Apple Music": ["streaming"],
    "Spotify": ["streaming"],
    "SoundCloud": ["streaming"],
    "Beatport": ["streaming"],
    "Beatsource": ["streaming"],

    # Performance/playback topics
    "playing tracks": ["playback"],
    "tempo control": ["mixing"],
    "master tempo": ["key"],
    "analysis lock": ["analysis"],
    "quantize": ["beatgrid"],
    "loop play": ["playback"],
    "beat sync": ["mixing"],
    "matching": ["mixing"],
    "pitch bend": ["mixing"],
    "key sync": ["key"],
    "key shift": ["key"],
    "automatic playback": ["playback"],
    "slip mode": ["playback"],
    "instant doubles": ["playback"],
    "beat jump": ["playback"],
    "automix": ["playback"],

    # Interface elements
    "dual player": ["interface"],
    "EQ": ["mixing"],
    "crossfader": ["mixing"],
    "info window": ["interface"],
    "information": ["interface"],
    "notification": ["interface"],
    "sequencer panel": ["sequencer"],
    "mixer panel": ["mixing"],
    "recording panel": ["recording"],
    "enlarged waveform": ["waveform"],
    "browser panel": ["browsing"],

    # Equipment
    "DJ equipment": ["equipment"],
    "DJ controller": ["equipment"],
    "DJ controllers": ["equipment"],
    "DJ player": ["devices"],
    "DJ players": ["devices"],
    "compatible equipment": ["compatibility", "equipment"],
    "DVS hardware": ["dvs"],
    "exFAT": ["file-formats"],
    "waveform colors": ["waveform", "color"],
    "USB export": ["usb", "export"],
    "usb-export": ["usb", "export"],
    "link-export": ["export", "link"],

    # Connection
    "LAN": ["connection"],
    "Ableton Link": ["link"],
    "tag list": ["browsing"],
    "headphones": ["mixing"],
    "monitoring": ["mixing"],

    # Capture/sampler
    "capture": ["sampler"],
    "loop capture": ["sampler"],
    "slicer capture": ["slicer"],
    "sample scratch": ["sampler"],
    "performance pads": ["pads"],

    # STEMS sub-features
    "active stem": ["stems"],
    "stem iso": ["stems"],
    "stem fx": ["stems", "effects"],
    "groove circuit": ["effects"],
    "GROOVE CIRCUIT": ["effects"],
    "drum swap": ["effects"],
    "DRUM SWAP": ["effects"],
    "drum capture": ["effects"],
    "DRUM CAPTURE": ["effects"],
    "drum release fx": ["effects"],
    "vocal": ["stems"],
    "inst": ["stems"],
    "bass": ["stems"],
    "drums": ["stems"],

    # Recording
    "WAV": ["file-formats"],
    "social media": ["recording"],
    "Mixcloud": ["recording"],
    "YouTube": ["recording"],
    "share mix": ["recording"],
    "upload": ["recording"],

    # Subscription
    "account": ["subscription"],
    "accounts": ["subscription"],
    "trials": ["subscription"],
    "hardware unlock": ["subscription"],
    "owner registration": ["subscription"],
    "payments": ["subscription"],
    "coupons": ["subscription"],
    "activation": ["subscription"],

    # Settings/prefs
    "settings": ["preferences"],

    # Unmapped from first pass — manual files
    "preparation": ["export"],
    "cue": ["cue-points"],
    "loop": ["playback"],
    "hot-cue-bank-list": ["hot-cue"],
    "play-histories": ["history"],
    "usb-storage": ["usb"],
    "audio input": ["devices"],
    "silence detection": ["analysis"],
    "BEAT FX": ["effects"],
    "SOUND COLOR FX": ["effects"],
    "Release FX": ["effects"],
    "MERGE FX": ["effects"],
    "PAD FX": ["effects"],
    "multi-mode": ["effects"],
    "single-mode": ["effects"],
    "BPM": ["beatgrid"],
    "effect panel": ["effects"],
    "color knob": ["effects"],
    "sampler deck": ["sampler"],
    "sampler slots": ["sampler"],
    "sample audio": ["sampler"],
    "play mode": ["sampler"],
    "oneshot": ["sampler"],
    "gate mode": ["sampler"],
    "sync": ["mixing"],
    "edit mode": ["edit"],
    "slot gain": ["sampler"],
    "bpm sync": ["mixing"],
    "overdub recording": ["sequencer"],
    "sequence playback": ["sequencer"],
    "sequence mute": ["sequencer"],
    "sequence erase": ["sequencer"],
    "metronome": ["sequencer"],
    "sequence load": ["sequencer"],
    "sequence call": ["sequencer"],
    "slicer loop": ["slicer"],
    "sections": ["slicer"],
    "active censor": ["effects"],
    "censoring": ["effects"],
    "IN point": ["cue-points"],
    "OUT point": ["cue-points"],
    "plan": ["subscription"],
    "view": ["interface"],
    "audio": ["devices"],
    "controller": ["equipment"],
    "keyboard": ["interface"],
    "advanced": ["preferences"],
    "extensions": ["preferences"],
    "groove-circuit": ["effects"],
    "drum-swap": ["effects"],
    "drum-capture": ["effects"],
    "menus": ["interface"],
    "file-menu": ["interface"],
    "view-menu": ["interface"],
    "track-menu": ["interface"],
    "playlist-menu": ["interface"],
    "help-menu": ["interface"],

    # Hyphenated forms (from first pass best-effort)
    "audio-input": ["devices"],
    "silence-detection": ["analysis"],
    "beat-fx": ["effects"],
    "color-knob": ["effects"],
    "effect-panel": ["effects"],
    "merge-fx": ["effects"],
    "pad-fx": ["effects"],
    "release-fx": ["effects"],
    "sound-color-fx": ["effects"],
    "bpm-sync": ["mixing"],
    "edit-mode": ["edit"],
    "gate-mode": ["sampler"],
    "play-mode": ["sampler"],
    "sample-audio": ["sampler"],
    "sampler-deck": ["sampler"],
    "sampler-slots": ["sampler"],
    "slot-gain": ["sampler"],
    "overdub-recording": ["sequencer"],
    "sequence-call": ["sequencer"],
    "sequence-erase": ["sequencer"],
    "sequence-load": ["sequencer"],
    "sequence-mute": ["sequencer"],
    "sequence-playback": ["sequencer"],
    "slicer-loop": ["slicer"],
    "active-censor": ["effects"],
    "in-point": ["cue-points"],
    "out-point": ["cue-points"],
    "cloud-analysis": ["cloud", "analysis"],
    "cloud-export": ["cloud", "export"],
    "cloud-library-sync": ["cloud"],
    "cloud-storage": ["cloud"],
    "google-drive": ["cloud"],
    "cue-analysis": ["analysis"],
    "library-management": ["library"],
    "track-management": ["library"],
    "audio-processing": ["effects"],

    # FAQ unmapped
    "cloud library sync": ["cloud"],
    "Dropbox": ["cloud"],
    "Google Drive": ["cloud"],
    "cloud storage": ["cloud"],
    "CloudDirectPlay": ["cloud"],
    "cloud export": ["cloud", "export"],
    "cloud analysis": ["cloud", "analysis"],
    "library management": ["library"],
    "track management": ["library"],
    "importing": ["import"],
    "exporting": ["export"],
    "CUE analysis": ["analysis"],
    "audio processing": ["effects"],
    "troubleshooting": ["compatibility"],

    # Appendix
    "eula": ["system-requirements"],
    "communications": ["system-requirements"],
    "supported-media": ["file-formats"],
    "online-support": ["system-requirements"],
    "disclaimer": ["system-requirements"],
    "copyright": ["system-requirements"],
    "trademarks": ["system-requirements"],

    # Misc
    "preparing": ["export"],
    "workflow": ["export"],
    "mix point link": ["link", "mixing"],
    "mix in": ["mixing"],
    "mix out": ["mixing"],
    "silent": ["mixing"],
    "OneLibrary": ["onelibrary"],
}

# ID renames
ID_RENAMES = {
    # Fix duplicate: manual/01-introduction.md keeps "introduction", guides gets renamed
}

ALLOWED_MODES = {"common", "export", "performance", "lighting", "edit"}


def parse_frontmatter(content):
    """Extract frontmatter string and body from markdown content."""
    if not content.startswith("---\n"):
        return None, content
    end = content.find("\n---\n", 4)
    if end == -1:
        return None, content
    fm = content[4:end]
    body = content[end + 5:]
    return fm, body


def fix_topics_line(line):
    """Fix a topics: [...] line to use controlled vocabulary."""
    m = re.match(r'^(topics:\s*\[)(.*?)(\]\s*)$', line)
    if not m:
        return line
    prefix, topics_str, suffix = m.groups()
    raw_topics = [t.strip() for t in topics_str.split(',')]

    new_topics = set()
    for topic in raw_topics:
        if not topic:
            continue
        # Check if already controlled
        if topic.lower() in CONTROLLED_TOPICS:
            new_topics.add(topic.lower())
        elif topic in TOPIC_MAP:
            for replacement in TOPIC_MAP[topic]:
                new_topics.add(replacement)
        elif topic.lower() in {t.lower() for t in TOPIC_MAP}:
            # Case-insensitive lookup
            for key, replacements in TOPIC_MAP.items():
                if key.lower() == topic.lower():
                    for r in replacements:
                        new_topics.add(r)
                    break
        else:
            # Try lowercase
            if topic.lower().replace(' ', '-') in CONTROLLED_TOPICS:
                new_topics.add(topic.lower().replace(' ', '-'))
            else:
                print(f"  UNMAPPED topic: '{topic}'", file=sys.stderr)
                # Keep it lowercase with hyphens as best effort
                normalized = topic.lower().replace(' ', '-')
                new_topics.add(normalized)

    sorted_topics = sorted(new_topics)
    return f"{prefix}{', '.join(sorted_topics)}{suffix}"


def fix_modes_line(line):
    """Normalize mode casing to lowercase."""
    m = re.match(r'^(modes:\s*\[)(.*?)(\]\s*)$', line)
    if not m:
        return line
    prefix, modes_str, suffix = m.groups()
    raw_modes = [t.strip() for t in modes_str.split(',')]

    new_modes = []
    for mode in raw_modes:
        if not mode:
            continue
        normalized = mode.lower()
        if normalized in ALLOWED_MODES:
            new_modes.append(normalized)
        else:
            print(f"  UNMAPPED mode: '{mode}'", file=sys.stderr)
            new_modes.append(normalized)

    return f"{prefix}{', '.join(new_modes)}{suffix}"


def fix_id_line(line, filepath):
    """Fix duplicate IDs."""
    if "guides/introduction.md" in filepath:
        m = re.match(r'^(id:\s*)introduction\s*$', line)
        if m:
            return f"{m.group(1)}guide-introduction\n"
    return line


def process_file(filepath):
    """Process a single markdown file."""
    with open(filepath, 'r') as f:
        content = f.read()

    if not content.startswith("---\n"):
        return False

    lines = content.split('\n')
    in_fm = False
    changed = False
    new_lines = []

    for i, line in enumerate(lines):
        if i == 0 and line == '---':
            in_fm = True
            new_lines.append(line)
            continue
        if in_fm and line == '---':
            in_fm = False
            new_lines.append(line)
            continue

        if in_fm:
            original = line
            if line.startswith('topics:'):
                line = fix_topics_line(line)
            elif line.startswith('modes:'):
                line = fix_modes_line(line)
            elif line.startswith('id:'):
                line = fix_id_line(line, filepath)

            if line != original:
                changed = True

        new_lines.append(line)

    if changed:
        with open(filepath, 'w') as f:
            f.write('\n'.join(new_lines))
        return True
    return False


def main():
    dirs = ['manual', 'guides', 'faq', 'features', 'reference']
    total_fixed = 0
    total_files = 0

    for d in dirs:
        dirpath = os.path.join(CORPUS_DIR, d)
        if not os.path.isdir(dirpath):
            continue
        for fname in sorted(os.listdir(dirpath)):
            if not fname.endswith('.md'):
                continue
            filepath = os.path.join(dirpath, fname)
            total_files += 1
            print(f"Processing {d}/{fname}...", end=' ')
            if process_file(filepath):
                print("FIXED")
                total_fixed += 1
            else:
                print("ok")

    print(f"\nProcessed {total_files} files, fixed {total_fixed}")


if __name__ == '__main__':
    main()
