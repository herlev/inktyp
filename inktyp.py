#!/usr/bin/env python
# coding=utf-8

import inkex
from inkex.units import convert_unit
import subprocess

# import sys
# inkex.utils.errormsg(inkex.__file__)
# inkex.utils.errormsg(sys.executable)
# inkex.utils.errormsg(sys.path)

# Extracts the equation from the svg description starting with "typst: "
def extract_equation(svg):
    if svg.tag_name != "g":
        return None
    for c in svg:
        if c.tag_name == "desc":
            desc = c.text
            prefix = "typst: "
            if desc.startswith(prefix):
                return desc.removeprefix(prefix)
    return None


# Prepares the svg for insertion in inkscape by scaling, changing position and setting random IDs
def prepare_svg(svg_string, scale, transform, svg_ids):
    svg: inkex.SvgDocumentElement = inkex.load_svg(svg_string).getroot()
    # change outer tag from svg to g as the object is not moveable in inkscape otherwise
    svg.tag = '{http://www.w3.org/2000/svg}g'
    svg.attrib.clear()
    # center svg in viewport
    if transform is not None:
        svg.transform = transform
    scale = convert_unit("1pt", "px") / scale
    for child in svg:
        if isinstance(child, inkex.ShapeElement):
            child.transform = inkex.Transform(scale=scale) @ child.transform
        if isinstance(child, inkex.Defs):
            for def_child in child:
                def_child.set_random_ids(
                    backlinks=True, blacklist=svg_ids
                )
    return svg


# Generates a new svg from a typst eqution
class InktypGenerate(inkex.GenerateExtension):
    def svg(self, svg_string):
        self.svg_string = svg_string
        return self

    def generate(self):
        return prepare_svg(self.svg_string, self.svg.scale, self.container_transform(), self.svg.get_ids())


class Inktyp(inkex.EffectExtension):
    def effect(self):
        num_selected = len(self.svg.selection)
        if num_selected == 0:
            p = subprocess.run(["inktyp", "new"], stdout=subprocess.PIPE)
            if p.returncode == 0:
                InktypGenerate().svg(p.stdout).run()
            return
        if num_selected != 1:
            return
        selection = self.svg.selection[0]
        equation = extract_equation(selection)
        if equation is None:
            return
        p = subprocess.run(["inktyp", "edit", f"{equation}"], stdout=subprocess.PIPE)
        if p.returncode != 0:
            return
        svg = prepare_svg(p.stdout, self.svg.scale, None, self.svg.get_ids())
        # Replace child elements of selection, such that
        # the selection keeps its scale and position
        selection.remove_all()
        for child in svg:
            selection.append(child)


if __name__ == '__main__':
    Inktyp().run()
