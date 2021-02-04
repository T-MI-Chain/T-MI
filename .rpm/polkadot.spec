%define debug_package %{nil}

Name: tmi
Summary: Implementation of a https://tmi.network node in Rust based on the Substrate framework.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

Requires: systemd, shadow-utils
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}


%prep
%setup -q


%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%post
config_file="/etc/default/tmi"
getent group tmi >/dev/null || groupadd -r tmi
getent passwd tmi >/dev/null || \
    useradd -r -g tmi -d /home/tmi -m -s /sbin/nologin \
    -c "User account for running tmi as a service" tmi
if [ ! -e "$config_file" ]; then
    echo 'tmi_CLI_ARGS=""' > /etc/default/tmi
fi
exit 0

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/usr/lib/systemd/system/tmi.service
