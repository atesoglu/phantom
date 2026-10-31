project('phantom', 'rust',
  version: '0.1.0',
  license: 'MIT',
  meson_version: '>=1.0')

cargo = find_program('cargo', required: true)

build = custom_target('cargo-build',
  command: [cargo, 'build', '--release', '--workspace'],
  output: ['phantom-ui'],
  console: true,
  build_by_default: true)

install_data('target/release/phantom-ui', install_dir: get_option('bindir'))