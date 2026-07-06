Name:           {{ name }}
Version:        {{ version }}
Release:        {{ release }}
Summary:        {{ output_main.summary }}
License:        {{ license }}
URL:            {{ url }}
VCS:            {{ vcs }}
{% for source in sources -%}
Source{{ loop.index }}:        {{ source.url }}
{% endfor -%}
BuildSystem:    {{ build_system }}

{% for build_require in output_main.build_requires -%}
BuildRequires:  {{ build_require }}
{% endfor %}
{% for require in output_main.requires -%}
BuildRequires:  {{ require }}
{% endfor -%}

%description
{{ output_main.description }}

{%~ for output in output_others %}
%package        {{ output.name }}
Summary:        {{ output.summary }}
{% for build_require in output.build_requires -%}
BuildRequires:  {{ build_require }}
{% endfor %}
{% for require in output.requires -%}
BuildRequires:  {{ require }}
{% endfor -%}

%description    {{ output.name }}
{{ output.description }}
{% endfor %}

{%~ for build_step in build_steps -%}
%build -a
{{ build_step.environment }}
{{ build_step.script }}
{%- endfor -%}

%files
{{ output_main.files }}

{%~ for output in output_others %}
%files {{ output.name }}
{{ output.files }}
{% endfor ~%}

%changelog
{%- if let Some(changelog) = changelog %}
{{ changelog }}
{% else %}
%{?autochangelog}
{% endif %}
