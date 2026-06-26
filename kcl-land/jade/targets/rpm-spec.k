import ..package

# 子包定义转子包定义部分字符串
outputsToPackageSection = lambda os: {str:package.Output} -> str {
    # 单个子包定义转单个子包定义部分字符串
    outputContentToPackageSectionString = lambda name: str, o: package.Output -> str {
        if o.buildRequires is not Undefined:
            _buildRequires = "\n".join(map b in o.buildRequires {
                "BuildRequires:  ${b}"
            })
        else:
            _buildRequires = ""
        if o.runtimeRequires is not Undefined:
            _runtimeRequires = "\n".join(map r in o.runtimeRequires {
                "Requires:       ${r}"
            })
        else:
            _runtimeRequires = ""
        """
%package        ${name}
Summary:        ${o.summary}
""".lstrip() + _buildRequires + _runtimeRequires + """
%description    ${name}
${o.description}
""".rstrip()
    }

    "\n".join(map name, output in os {
        outputContentToPackageSectionString(name, output)
    })
}

# 子包定义转子包 file 部分字符串
outputsToFilesSection = lambda os: {str:package.Output} -> str {
    # 单个子包定义转单个子包 file 部分字符串
    outputContentToFilesSectionString = lambda name: str, o: package.Output -> str {
        if o.files is not Undefined:
            _files = "%files          ${name}\n" + "\n".join(o.files)
        else:
            _files = ""
        _files
    }

    "\n".join(map name, output in os {
        outputContentToFilesSectionString(name, output)
    })
}

packageToRpmSpec = lambda p: package.Package -> str {
    if p.buildSystem is not Undefined:
        _buildSystem = "BuildSystem:    " + p.buildSystem

    if p.sources is not Undefined:
        _sources = "\n".join(["Source${i}:        ${v.url}" for i, v in p.sources])

    if p.outputs.main.buildRequires is not Undefined:
        _buildRequires = "\n".join(map b in p.outputs.main.buildRequires {
            "BuildRequires:  ${b}"
        })

    if p.outputs.main.runtimeRequires is not Undefined:
        _runtimeRequires = "\n".join(map r in p.outputs.main.runtimeRequires {
            "Requires:       ${r}"
        })

    _packageSection = outputsToPackageSection(p.outputs)
    _filesExtra = outputsToFilesSection(p.outputs)

    """
Name:           ${p.name}
Version:        ${p.version}
Summary:        ${p.outputs.main.summary}
Release:        %autorelease
License:        ${p.license}
URL:            ${p.url}
VCS:            ${p?.extra.openRuyi.vcs}
${_sources}
${_buildSystem}

# [TODO] BuildOption
> BuildOption(conf):  --disable-static
> BuildOption(conf):  --disable-dependency-tracking

${_buildRequires}

%description
${p.outputs.main.description}

${_packageSection}

%conf -p
./autogen.sh

%install -a
install -m0644 README %{buildroot}%{_docdir}/%{name}

find %{buildroot}%{_datadir}/%{name}/demo -type f -exec chmod a-x {} ';'
mv %{buildroot}%{_datadir}/%{name}/demo %{buildroot}%{_docdir}/%{name}/
find %{buildroot}%{_docdir}/%{name}/demo -type f -name "*.py" -exec mv {} {}.demo ';'

rm -f %{buildroot}%{_bindir}/dot_builtins
rm -rf %{buildroot}%{_datadir}/graphviz/graphs

mkdir -p %{buildroot}%{_libdir}/graphviz
touch %{buildroot}%{_libdir}/graphviz/config6

mkdir -p %{buildroot}%{_datadir}/%{name}/examples

%transfiletriggerin -- %{_libdir}/graphviz
%{_bindir}/dot -c 2>/dev/null || :

%transfiletriggerpostun -- %{_libdir}/graphviz
%{_bindir}/dot -c 2>/dev/null || :

%files
%license COPYING
%doc %{_docdir}/graphviz
%{_bindir}/*
%dir %{_libdir}/graphviz
%{_libdir}/graphviz/*.so.*
%{_prefix}/lib/perl5/vendor_perl/*
%{_mandir}/man1/*.1*
%{_mandir}/man7/*.7*
%dir %{_datadir}/graphviz
%exclude %{_docdir}/graphviz/demo
%{_datadir}/graphviz/gvpr
%{_datadir}/graphviz/examples
%ghost %{_libdir}/graphviz/config6
%doc %{_docdir}/graphviz/demo

%exclude %{_libdir}/graphviz/*/*
%exclude %{_libdir}/graphviz/libgvplugin_gd.*

${_filesExtra}

%changelog
%{?autochangelog}
""".strip()
}
