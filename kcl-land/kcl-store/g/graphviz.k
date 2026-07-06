import jade.package

package.Package {
    name: "graphviz"
    version: "14.1.5"
    release: "%autorelease"
    vcs: "git"
    license: "EPL-1.0 AND CPL-1.0 AND BSD-3-Clause AND MIT AND GPL-3.0-or-later"
    url: "http://www.graphviz.org/"
    buildSystem: "autotools"
    sources: {
        a: package.Source {
            type: "Remote"
            url: "https://gitlab.com/graphviz/graphviz/-/archive/${version}/graphviz-${version}.tar.gz"
            checksum: "2cb3136bffb335346e9c5c99a7c615df32295839a6498df44f576adfc2937a93"
        }
    }
    outputs: {
        main: {
            summary: "Graph Visualization Tools"
            description: "A collection of tools for the manipulation and layout of graphs (as in nodes and edges, not as in barcharts)."
            buildRequires: [
                "autoconf"
                "pkgconfig(zlib)"
                "pkgconfig(cairo) >= 1.1.10"
                "后略"
            ]
            files: [
                "%license COPYING"
                "%doc README.md"
                "%doc %{_docdir}/graphviz"
                "%{_bindir}/*"
                "%dir %{_libdir}/graphviz"
                "%{_libdir}/graphviz/*.so.*"
                "%{_prefix}/lib/perl5/vendor_perl/*"
                "%{_mandir}/man1/*.1*"
                "%{_mandir}/man7/*.7*"
                "%dir %{_datadir}/graphviz"
                "%exclude %{_docdir}/graphviz/demo"
                "%{_datadir}/graphviz/gvpr"
                "%{_datadir}/graphviz/examples"
                "%ghost %{_libdir}/graphviz/config6"
                "%doc %{_docdir}/graphviz/demo"

                "%exclude %{_libdir}/graphviz/*/*"
                "%exclude %{_libdir}/graphviz/libgvplugin_gd.*"
            ]
        }
        devel: {
            summary: "Development package for graphviz"
            description: "This package contains development files for graphviz."
            runtimeRequires: [
                "${name}-libs%{?_isa} = ${version}-%{release}"
                "pkgconfig"
            ]
            files: [
                "%{_includedir}/graphviz"
                "%{_libdir}/*.so"
                "%{_libdir}/graphviz/*.so"
                "%{_libdir}/pkgconfig/libcdt.pc"
                "%{_libdir}/pkgconfig/libcgraph.pc"
                "%{_libdir}/pkgconfig/libgvc.pc"
                "%{_libdir}/pkgconfig/libgvpr.pc"
                "%{_libdir}/pkgconfig/libpathplan.pc"
                "%{_libdir}/pkgconfig/libxdot.pc"
                "%{_mandir}/man3/*.3*"
            ]
        }
        libs: {
            summary: "Graphviz libraries"
            description: "Graphviz libraries."
            files: [
                "%license COPYING"
                "%{_libdir}/libcdt.so.*"
                "%{_libdir}/libcgraph.so.*"
                "%{_libdir}/libgvc.so.*"
                "%{_libdir}/libgvpr.so.*"
                "%{_libdir}/libpathplan.so.*"
                "%{_libdir}/libxdot.so.*"
                "%{_libdir}/graphviz/libgvplugin_gd.so.*"
            ]
        }
        python3: {
            summary: "Python 3 extension for graphviz"
            description: "Python 3 extension for graphviz."
            runtimeRequires: [
                "${name}-libs%{?_isa} = ${version}-%{release}"
            ]
            files: [
                "%{python3_sitearch}/*"
                "%{_mandir}/man3/gv.3python*"
            ]
        }
    }
}
