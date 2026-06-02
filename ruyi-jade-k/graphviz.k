import jade.package
# import jade.targets

package.Package {
    name: "graphviz"
    version: "14.0.4"
    release: "1"
    vcs: "git"
    license: "EPL-1.0 AND CPL-1.0 AND BSD-3-Clause AND MIT AND GPL-3.0-or-later"
    url: "http://www.graphviz.org/"
    buildSystem: "autotools"
    sources: {
        tar: package.Source {
            type: "Remote"
            url: "https://gitlab.com/graphviz/graphviz/-/archive/${version}/graphviz-${version}.tar.gz"
            checksum: "fake-none"
        }
        fake: package.Source {
            type: "Remote"
            url: "fake://i.am.fake/"
            checksum: "fake-none"
        }
    }
    outputs: {
        main: {
            summary: "Graph Visualization Tools"
            description: "<ommited>"
            buildRequires: [
                "autoconf"
                "pkgconfig(zlib)"
                "pkgconfig(cairo) >= 1.1.10"
            ]
            files: ["a", "b"]
        }
        devel: {
            summary: "Development package for graphviz"
            description: "This package contains development files for graphviz."
            files: ["fake 1", "fake 2"]
        }
        libs: {
            summary: "Graphviz libraries"
            description: "Graphviz libraries."
        }
    }
}

# graphvizSpec = targets.packageToRpmSpec(graphviz)
