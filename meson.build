project('phantom', 'rust',
  version: '0.1.0',
  license: 'MIT',
  meson_version: '>=1.0')

cargo = find_program('cargo', required: true)

# Build Rust workspace
build = custom_target('cargo-build',
  command: [cargo, 'build', '--release', '--workspace'],
  output: ['phantom-daemon', 'phantom-ui'],
  console: true,
  build_by_default: true)

# Install binaries
install_data('target/release/phantom-daemon', install_dir: get_option('bindir'))
install_data('target/release/phantom-ui', install_dir: get_option('bindir'))

# systemd user units
install_data('data/systemd/phantom.service',
  install_dir: join_paths(get_option('prefix'), 'lib', 'systemd', 'user'))

# desktop entry
install_data('data/phantom.desktop',
  install_dir: join_paths(get_option('prefix'), 'share', 'applications'))