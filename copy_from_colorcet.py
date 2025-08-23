import colorcet
import re
from matplotlib import colormaps


def parse_cmaps() -> list[str]:
    s = []
    for name in colorcet.all_original_names():
        cmap = getattr(colorcet, name)
        if isinstance(cmap, list) and isinstance(cmap[0], list):
            c = f"&{[[float(i) for i in j] for j in cmap]}"
            s.append(f"    \"{name.lower()}\" => {c},\n")
    return sorted(s)


def parse_aliases() -> list[str]:
    pat = re.compile(r",\s+")
    s = []
    for name in colorcet.all_original_names():
        for alias in pat.split(colorcet.get_aliases(name)):
            s.append(f"    \"{alias.lower()}\" => \"{name.lower()}\",\n")
    return s


def parse_mpl_cmaps() -> list[str]:
    s = []
    for name in list(colormaps):
        if not name.lower().startswith("cet") and not name.lower().endswith("_r"):
            cmap = colormaps[name]
            cmap = cmap(range(cmap.N))[:, :3].tolist()
            c = f"&{[[float(i) for i in j] for j in cmap]}"
            s.append(f"    \"{name.lower()}\" => {c},\n")
    return s


def main() -> None:
    """extract colormaps from the python package colorcet, run cargo fmt after"""
    cmaps = parse_cmaps() + parse_mpl_cmaps()
    aliases = parse_aliases()

    with open("src/colormaps.rs", "w") as f:
        f.write("use phf::phf_map;\n")
        f.write("\n")
        f.write("/// unique colormaps\n")
        f.write("#[allow(clippy::approx_constant)]\n")
        f.write("pub static COLOR_MAPS: phf::Map<&'static str, &[[f64; 3]]> = phf_map! {\n")
        for i in sorted(cmaps):
            f.write(i)
        f.write("};\n")
        f.write("\n")
        f.write("/// aliases to colormaps\n")
        f.write("#[allow(clippy::approx_constant)]\n")
        f.write("pub static ALIASES: phf::Map<&'static str, &'static str> = phf_map! {\n")
        for i in sorted(aliases):
            f.write(i)
        f.write("};")


if __name__ == "__main__":
    main()