import json
from pathlib import Path

TRAIT_PATH = Path(__file__).resolve().parent.parent / "traits" / "traits.json"


def load_traits():
    return json.loads(TRAIT_PATH.read_text())


def render_svg(traits: list[int]) -> str:
    layers = load_traits()
    palette = {
        "fur": ["#8b5a3c", "#2b2b2b", "#5b4ac7", "#f4f0e6"],
        "eyes": ["#f0b540", "#ff3b30", "#8fd1ff"],
        "accessories": ["#000000", "#7c3aed", "#f472b6"],
        "expression": ["#ffffff", "#d1d5db", "#a78bfa"],
        "background": ["#b6f5d0", "#ffb580", "#7dd3fc"],
    }
    svg = [
        '<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256">',
        f'<rect width="256" height="256" fill="{palette["background"][traits[4]]}" />',
        '<g>',
        f'<rect x="64" y="64" width="128" height="128" rx="24" fill="{palette["fur"][traits[0]]}" />',
        f'<circle cx="110" cy="112" r="12" fill="{palette["eyes"][traits[1]]}" />',
        f'<circle cx="146" cy="112" r="12" fill="{palette["eyes"][traits[1]]}" />',
        f'<rect x="98" y="136" width="60" height="10" rx="5" fill="{palette["expression"][traits[3]]}" />',
        f'<circle cx="128" cy="84" r="16" fill="{palette["accessories"][traits[2]]}" />',
        '</g>',
        '</svg>',
    ]
    return "\n".join(svg)


if __name__ == "__main__":
    print(render_svg([0, 1, 2, 2, 1]))
