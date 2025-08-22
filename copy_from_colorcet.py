import colorcet
import re


def parse_cmaps() -> list[str]:
    pat = re.compile(r"(\s|\[)(\d)(,\s|])")
    s = []
    for name in colorcet.all_original_names():
        cmap = getattr(colorcet, name)
        if isinstance(cmap, list) and len(cmap) == 256 and isinstance(cmap[0], list):
            c = f"{cmap}"
            c = pat.sub(r"\1\2.0\3", c)
            c = pat.sub(r"\1\2.0\3", c)
            c = pat.sub(r"\1\2.0\3", c)
            s.append(f"    \"{name}\" => {c},\n")
    return sorted(s)


def parse_aliases() -> list[str]:
    pat = re.compile(r",\s+")
    s = []
    for name in colorcet.all_original_names():
        for alias in pat.split(colorcet.get_aliases(name)):
            s.append(f"    \"{alias}\" => \"{name}\",\n")
    return sorted(s)


def main() -> None:
    """extract colormaps from the python package colorcet, run cargo fmt after"""
    cmaps = parse_cmaps()
    aliases = parse_aliases()

    with open("src/colormaps.rs", "w") as f:
        f.write("use phf::phf_map;\n")
        f.write("\n")
        f.write("/// unique colormaps\n")
        f.write("#[allow(clippy::approx_constant)]\n")
        f.write("pub static COLOR_MAPS: phf::Map<&'static str, [[f64; 3]; 256]> = phf_map! {\n")
        for i in cmaps:
            f.write(i)
        f.write("};\n")
        f.write("\n")
        f.write("/// aliases to colormaps\n")
        f.write("#[allow(clippy::approx_constant)]\n")
        f.write("pub static ALIASES: phf::Map<&'static str, &'static str> = phf_map! {\n")
        for i in aliases:
            f.write(i)
        f.write("};")


if __name__ == "__main__":
    main()